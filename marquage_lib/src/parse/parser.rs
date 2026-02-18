use crate::{parse::{error::ParserError, lexer::Lexer}, value::Value};

enum State {
  Pending,

}

pub struct Parser<'parser>{
  lexer: Lexer<'parser>,
  state: State
}

impl<'parser> Parser<'parser> {
  pub fn new(lexer: Lexer<'parser>) -> Self {
    Self {
      lexer,
      state: State::Pending
    }
  }

  pub fn parse(&mut self) -> Result<Value, ParserError<'_>> {
    loop {
      match self.lexer.lex() {
        Ok(token) => {todo!()},
        Err(e) => {
          return Err(ParserError::LexingError(e))
        }
      }
    }
  }
}
