use std::collections::HashSet;

use crate::{
  Map,
  data::Value,
  parse::{error::ParserError, lexer::Lexer, literal::Literal, span::Span},
};

/// A parser parsing tokens from [Lexer].
pub struct Parser<'parser> {
  lexer: Lexer<'parser>,
}

impl<'parser> Parser<'parser> {
  /// Create a parser.
  pub fn new(lexer: Lexer<'parser>) -> Self {
    Self { lexer }
  }

  /// Parse token stream.
  ///
  /// # Errors
  /// [ParserError] will be returned if encounter errors when parsing.
  pub fn parse(mut self) -> Result<Value, ParserError> {
    self.parse_object(false)
  }

  fn parse_object(&mut self, check_brace: bool) -> Result<Value, ParserError> {
    let mut map = Map::new();
    // Keys defined more than once. Their values are collected into an array, so
    // an array written by the user is never confused with the array built here.
    let mut duplicated = HashSet::new();
    loop {
      let token = self.lexer.lex()?;
      let (literal, span) = token.split();
      match literal {
        Literal::QuotedString(s) | Literal::RawString(s) => {
          let key = s;
          self.check_equal(&span)?;
          let (val, check) = self.expect_value(&span)?;
          if check {
            self.check_semicolon(&span)?;
          }
          insert_entry(&mut map, &mut duplicated, key, val);
          continue;
        },
        Literal::End => {
          if check_brace {
            self.check_brace()?;
          }
          return Ok(Value::Object(map));
        },
        Literal::CloseBrace => {
          if check_brace {
            return Ok(Value::Object(map));
          } else {
            return Err(ParserError::UnexpectedCloseBrace(span));
          }
        },
        _ => return Err(ParserError::ExpectKey(literal, span)),
      }
    }
  }

  fn parse_array(&mut self) -> Result<Value, ParserError> {
    let mut arr = Vec::<Value>::new();
    enum State {
      ExpectComma,
      ExpectElement,
    }
    let mut state = State::ExpectElement;
    loop {
      let token = self.lexer.lex()?;
      let (literal, span) = token.split();
      match state {
        State::ExpectElement => match literal {
          Literal::CloseBracket => {
            return Ok(Value::Array(arr));
          },
          other => {
            let val = self.check_element(other, span)?;
            arr.push(val);
            state = State::ExpectComma;
            continue;
          },
        },
        State::ExpectComma => match literal {
          Literal::CloseBracket => {
            return Ok(Value::Array(arr));
          },
          Literal::Comma => state = State::ExpectElement,
          _ => {
            return Err(ParserError::ExpectCommaOrCloseBracket(literal, span));
          },
        },
      }
    }
  }

  fn check_brace(&mut self) -> Result<(), ParserError> {
    let token = self.lexer.lex()?;
    let (literal, span) = token.split();
    if literal.is_close_brace() { Ok(()) } else { Err(ParserError::ExpectBrace(literal, span)) }
  }

  fn check_semicolon(&mut self, other: &Span) -> Result<(), ParserError> {
    let token = self.lexer.lex()?;
    let (literal, span) = token.split();
    if literal.is_semicolon() && span.is_same_line(other) {
      Ok(())
    } else {
      Err(ParserError::ExpectSemicolon(literal, span))
    }
  }

  fn check_equal(&mut self, other: &Span) -> Result<(), ParserError> {
    let token = self.lexer.lex()?;
    let (literal, span) = token.split();
    if literal.is_equal() && span.is_same_line(other) {
      Ok(())
    } else {
      Err(ParserError::ExpectEqual(literal, span))
    }
  }

  fn check_element(&mut self, literal: Literal, span: Span) -> Result<Value, ParserError> {
    match literal {
      Literal::Boolean(b) => Ok(Value::Boolean(b)),
      Literal::FloatNumber(f) => Ok(Value::FloatNumber(f)),
      Literal::QuotedString(s) => Ok(Value::QuotedString(s)),
      Literal::RawString(s) => Ok(Value::RawString(s.to_string())),
      Literal::SignedIntegerNumber(n) => Ok(Value::SignedIntegerNumber(n)),
      Literal::UnsignedIntegerNumber(n) => Ok(Value::UnsignedIntegerNumber(n)),
      Literal::Void => Ok(Value::Void),
      Literal::OpenBrace => Ok(self.parse_object(true)?),
      Literal::OpenBracket => Ok(self.parse_array()?),
      other => Err(ParserError::ExpectValue(other, span)),
    }
  }

  fn expect_value(&mut self, other: &Span) -> Result<(Value, bool), ParserError> {
    let token = self.lexer.lex()?;
    let (literal, span) = token.split();
    match literal {
      Literal::Boolean(b) if span.is_same_line(other) => Ok((Value::Boolean(b), true)),
      Literal::FloatNumber(f) if span.is_same_line(other) => Ok((Value::FloatNumber(f), true)),
      Literal::QuotedString(s) if span.is_same_line(other) => Ok((Value::QuotedString(s), true)),
      Literal::RawString(s) if span.is_same_line(other) => Ok((Value::RawString(s), true)),
      Literal::SignedIntegerNumber(n) if span.is_same_line(other) => {
        Ok((Value::SignedIntegerNumber(n), true))
      },
      Literal::UnsignedIntegerNumber(n) if span.is_same_line(other) => {
        Ok((Value::UnsignedIntegerNumber(n), true))
      },
      Literal::Void if span.is_same_line(other) => Ok((Value::Void, true)),
      Literal::OpenBrace => Ok((self.parse_object(true)?, false)),
      Literal::OpenBracket => Ok((self.parse_array()?, true)),
      other => Err(ParserError::ExpectValue(other, span)),
    }
  }
}

/// Insert an entry into `map`.
///
/// `Marquage` allows a key to be defined more than once. Every occurrence is
/// collected into a [`Value::Array`], so `a = 1; a = 2;` gives `Array([1, 2])`
/// and can be parsed into a `Vec<T>`. A key defined only once keeps its own
/// value.
///
/// An array written by the user is never flattened, so the array's length always
/// equals the count of the key's definitions: `a = [1, 2]; a = [3];` gives
/// `Array([Array([1, 2]), Array([3])])`. `duplicated` records the keys whose
/// value has already been turned into an array, telling them apart from the
/// arrays written by the user.
fn insert_entry(map: &mut Map, duplicated: &mut HashSet<String>, key: String, value: Value) {
  // The first occurrence.
  if map.get(&key).is_none() {
    map.insert(key, value);
    return;
  }

  let slot = map.get_mut(&key).expect("the key is defined before");
  if duplicated.contains(&key) {
    // The third and later occurrences. The value is an array here.
    if let Value::Array(arr) = slot {
      arr.push(value);
    }
  } else {
    // The second occurrence turns the value into an array.
    let first = std::mem::replace(slot, Value::Void);
    *slot = Value::Array(vec![first, value]);
    duplicated.insert(key);
  }
}
