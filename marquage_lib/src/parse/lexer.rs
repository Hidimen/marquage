use crate::parse::{
  error::LexerError, literal::Literal, position::Position,
  source_map::SourceMap, span::Span, token::Token,
};

pub struct Lexer<'lex> {
  map: SourceMap<'lex>,
  pos: Position,
}

impl<'lex> Lexer<'lex> {
  pub fn new<'raw>(raw: &'raw str) -> Self
  where
    'raw: 'lex,
  {
    Self { map: SourceMap::new(raw), pos: Position(1, 1) }
  }

  pub fn lex(&mut self) -> Result<Token<'_>, LexerError<'_>> {
    if let Some((s, offset)) = self.skipping_advance() {
      let legacy_pos = self.pos.add_column_by(1);
      let current_pos = self.pos;
      match s {
        "{" => {Ok(Self::create_token(
          Literal::OpenBrace,
          legacy_pos,
          current_pos,
          (offset, offset + 1),
        ))},
        "}" => Ok(Self::create_token(
          Literal::CloseBrace,
          legacy_pos,
          current_pos,
          (offset, offset + 1),
        )),
        "[" => Ok(Self::create_token(
          Literal::OpenBracket,
          legacy_pos,
          current_pos,
          (offset, offset + 1),
        )),
        "]" => Ok(Self::create_token(
          Literal::CloseBracket,
          legacy_pos,
          current_pos,
          (offset, offset + 1),
        )),
        "(" => Ok(Self::create_token(
          Literal::OpenParen,
          legacy_pos,
          current_pos,
          (offset, offset + 1),
        )),
        ")" => Ok(Self::create_token(
          Literal::CloseParen,
          legacy_pos,
          current_pos,
          (offset, offset + 1),
        )),
        ";" => Ok(Self::create_token(
          Literal::Semicolon,
          legacy_pos,
          current_pos,
          (offset, offset + 1),
        )),
        "," => Ok(Self::create_token(
          Literal::Comma,
          legacy_pos,
          current_pos,
          (offset, offset + 1),
        )),
        "@" => Ok(Self::create_token(
          Literal::At,
          legacy_pos,
          current_pos,
          (offset, offset + 1),
        )),
        "=" => Ok(Self::create_token(
          Literal::Equal,
          legacy_pos,
          current_pos,
          (offset, offset + 1),
        )),
        "-" => self.handle_number(offset, legacy_pos, true),
        "#" => Ok(self.handle_comment(offset, legacy_pos)),
        "\"" => self.handle_quoted_string(offset, legacy_pos),
        other if self.is_digital(other) => {
          self.handle_number(offset, legacy_pos, false)
        },
        other if other == "v" => self.handle_void(offset, legacy_pos),
        other if other == "t" => self.handle_true(offset, legacy_pos),
        other if other == "f" => self.handle_false(offset, legacy_pos),
        _ => self.handle_raw_string(offset, legacy_pos),
      }
    } else {
      Ok(Self::create_token(Literal::End, self.pos, self.pos, (0, 0)))
    }
  }

  fn handle_void(
    &mut self, start_offset: usize, start: Position,
  ) -> Result<Token<'_>, LexerError<'_>> {
    let remains = ["o", "i", "d"];
    for i in remains {
      if let Some((s, _)) = self.advance()
        && s == i
      {
        self.pos.add_column();
        continue;
      } else {
        self.map.move_to(start_offset);
        self.pos = start;
        return self.handle_raw_string(start_offset, start);
      }
    }

    if let Some(p) = self.peek()
      && (p == " "
        || p == "\r"
        || p == "\n"
        || p == "#"
        || p == ";"
        || p == ","
        || p == "]"
        || p == "}"
        || p == ")")
    {
      Ok(Self::create_token(
        Literal::Void,
        start,
        self.pos,
        (start_offset, self.current_offset()),
      ))
    } else {
      self.map.move_to(start_offset);
      self.pos = start;
      self.handle_raw_string(start_offset, start)
    }
  }

  fn handle_true(
    &mut self, start_offset: usize, start: Position,
  ) -> Result<Token<'_>, LexerError<'_>> {
    let remains = ["r", "u", "e"];
    for i in remains {
      if let Some((s, _)) = self.advance()
        && s == i
      {
        self.pos.add_column();
        continue;
      } else {
        self.map.move_to(start_offset);
        self.pos = start;
        return self.handle_raw_string(start_offset, start);
      }
    }

    if let Some(p) = self.peek()
      && (p == " "
        || p == "\r"
        || p == "\n"
        || p == "#"
        || p == ";"
        || p == ","
        || p == "]"
        || p == "}"
        || p == ")")
    {
      Ok(Self::create_token(
        Literal::Boolean(true),
        start,
        self.pos,
        (start_offset, self.current_offset()),
      ))
    } else {
      self.map.move_to(start_offset);
      self.pos = start;
      self.handle_raw_string(start_offset, start)
    }
  }

  fn handle_false(
    &mut self, start_offset: usize, start: Position,
  ) -> Result<Token<'_>, LexerError<'_>> {
    let remains = ["a", "l", "s", "e"];
    for i in remains {
      if let Some((s, _)) = self.advance()
        && s == i
      {
        self.pos.add_column();
        continue;
      } else {
        self.map.move_to(start_offset);
        self.pos = start;
        return self.handle_raw_string(start_offset, start);
      }
    }

    if let Some(p) = self.peek()
      && (p == " "
        || p == "\r"
        || p == "\n"
        || p == "#"
        || p == ";"
        || p == ","
        || p == "]"
        || p == "}"
        || p == ")")
    {
      Ok(Self::create_token(
        Literal::Boolean(false),
        start,
        self.pos,
        (start_offset, self.current_offset()),
      ))
    } else {
      self.map.move_to(start_offset);
      self.pos = start;
      self.handle_raw_string(start_offset, start)
    }
  }

  fn handle_quoted_string(
    &mut self, start_offset: usize, start: Position,
  ) -> Result<Token<'_>, LexerError<'_>> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut cache = start_offset + 1;
    while let Some((s, offset)) = self.advance() {
      match s {
        "\r" | "\n" => {
          return Err(LexerError::UnexpectedNewline {
            span: Span::new(
              start,
              self.pos,
              (start_offset, self.current_offset()),
            ),
          });
        },
        "\"" => {
          buffer.extend_from_slice(
            self.map.get_by_offset(cache, offset).as_bytes(),
          );
          self.pos.add_column();
          return Ok(Self::create_token(
            Literal::QuotedString(String::from_utf8_lossy(&buffer).to_string()),
            start,
            self.pos,
            (start_offset, self.current_offset()),
          ));
        },
        "\\" => {
          if let Some(s) = self.peek() {
            if s == "n" {
              self.consume();
              self.pos.add_column();
              buffer.extend_from_slice(
                self
                  .map
                  .get_by_offset(cache, self.current_offset() - 2)
                  .as_bytes(),
              );
              buffer.extend_from_slice(b"\n");
              cache = self.current_offset();
              continue;
            } else if s == "r" {
              self.consume();
              self.pos.add_column();
              buffer.extend_from_slice(
                self
                  .map
                  .get_by_offset(cache, self.current_offset() - 2)
                  .as_bytes(),
              );
              buffer.extend_from_slice(b"\r");
              cache = self.current_offset();
              continue;
            } else if s == "t" {
              self.consume();
              self.pos.add_column();
              buffer.extend_from_slice(
                self
                  .map
                  .get_by_offset(cache, self.current_offset() - 2)
                  .as_bytes(),
              );
              buffer.extend_from_slice(b"\t");
              cache = self.current_offset();
              continue;
            } else {
              return Err(LexerError::UndefinedEscape {
                span: Span::new(
                  start,
                  self.pos,
                  (self.current_offset() - 1, self.current_offset()),
                ),
              });
            }
          } else {
            return Err(LexerError::UnexpectedInterruption);
          }
        },
        _ => {
          self.pos.add_column();
          continue;
        },
      }
    }

    Err(LexerError::UnexpectedInterruption)
  }

  fn handle_comment(
    &mut self, start_offset: usize, start: Position,
  ) -> Token<'_> {
    while let Some((s, _)) = self.advance() {
      match s {
        "\n" => {
          self.map.back();
          break;
        },
        _ => {
          self.pos.add_column();
          continue;
        },
      }
    }

    Self::create_token(
      Literal::Comment(""),
      start,
      self.pos,
      (start_offset, self.current_offset()),
    )
  }

  fn handle_number(
    &mut self, start_offset: usize, start: Position, neg: bool,
  ) -> Result<Token<'_>, LexerError<'_>> {
    while let Some((s, _)) = self.advance() {
      match s {
        " " | "\r" | "\n" | "#" | ";" | "," | "]" | "}" | ")" => {
          self.back();
          break;
        },
        other
          if other == "[" || other == "{" || other == "[" || other == "\"" =>
        {
          return Err(LexerError::UnexpectedLiteral {
            literal: other,
            span: Span::new(
              start,
              self.pos,
              (self.current_offset() - 1, self.current_offset()),
            ),
          });
        },
        "." => {
          if let Some(p) = self.peek() {
            if self.is_digital(p) {
              return self.handle_float_number(start_offset, start);
            } else {
              return self.handle_raw_string(start_offset, start);
            }
          } else {
            return self.handle_raw_string(start_offset, start);
          }
        },
        other if self.is_digital(other) => {
          self.pos.add_column();
          continue;
        },
        _ => {
          return self.handle_raw_string(start_offset, start);
        },
      }
    }

    if neg {
      Ok(Self::create_token(
        Literal::SignedIntegerNumber(
          self
            .map
            .get_by_offset(start_offset, self.current_offset())
            .parse()
            .unwrap(),
        ),
        start,
        self.pos,
        (start_offset, self.current_offset()),
      ))
    } else {
      Ok(Self::create_token(
        Literal::UnsignedIntegerNumber(
          self
            .map
            .get_by_offset(start_offset, self.current_offset())
            .parse()
            .unwrap(),
        ),
        start,
        self.pos,
        (start_offset, self.current_offset()),
      ))
    }
  }

  fn handle_float_number(
    &mut self, start_offset: usize, start: Position,
  ) -> Result<Token<'_>, LexerError<'_>> {
    while let Some((s, _)) = self.advance() {
      match s {
        " " | "\r" | "\n" | "#" | ";" | "," | "]" | "}" | ")" => {
          self.back();
          break;
        },
        other
          if other == "[" || other == "{" || other == "[" || other == "\"" =>
        {
          return Err(LexerError::UnexpectedLiteral {
            literal: other,
            span: Span::new(
              start,
              self.pos,
              (self.current_offset() - 1, self.current_offset()),
            ),
          });
        },
        other if self.is_digital(other) => {
          self.pos.add_column();
          continue;
        },
        _ => {
          return self.handle_raw_string(start_offset, start);
        },
      }
    }

    Ok(Self::create_token(
      Literal::FloatNumber(
        self
          .map
          .get_by_offset(start_offset, self.current_offset())
          .parse()
          .unwrap(),
      ),
      start,
      self.pos,
      (start_offset, self.current_offset()),
    ))
  }

  fn handle_raw_string(
    &mut self, start_offset: usize, start: Position,
  ) -> Result<Token<'_>, LexerError<'_>> {
    while let Some((s, _)) = self.advance() {
      match s {
        " " | "\r" | "\n" | "#" | ";" | "," | "]" | "}" | ")" => {
          self.back();
          break;
        },
        other
          if other == "[" || other == "{" || other == "[" || other == "\"" =>
        {
          return Err(LexerError::UnexpectedLiteral {
            literal: other,
            span: Span::new(
              start,
              self.pos,
              (self.current_offset() - 1, self.current_offset()),
            ),
          });
        },
        _ => {
          self.pos.add_column();
          continue;
        },
      }
    }

    Ok(Self::create_token(
      Literal::RawString(
        self.map.get_by_offset(start_offset, self.current_offset()),
      ),
      start,
      self.pos,
      (start_offset, self.current_offset()),
    ))
  }

  fn create_token(
    literal: Literal<'lex>, start: Position, end: Position,
    offsets: (usize, usize),
  ) -> Token<'lex> {
    Token::new(literal, Span::new(start, end, offsets))
  }

  #[inline]
  fn advance(&mut self) -> Option<(&'lex str, usize)> {
    self.map.advance()
  }

  #[inline]
  fn back(&mut self) {
    self.map.back();
  }

  #[inline]
  fn current_offset(&self) -> usize {
    self.map.get_offset()
  }

  #[inline]
  fn peek(&self) -> Option<&'lex str> {
    self.map.peek()
  }

  #[inline]
  fn consume(&mut self) {
    self.map.consume();
  }

  #[allow(dead_code)]
  fn is_one(&self) -> bool{
    if let Some(p) = self.peek() && (p != " " && p != "\n" && p != "\r" && p != "\t" && p != "#" && p != "," && p != ";" && p != "-" && p != "@" && p != "[" && p != "{" && p != "(" && p != "]" && p != "}" && p != ")" && p != "="){
      false
    }else{
      true
    }
  }

  fn skipping_advance(&mut self) -> Option<(&'lex str, usize)> {
    while let Some((s, offset)) = self.map.advance() {
      if s == " " || s == "\t" {
        continue;
      } else if s == "\r" {
        if let Some(n) = self.peek() {
          if n == "\n" {
            self.pos.add_line_by(1);
            self.consume();
          }
        }
        continue;
      } else if s == "\n" {
        self.pos.add_line_by(1);
        continue;
      } else {
        return Some((s, offset));
      }
    }

    None
  }

  fn is_digital(&self, s: &'lex str) -> bool {
    s == "0"
      || s == "1"
      || s == "2"
      || s == "3"
      || s == "4"
      || s == "5"
      || s == "6"
      || s == "7"
      || s == "8"
      || s == "9"
  }
}
