use crate::parse::{literal::Literal, span::Span};

#[derive(Debug)]
pub struct Token(Literal, Span);

impl Token {
  pub fn new(literal: Literal, span: Span) -> Self {
    Self(literal, span)
  }

  pub fn get_literal_ref(&self) -> &Literal {
    &self.0
  }

  pub fn get_literal(self) -> Literal {
    self.0
  }

  pub fn get_span(&self) -> Span {
    self.1
  }

  pub fn split(self) -> (Literal, Span) {
    (self.0, self.1)
  }
}
