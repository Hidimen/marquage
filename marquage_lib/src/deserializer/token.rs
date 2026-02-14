use super::{literal::Literal, position::Position};

#[derive(Debug)]
pub struct Token(Literal, Position);

impl Token {
  pub fn new(literal: Literal, position: Position) -> Self {
    Self(literal, position)
  }

  #[inline(always)]
  pub fn get_literal_ref(&self) -> &Literal {
    &self.0
  }

  #[inline(always)]
  pub fn get_literal(self) -> Literal {
    self.0
  }

  #[inline(always)]
  pub fn get_position(&self) -> &Position {
    &self.1
  }
}
