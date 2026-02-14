use crate::value::StringType;

use super::{error::DeserializerError, literal::Literal, position::Position, token::Token};

pub struct Lexer {
  pointer: usize,
  data: String,
  position: Position,
}

impl Lexer {
  pub fn new(data: String) -> Self {
    Self { pointer: 0, data, position: Position::new(1, 0) }
  }

  pub fn get(&mut self) -> Result<Token, DeserializerError> {
    if let Some(data) = self.advance(true) {
      match data {
        '"' => self.process_string_with_double_quote(),
        '\'' => self.process_string_with_single_quote(),
        ';' => Ok(self.create_token(Literal::Semicolon)),
        ',' => Ok(self.create_token(Literal::Comma)),
        '[' => Ok(self.create_token(Literal::OpenBracket)),
        ']' => Ok(self.create_token(Literal::CloseBracket)),
        '{' => Ok(self.create_token(Literal::OpenBrace)),
        '}' => Ok(self.create_token(Literal::CloseBrace)),
        '(' => Ok(self.create_token(Literal::OpenParen)),
        ')' => Ok(self.create_token(Literal::CloseParen)),
        '&' => self.process_reference(),
        '*' => self.process_dereference(),
        '-' => self.process_signed_number(),
        't' => self.process_specific_sets(0),
        'f' => self.process_specific_sets(1),
        'v' => self.process_specific_sets(2),
        '/' => {
          if self.peek(1) == '/' {
            self.process_comment();
            Ok(self.create_token(Literal::Comment))
          } else {
            self.process_raw_string()
          }
        },
        other => {
          if self.is_ascii_from_chars(other) {
            self.process_raw_string()
          } else if self.is_digit_from_chars(other) {
            self.process_number()
          } else {
            Err(DeserializerError::UnexpectedLiteral(other, self.position))
          }
        },
      }
    } else {
      Ok(self.create_token(Literal::End))
    }
  }

  fn process_reference(&mut self) -> Result<Token, DeserializerError> {
    let start_pos: usize = self.pointer;
    while let Some(data) = self.advance(false) {
      if self.is_ascii_from_chars(data) {
        continue;
      } else if data == '_' || data == '-' {
        continue;
      } else if data == ' ' || data == '\t' || data == ',' || data == ';' || data == ']' {
        self.pointer -= 1;
        self.position.subtract_column_by(1);
        return Ok(
          self.create_token(Literal::Reference(self.data[start_pos..self.pointer].to_string())),
        );
      } else {
        return Err(DeserializerError::UnexpectedLiteral(data, self.position));
      }
    }
    Err(DeserializerError::UnexpectedTermination(self.position))
  }

  fn process_dereference(&mut self) -> Result<Token, DeserializerError> {
    let start_pos: usize = self.pointer;
    while let Some(data) = self.advance(false) {
      if self.is_ascii_from_chars(data) {
        continue;
      } else if data == '_' || data == '-' {
        continue;
      } else if data == ' ' || data == '\t' || data == ',' || data == ';' || data == ']' {
        self.pointer -= 1;
        self.position.subtract_column_by(1);
        return Ok(
          self.create_token(Literal::Dereference(self.data[start_pos..self.pointer].to_string())),
        );
      } else {
        return Err(DeserializerError::UnexpectedLiteral(data, self.position));
      }
    }
    Err(DeserializerError::UnexpectedTermination(self.position))
  }

  fn process_string_with_double_quote(&mut self) -> Result<Token, DeserializerError> {
    let mut start_pos: usize = self.pointer;
    let mut buffer = Vec::<u8>::new();
    while let Some(data) = self.advance(false) {
      if data == '"' {
        buffer.extend_from_slice(self.data[start_pos..self.pointer - 1].as_bytes());
        let next_char = self.peek(1);
        if next_char != ','
          && next_char != ';'
          && next_char != ' '
          && next_char != '\t'
          && next_char != '\r'
          && next_char != '\n'
          && next_char != ']'
        {
          return Err(DeserializerError::UnexpectedLiteral(
            next_char,
            Position::from((self.position.get_line(), self.position.get_column() + 1)),
          ));
        }
        return Ok(self.create_token(Literal::String(
          String::from_utf8_lossy(&buffer).to_string(),
          StringType::DoubleQuoted,
        )));
      } else if data == '\\' {
        match self.peek(1) {
          'n' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\n".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          'r' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\r".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          't' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\t".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\\' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\\".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '"' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\"".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\'' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("'".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\0' => {
            return Err(DeserializerError::UnexpectedTermination(self.position));
          },
          other => {
            return Err(DeserializerError::UnexpectedLiteral(other, self.position));
          },
        }
      } else if data == '\n' || data == '\r' {
        return Err(DeserializerError::InvalidNewLine(self.position));
      } else {
        continue;
      }
    }
    Err(DeserializerError::UnexpectedTermination(self.position))
  }

  fn process_string_with_single_quote(&mut self) -> Result<Token, DeserializerError> {
    let mut start_pos: usize = self.pointer;
    let mut buffer = Vec::<u8>::new();
    while let Some(data) = self.advance(false) {
      if data == '\'' {
        buffer.extend_from_slice(self.data[start_pos..self.pointer - 1].as_bytes());
        let next_char = self.peek(1);
        if next_char != ','
          && next_char != ';'
          && next_char != ' '
          && next_char != '\t'
          && next_char != '\r'
          && next_char != '\n'
          && next_char != ']'
        {
          return Err(DeserializerError::UnexpectedLiteral(
            next_char,
            Position::from((self.position.get_line(), self.position.get_column() + 1)),
          ));
        }
        return Ok(self.create_token(Literal::String(
          String::from_utf8_lossy(&buffer).to_string(),
          StringType::SingleQuoted,
        )));
      } else if data == '\\' {
        match self.peek(1) {
          'n' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\n".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          'r' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\r".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          't' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\t".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\\' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\\".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '"' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\"".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\'' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("'".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\0' => {
            return Err(DeserializerError::UnexpectedTermination(self.position));
          },
          other => {
            return Err(DeserializerError::UnexpectedLiteral(other, self.position));
          },
        }
      } else if data == '\n' || data == '\r' {
        return Err(DeserializerError::InvalidNewLine(self.position));
      } else {
        continue;
      }
    }
    Err(DeserializerError::UnexpectedTermination(self.position))
  }

  fn process_raw_string(&mut self) -> Result<Token, DeserializerError> {
    let start_pos: usize = self.pointer - 1;
    while let Some(data) = self.advance(false) {
      match data {
        '/' => {
          if '/' == self.peek(1) {
            self.pointer -= 1;
            self.position.subtract_column_by(1);
            return Ok(self.create_token(Literal::String(
              self.data[start_pos..self.pointer].to_string(),
              StringType::Raw,
            )));
          } else {
            continue;
          }
        },
        ' ' => {
          self.pointer -= 1;
          self.position.subtract_column_by(1);
          return Ok(self.create_token(Literal::String(
            self.data[start_pos..self.pointer].to_string(),
            StringType::Raw,
          )));
        },
        '\t' => {
          self.pointer -= 1;
          self.position.subtract_column_by(1);
          return Ok(self.create_token(Literal::String(
            self.data[start_pos..self.pointer].to_string(),
            StringType::Raw,
          )));
        },
        '\n' => {
          self.pointer -= 1;
          self.position.subtract_column_by(1);
          return Ok(self.create_token(Literal::String(
            self.data[start_pos..self.pointer].to_string(),
            StringType::Raw,
          )));
        },
        '\r' => {
          self.pointer -= 1;
          self.position.subtract_column_by(1);
          return Ok(self.create_token(Literal::String(
            self.data[start_pos..self.pointer].to_string(),
            StringType::Raw,
          )));
        },
        ',' => {
          self.pointer -= 1;
          self.position.subtract_column_by(1);
          return Ok(self.create_token(Literal::String(
            self.data[start_pos..self.pointer].to_string(),
            StringType::Raw,
          )));
        },
        ';' => {
          self.pointer -= 1;
          self.position.subtract_column_by(1);
          return Ok(self.create_token(Literal::String(
            self.data[start_pos..self.pointer].to_string(),
            StringType::Raw,
          )));
        },
        ']' => {
          self.pointer -= 1;
          self.position.subtract_column_by(1);
          return Ok(self.create_token(Literal::String(
            self.data[start_pos..self.pointer].to_string(),
            StringType::Raw,
          )));
        },
        _ => {
          continue;
        },
      }
    }
    Ok(self.create_token(Literal::String(
      self.data[start_pos..self.pointer].to_string(),
      StringType::Raw,
    )))
  }

  fn process_number(&mut self) -> Result<Token, DeserializerError> {
    let start_pos: usize = self.pointer - 1;
    while let Some(data) = self.advance(false) {
      if self.is_digit_from_chars(data) {
        continue;
      } else if data == '.' {
        return self.process_float_number(start_pos);
      } else if data == ' '
        || data == '\t'
        || data == '\r'
        || data == '\n'
        || data == ']'
        || data == ','
        || data == ';'
      {
        self.pointer -= 1;
        self.position.subtract_column_by(1);
        return Ok(self.create_token(Literal::UnsignedIntegerNumber(
          (self.data[start_pos..self.pointer]).parse::<u32>().unwrap(),
        )));
      } else if data == '/' {
        if '/' == self.peek(1) {
          self.pointer -= 1;
          self.position.subtract_column_by(1);
          return Ok(self.create_token(Literal::UnsignedIntegerNumber(
            self.data[start_pos..self.pointer].parse::<u32>().unwrap(),
          )));
        } else {
          continue;
        }
      } else {
        self.move_pointer_to(start_pos + 1);
        return self.process_raw_string();
      }
    }
    Ok(self.create_token(Literal::UnsignedIntegerNumber(
      self.data[start_pos..self.pointer].parse::<u32>().unwrap(),
    )))
  }

  fn process_signed_number(&mut self) -> Result<Token, DeserializerError> {
    let start_pos: usize = self.pointer - 1;
    while let Some(data) = self.advance(false) {
      if self.is_digit_from_chars(data) {
        continue;
      } else if data == '.' {
        return self.process_float_number(start_pos);
      } else if data == ' '
        || data == '\t'
        || data == '\r'
        || data == '\n'
        || data == ']'
        || data == ','
        || data == ';'
      {
        self.pointer -= 1;
        self.position.subtract_column_by(1);
        return Ok(self.create_token(Literal::SignedIntegerNumber(
          (self.data[start_pos..self.pointer]).parse::<i32>().unwrap(),
        )));
      } else if data == '/' {
        if '/' == self.peek(1) {
          self.pointer -= 1;
          self.position.subtract_column_by(1);
          return Ok(self.create_token(Literal::SignedIntegerNumber(
            self.data[start_pos..self.pointer].parse::<i32>().unwrap(),
          )));
        } else {
          continue;
        }
      } else {
        self.move_pointer_to(start_pos + 1);
        return self.process_raw_string();
      }
    }
    Ok(self.create_token(Literal::SignedIntegerNumber(
      self.data[start_pos..self.pointer].parse::<i32>().unwrap(),
    )))
  }

  fn process_float_number(&mut self, start_pos: usize) -> Result<Token, DeserializerError> {
    while let Some(data) = self.advance(false) {
      if self.is_digit_from_chars(data) {
        continue;
      } else if data == ' '
        || data == '\t'
        || data == '\r'
        || data == '\n'
        || data == ']'
        || data == ','
        || data == ';'
      {
        self.pointer -= 1;
        self.position.subtract_column_by(1);
        return Ok(self.create_token(Literal::FloatNumber(
          (self.data[start_pos..self.pointer]).parse::<f32>().unwrap(),
        )));
      } else if data == '/' {
        if '/' == self.peek(1) {
          self.pointer -= 1;
          self.position.subtract_column_by(1);
          return Ok(self.create_token(Literal::FloatNumber(
            self.data[start_pos..self.pointer].parse::<f32>().unwrap(),
          )));
        } else {
          continue;
        }
      } else {
        self.move_pointer_to(start_pos + 1);
        return self.process_raw_string();
      }
    }
    Ok(self.create_token(Literal::FloatNumber(
      self.data[start_pos..self.pointer].parse::<f32>().unwrap(),
    )))
  }

  fn process_specific_sets(&mut self, sets: u8) -> Result<Token, DeserializerError> {
    /*
      Sets:
       - 0: true
       - 1: false
       - 2: void
       This function will match one by one.
       Only switch to string parsing function if it meets other characters.
    */
    let start_pos: usize = self.pointer;
    match sets {
      0 => {
        const SET: [char; 3] = ['r', 'u', 'e'];
        for c in SET {
          if let Some(data) = self.advance(false) {
            if data == c {
              continue;
            } else {
              self.move_pointer_to(start_pos);
              return self.process_raw_string();
            }
          } else {
            return Err(DeserializerError::UnexpectedTermination(self.position));
          }
        }
        let peeked = self.peek(1);
        if peeked == ','
          || peeked == ';'
          || peeked == ' '
          || peeked == '\t'
          || peeked == ']'
          || peeked == '\r'
          || peeked == '\n'
          || peeked == '\0'
        {
          Ok(self.create_token(Literal::Boolean(true)))
        } else if peeked == '/' && self.peek(2) == '/' {
          Ok(self.create_token(Literal::Boolean(true)))
        } else {
          self.move_pointer_to(start_pos);
          self.process_raw_string()
        }
      },
      1 => {
        const SET: [char; 4] = ['a', 'l', 's', 'e'];
        for c in SET {
          if let Some(data) = self.advance(false) {
            if data == c {
              continue;
            } else {
              self.move_pointer_to(start_pos);
              return self.process_raw_string();
            }
          } else {
            return Err(DeserializerError::UnexpectedTermination(self.position));
          }
        }
        let peeked = self.peek(1);
        if peeked == ','
          || peeked == ';'
          || peeked == ' '
          || peeked == '\t'
          || peeked == ']'
          || peeked == '\r'
          || peeked == '\n'
          || peeked == '\0'
        {
          Ok(self.create_token(Literal::Boolean(false)))
        } else if peeked == '/' && self.peek(2) == '/' {
          Ok(self.create_token(Literal::Boolean(false)))
        } else {
          self.move_pointer_to(start_pos);
          self.process_raw_string()
        }
      },
      2 => {
        const SET: [char; 3] = ['o', 'i', 'd'];
        for c in SET {
          if let Some(data) = self.advance(false) {
            if data == c {
              continue;
            } else {
              self.move_pointer_to(start_pos);
              return self.process_raw_string();
            }
          } else {
            return Err(DeserializerError::UnexpectedTermination(self.position));
          }
        }
        let peeked = self.peek(1);
        if peeked == ','
          || peeked == ';'
          || peeked == ' '
          || peeked == '\t'
          || peeked == ']'
          || peeked == '\r'
          || peeked == '\n'
          || peeked == '\0'
        {
          Ok(self.create_token(Literal::Void))
        } else if peeked == '/' && self.peek(2) == '/' {
          Ok(self.create_token(Literal::Void))
        } else {
          self.move_pointer_to(start_pos);
          self.process_raw_string()
        }
      },
      _ => Err(DeserializerError::NoSetsFound(self.position)),
    }
  }

  fn process_comment(&mut self) {
    while let Some(data) = self.advance(false) {
      if data == '\n' || data == '\r' {
        self.pointer -= 1;
        self.position.subtract_column_by(1);
        return;
      }
    }
  }

  #[inline]
  fn peek(&mut self, count: usize) -> char {
    /*
      It behaves like `advance` method, but do not move pointer
      and return exact data even if it meets end.
    */
    if let Some(data) = self.data.chars().nth(self.pointer + (count - 1)) { data } else { '\0' }
  }

  #[inline]
  fn advance(&mut self, skip_whitespace_and_newline: bool) -> Option<char> {
    /*
      It starts at 0.
      When called, it will read current data to which pointer points and move the pointer.

      It will add column by 1.
    */
    if !skip_whitespace_and_newline {
      let data = self.data.chars().nth(self.pointer);
      match data {
        Some(c) => {
          self.move_pointer_by(1);
          self.position.add_column_by(1);
          Some(c)
        },
        None => None,
      }
    } else {
      loop {
        let data = self.data.chars().nth(self.pointer);
        if let Some(cache) = data {
          self.move_pointer_by(1);
          self.position.add_column_by(1);
          if cache == ' ' || cache == '\t' {
            continue;
          } else if cache == '\n' {
            self.position.add_line_by(1);
            self.position.set_column_to(0);
          } else if cache == '\r' {
            if self.peek(1) == '\n' {
              self.move_pointer_by(1);
            }
            self.position.add_line_by(1);
            self.position.set_column_to(0);
          } else {
            return Some(cache);
          }
        } else {
          return None;
        }
      }
    }
  }

  #[inline(always)]
  fn create_token(&self, literal: Literal) -> Token {
    Token::new(literal, self.position)
  }

  #[inline(always)]
  fn move_pointer_by(&mut self, count: usize) {
    self.pointer += count;
  }

  #[inline(always)]
  fn move_pointer_to(&mut self, pos: usize) {
    self.pointer = pos;
  }

  #[inline(always)]
  fn is_digit_from_chars(&self, data: char) -> bool {
    data >= '0' && data <= '9'
  }

  #[inline(always)]
  fn is_ascii_from_chars(&self, data: char) -> bool {
    data >= 'a' && data <= 'z'
  }
}
