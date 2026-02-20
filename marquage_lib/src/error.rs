use std::fmt::Display;

#[derive(Debug)]
pub enum CastError {
  IncompatibleType,
  FieldNotFound(String),
}

impl Display for CastError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::IncompatibleType => write!(f, "Incompatible type"),
      Self::FieldNotFound(field) => write!(f, "Field {field} is not found"),
    }
  }
}
