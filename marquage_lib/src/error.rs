use std::fmt::Display;

#[derive(Debug)]
pub enum CastError {
  IncompatibleType,
}

impl Display for CastError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::IncompatibleType => write!(f, "Incompatible type"),
    }
  }
}
