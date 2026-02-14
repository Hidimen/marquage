use crate::parser::{literal::Literal, span::Span};

#[derive(Debug)]
pub struct Token<'token>(Literal<'token>, Span);

impl<'token> Token<'token> {
  pub fn new(literal: Literal<'token>, span: Span) -> Self {
    Self(literal, span)
  }

  pub fn get_literal(&self) -> &Literal<'token> {
    &self.0
  }

  pub fn get_span(&self) -> Span {
    self.1
  }
}