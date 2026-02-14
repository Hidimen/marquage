use std::io::Error;

use indexmap::IndexMap;

use crate::{deserializer::error::DeserializerError, value::Value};

/// A data type that implements this can be deserialized
pub trait Deserializable: Sized {
  /// Deserialize [`Value`](crate::value::Value) into the deserializable data itself
  fn deserialize(v: &Value) -> Result<Self, DeserializableError>;
}

macro_rules! impl_unsigned_number {
  ($ty:ident) => {
    impl Deserializable for $ty {
      fn deserialize(v: &Value) -> Result<Self, DeserializableError> {
        match v {
          Value::UnsignedIntegerNumber(n) => Ok(*n as $ty),
          _ => Err(DeserializableError::UnmatchedType(stringify!($ty))),
        }
      }
    }
  };
}

macro_rules! impl_signed_number {
  ($ty:ident) => {
    impl Deserializable for $ty {
      fn deserialize(v: &Value) -> Result<Self, DeserializableError> {
        match v {
          Value::SignedIntegerNumber(n) => Ok(*n as $ty),
          _ => Err(DeserializableError::UnmatchedType(stringify!($ty))),
        }
      }
    }
  };
}

impl_unsigned_number!(u8);
impl_unsigned_number!(u16);
impl_unsigned_number!(u32);
impl_unsigned_number!(u64);
impl_unsigned_number!(usize);
impl_signed_number!(i8);
impl_signed_number!(i16);
impl_signed_number!(i32);
impl_signed_number!(i64);
impl_signed_number!(isize);

impl Deserializable for f32 {
  fn deserialize(v: &Value) -> Result<Self, DeserializableError> {
    match v {
      Value::FloatNumber(n) => Ok(*n as f32),
      _ => Err(DeserializableError::UnmatchedType("f32")),
    }
  }
}

impl Deserializable for bool {
  fn deserialize(v: &Value) -> Result<Self, DeserializableError> {
    match v {
      Value::Boolean(b) => Ok(*b),
      _ => Err(DeserializableError::UnmatchedType("bool")),
    }
  }
}

impl Deserializable for () {
  fn deserialize(v: &Value) -> Result<Self, DeserializableError> {
    match v {
      Value::Void => Ok(()),
      _ => Err(DeserializableError::UnmatchedType("()")),
    }
  }
}

impl Deserializable for String {
  fn deserialize(v: &Value) -> Result<Self, DeserializableError> {
    match v {
      Value::String(data, _) => Ok(data.clone()),
      _ => Err(DeserializableError::UnmatchedType("string")),
    }
  }
}

impl<T: Deserializable> Deserializable for Vec<T> {
  fn deserialize(v: &Value) -> Result<Self, DeserializableError> {
    match v {
      Value::Array(arr) => arr.iter().map(|item| T::deserialize(item)).collect(),
      _ => Err(DeserializableError::UnmatchedType("vec")),
    }
  }
}

impl<T: Deserializable> Deserializable for IndexMap<String, T> {
  fn deserialize(v: &Value) -> Result<Self, DeserializableError> {
    match v {
      Value::Object(obj) => obj
        .clone()
        .into_iter()
        .map(|(k, val)| match T::deserialize(&val) {
          Ok(val) => Ok((k, val)),
          Err(e) => Err(e),
        })
        .collect(),
      _ => Err(DeserializableError::UnmatchedType("map")),
    }
  }
}

impl<T: Deserializable> Deserializable for Option<T> {
  fn deserialize(v: &Value) -> Result<Self, DeserializableError> {
    if let Value::Void = v {
      return Ok(None);
    }

    match T::deserialize(v) {
      Ok(val) => Ok(Some(val)),
      Err(e) => Err(e),
    }
  }
}

impl<T: Deserializable> Deserializable for Box<T> {
  fn deserialize(v: &Value) -> Result<Self, DeserializableError> {
    match T::deserialize(v) {
      Ok(val) => Ok(Box::new(val)),
      Err(e) => Err(e),
    }
  }
}

impl Deserializable for Value {
  fn deserialize(v: &Value) -> Result<Self, DeserializableError> {
    Ok(v.clone())
  }
}

#[derive(Debug)]
pub enum DeserializableError {
  UnknownError,
  UnmatchedType(&'static str),
  MissingField(&'static str),
  ParsingError(DeserializerError),
  UnmatchedValue(&'static str),
  ConvertError(&'static str, &'static str),
  IOError(Error),
}

impl std::fmt::Display for DeserializableError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::UnknownError => write!(f, "Unknown error"),
      Self::UnmatchedType(name) => write!(f, "Unmatched type, but expected {}", name),
      Self::MissingField(name) => write!(f, "Missing field: {}", name),
      Self::ParsingError(e) => write!(f, "Parsing Error: {}", e),
      Self::UnmatchedValue(v) => write!(f, "Expecting {}, but received others", v),
      Self::ConvertError(from, to) => {
        write!(f, "Failed to convert number from {} to {}", from, to)
      },
      Self::IOError(e) => write!(f, "An IO error occurred: {}", e),
    }
  }
}
