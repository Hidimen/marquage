use crate::{data::Value, error::CastError};

pub trait Parseable: Sized {
  fn parse(v: Value) -> Result<Self, CastError>;
}
