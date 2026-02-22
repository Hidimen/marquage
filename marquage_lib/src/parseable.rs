use crate::{data::Value, error::CastError};

/// A data structure that can be initialized from a [Value].
pub trait Parseable: Sized {
  /// Parse from [Value].
  fn parse(v: Value) -> Result<Self, CastError>;
}
