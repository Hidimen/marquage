//! Library error.
use std::fmt::Display;

use crate::parse::error::ParserError;

/// Type casting error.
///
/// Usually happens when [Value](crate::data::Value) converts to Rust data structures.
#[derive(Debug)]
pub enum CastError {
  /// Types are mismatched.
  IncompatibleType,
  /// Needed field is not found in a map collection.
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
impl std::error::Error for CastError {}

/// General library error.
#[derive(Debug)]
pub enum Error {
  /// Representing [CastError].
  Cast(CastError),
  /// Representing [ParserError].
  Parse(ParserError),
  /// A [u8] slice is an invalid UTF-8 string.
  InvalidSlice,
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Cast(c) => write!(f, "{c}"),
      Self::Parse(p) => write!(f, "{p}"),
      Self::InvalidSlice => write!(f, "Slice is not a valid UTF-8 dataset"),
    }
  }
}

impl std::error::Error for Error {}

impl From<CastError> for Error {
  fn from(value: CastError) -> Self {
    Self::Cast(value)
  }
}

impl From<ParserError> for Error {
  fn from(value: ParserError) -> Self {
    Self::Parse(value)
  }
}
