use crate::{
  data::Value,
  parse::{error::ParserError, lexer::Lexer, parser::Parser},
};

pub fn from_str(data: &str) -> Result<Value, ParserError> {
  let lexer = Lexer::new(data);
  let parser = Parser::new(lexer);
  parser.parse()
}
