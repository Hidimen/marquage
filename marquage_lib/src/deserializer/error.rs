use super::{literal::Literal, position::Position};
use std::fmt::Display;

#[derive(Debug)]
pub enum DeserializerError {
  UnexpectedLiteral(char, Position),
  UnexpectedTermination(Position),
  InvalidNewLine(Position),
  NoSetsFound(Position),
  InvalidCommand(String, Position),
  InvalidLiteral(Literal),
  UnknownReference(String),
  MissingSemicolon(Position),
  WrongState,
  UnexpectedEnd,
}

impl Display for DeserializerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::UnexpectedLiteral(val, position) => write!(
        f,
        "Lexing: Unexpected literal {}, occurred at line {}, column {}",
        val,
        position.get_line(),
        position.get_column()
      ),
      Self::UnexpectedTermination(position) => write!(
        f,
        "Lexing: Unexpected termination occurred at line {}, column {}",
        position.get_line(),
        position.get_column()
      ),
      Self::InvalidNewLine(position) => write!(
        f,
        "Lexing: Invalid new line occurred at line {}, column {}",
        position.get_line(),
        position.get_column()
      ),
      Self::NoSetsFound(position) => write!(
        f,
        "Lexing: Undefined set involved at line {}, column {}",
        position.get_line(),
        position.get_column()
      ),
      Self::InvalidCommand(name, position) => write!(
        f,
        "Lexing: Invalid command {} involved at line {}, column {}",
        name,
        position.get_line(),
        position.get_column()
      ),
      Self::InvalidLiteral(value) => write!(f, "Parsing: Unexpected token: {:?}", value),
      Self::UnknownReference(name) => write!(f, "Parsing: Unknown reference {}", name),
      Self::MissingSemicolon(position) => write!(
        f,
        "Parsing: Semicolon is missing, occurred at line {}, column {}",
        position.get_line(),
        position.get_column()
      ),
      Self::WrongState => write!(f, "Parsing: Wrong state when parsing"),
      Self::UnexpectedEnd => write!(f, "Parsing: Unexpected end"),
    }
  }
}

impl std::error::Error for DeserializerError {}
