use crate::parse::{literal::Literal, span::Span};

/// Containing a literal and a span.
#[derive(Debug)]
pub struct Token(Literal, Span);

impl Token {
  /// Create a new token.
  pub fn new(literal: Literal, span: Span) -> Self {
    Self(literal, span)
  }

  /// Get the ref of literal.
  pub fn get_literal_ref(&self) -> &Literal {
    &self.0
  }

  /// Get the literal.
  pub fn get_literal(self) -> Literal {
    self.0
  }

  /// Get span.
  pub fn get_span(&self) -> Span {
    self.1
  }

  /// Consume this token and split it into (literal, span).
  pub fn split(self) -> (Literal, Span) {
    (self.0, self.1)
  }
}
