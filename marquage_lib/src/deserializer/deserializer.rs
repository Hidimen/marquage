use std::collections::BTreeMap;

use crate::{
  map::{ArrayImpl, ObjectImpl},
  value::Value,
};

use super::{
  error::DeserializerError, lexer::Lexer, literal::Literal, position::Position, token::Token,
};

/// Tool struct helping parse data
///
/// # Example
/// ```rust
/// # use marquage::deserializer::lexer::Lexer;
/// # use marquage::deserializer::Deserializer;
///
/// let data = r###"hello world;"###.to_string();
/// let lexer = Lexer::new(data);
/// let deserializer = Deserializer::new(lexer);
/// let value = deserializer.parse().unwrap();
///
/// assert_eq!(value["hello"].as_string_ref().unwrap().as_str(), "world");
/// ```
pub struct Deserializer {
  lexer: Lexer,
  state: State,
  references: BTreeMap<String, Value>,
  pos_cache: Position,
}

impl Deserializer {
  /// Create a new instance
  pub fn new(lexer: Lexer) -> Self {
    Self {
      lexer,
      state: State::Pending,
      references: BTreeMap::<String, Value>::new(),
      pos_cache: Position::new(0, 0),
    }
  }

  /// Parse data from [`Lexer`]
  pub fn parse(mut self) -> Result<Value, DeserializerError> {
    match self.parse_object(false) {
      Ok(map) => Ok(Value::Object(map)),
      Err(e) => Err(e),
    }
  }

  fn parse_object(&mut self, check_brace: bool) -> Result<ObjectImpl, DeserializerError> {
    let mut map: ObjectImpl = ObjectImpl::new();
    loop {
      let token: Result<Token, DeserializerError> = self.lexer.get();
      match token {
        Ok(t) => match &self.state {
          State::Pending | State::ObjectProcessing => {
            let pos = *t.get_position();
            let literal = t.get_literal();
            if literal.is_string() {
              self.set_state(State::KeyProcessing(literal, pos.get_line()));
              continue;
            } else if literal.is_end() {
              if check_brace {
                return Err(DeserializerError::UnexpectedEnd);
              }
              self.set_state(State::Finishing);
            } else if literal.is_close_brace() {
              if check_brace {
                self.pos_cache = pos;
                return Ok(map);
              } else {
                return Err(DeserializerError::InvalidLiteral(literal));
              }
            } else if literal.is_reference() {
              self.set_state(State::ReferenceProcessing(literal, pos.get_line()));
              continue;
            } else if literal.is_comment() {
              continue;
            } else {
              return Err(DeserializerError::InvalidLiteral(literal));
            }
          },
          State::Finishing => {
            break;
          },
          State::KeyProcessing(key_token, line) => {
            let pos = *t.get_position();
            let literal = t.get_literal();
            if !pos.is_same_line(*line) {
              return Err(DeserializerError::InvalidLiteral(literal));
            }
            if literal.is_string()
              || literal.is_unsigned_int()
              || literal.is_signed_int()
              || literal.is_float()
              || literal.is_boolean()
              || literal.is_void()
            {
              map.insert(key_token.get_string_content().unwrap(), literal.into());
              self.set_state(State::SemicolonExpected(pos.into()));
              continue;
            } else if literal.is_dereference() {
              if let Some(val) = self.references.get(&literal.get_dereference_content().unwrap()) {
                map.insert(key_token.get_string_content().unwrap(), val.clone());
                self.set_state(State::SemicolonExpected(pos.into()));
                continue;
              } else {
                return Err(DeserializerError::UnknownReference(
                  literal.get_dereference_content().unwrap(),
                ));
              }
            } else if literal.is_open_brace() {
              let key_name = key_token.get_string_content().unwrap();
              self.set_state(State::ObjectProcessing);
              match self.parse_object(true) {
                Ok(object) => {
                  map.insert(key_name, Value::Object(object));
                  continue;
                },
                Err(e) => {
                  return Err(e);
                },
              }
            } else if literal.is_open_bracket() {
              let key_name = key_token.get_string_content().unwrap();
              self.set_state(State::ArrayProcessing);
              match self.parse_array(true) {
                Ok(array) => {
                  map.insert(key_name, Value::Array(array));
                  continue;
                },
                Err(e) => {
                  return Err(e);
                },
              }
            } else if literal.is_comment() {
              continue;
            } else {
              return Err(DeserializerError::InvalidLiteral(literal));
            }
          },
          State::ReferenceProcessing(reference_token, line) => {
            let pos = *t.get_position();
            let literal = t.get_literal();
            if !pos.is_same_line(*line) {
              return Err(DeserializerError::InvalidLiteral(literal));
            }
            if literal.is_string()
              || literal.is_unsigned_int()
              || literal.is_signed_int()
              || literal.is_float()
              || literal.is_boolean()
              || literal.is_void()
            {
              self
                .references
                .insert(reference_token.get_reference_content().unwrap(), literal.into());
              self.set_state(State::SemicolonExpected(pos.into()));
              continue;
            } else if literal.is_open_brace() {
              let ref_name = reference_token.get_reference_content().unwrap();
              self.set_state(State::ObjectProcessing);
              match self.parse_object(true) {
                Ok(object) => {
                  self.references.insert(ref_name, Value::Object(object));
                  continue;
                },
                Err(e) => {
                  return Err(e);
                },
              }
            } else if literal.is_open_bracket() {
              let ref_name = reference_token.get_reference_content().unwrap();
              self.set_state(State::ArrayProcessing);
              match self.parse_array(true) {
                Ok(array) => {
                  self.references.insert(ref_name, Value::Array(array));
                  continue;
                },
                Err(e) => {
                  return Err(e);
                },
              }
            } else if literal.is_comment() {
              continue;
            } else {
              return Err(DeserializerError::InvalidLiteral(literal));
            }
          },
          State::SemicolonExpected(position) => {
            if t.get_literal().is_semicolon() {
              self.set_state(State::Pending);
              continue;
            } else {
              return Err(DeserializerError::MissingSemicolon(*position));
            }
          },
          State::ArrayProcessing | State::CommaExpected(_) => {
            return Err(DeserializerError::WrongState);
          },
        },
        Err(e) => {
          return Err(e);
        },
      }
    }
    Ok(map)
  }

  fn parse_array(&mut self, check_semicolon: bool) -> Result<ArrayImpl, DeserializerError> {
    let mut map: ArrayImpl = ArrayImpl::new();
    loop {
      let token: Result<Token, DeserializerError> = self.lexer.get();
      match token {
        Ok(t) => match &self.state {
          State::ArrayProcessing => {
            let pos = *t.get_position();
            let literal = t.get_literal();
            if literal.is_string()
              || literal.is_unsigned_int()
              || literal.is_signed_int()
              || literal.is_float()
              || literal.is_boolean()
              || literal.is_void()
            {
              map.push(literal.into());
              self.set_state(State::CommaExpected(pos.into()));
              continue;
            } else if literal.is_dereference() {
              if let Some(val) = self.references.get(&literal.get_dereference_content().unwrap()) {
                map.push(val.clone());
                self.set_state(State::CommaExpected(pos.into()));
                continue;
              } else {
                return Err(DeserializerError::UnknownReference(
                  literal.get_dereference_content().unwrap(),
                ));
              }
            } else if literal.is_open_brace() {
              self.set_state(State::ObjectProcessing);
              match self.parse_object(true) {
                Ok(object) => {
                  map.push(Value::Object(object));
                },
                Err(e) => {
                  return Err(e);
                },
              }
              self.set_state(State::CommaExpected(self.pos_cache));
            } else if literal.is_open_bracket() {
              self.set_state(State::ArrayProcessing);
              match self.parse_array(false) {
                Ok(array) => {
                  map.push(Value::Array(array));
                  continue;
                },
                Err(e) => {
                  return Err(e);
                },
              }
            } else if literal.is_end() {
              return Err(DeserializerError::UnexpectedEnd);
            } else {
              return Err(DeserializerError::InvalidLiteral(literal));
            }
          },
          State::CommaExpected(pos) => {
            let literal = t.get_literal();
            if literal.is_close_bracket() {
              if check_semicolon {
                self.set_state(State::SemicolonExpected(*pos));
              } else {
                self.set_state(State::CommaExpected(*pos));
              }
              break;
            } else if literal.is_comma() {
              self.set_state(State::ArrayProcessing);
              continue;
            } else {
              return Err(DeserializerError::InvalidLiteral(literal));
            }
          },
          _ => {
            return Err(DeserializerError::WrongState);
          },
        },
        Err(e) => {
          return Err(e);
        },
      }
    }
    Ok(map)
  }

  fn set_state(&mut self, state: State) {
    self.state = state;
  }
}

enum State {
  Pending,

  Finishing,

  ReferenceProcessing(Literal, usize),

  KeyProcessing(Literal, usize),

  SemicolonExpected(Position),
  CommaExpected(Position),

  ObjectProcessing,
  ArrayProcessing,
}
