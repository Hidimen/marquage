use std::fmt::Display;

use paste::paste;

use crate::{Generable, Parseable};

use super::index::Index;

/// Storing object data, using [indexmap](https://docs.rs/indexmap/latest/indexmap/) to implement order storage.
pub type ObjectImpl = indexmap::IndexMap<String, Value>;
/// Storing array data.
pub type ArrayImpl = Vec<Value>;

/// Values that represent data structure in `Marquage`.
///
/// |[`Value`]|Data Structure|
/// |:-:|:-:|
/// |Void|void|
/// |RawString|string|
/// |QuotedString|"string"|
/// |Boolean|true<br>false|
/// |FloatNumber|0.1<br>-0.1|
/// |UnsignedIntegerNumber|1|
/// |SignedIntegerNumber|-1|
/// |Object|{}|
/// |Array|[]|
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  /// Representing literal `void`.
  Void,

  // Representing quoted string.
  QuotedString(String),
  // Representing raw string.
  RawString(String),

  /// Representing boolean.
  Boolean(bool),

  /// Representing float number.
  FloatNumber(f32),
  /// Representing unsigned integer number.
  UnsignedIntegerNumber(u32),
  /// Representing signed integer number.
  SignedIntegerNumber(i32),

  /// Representing an object.
  Object(ObjectImpl),
  /// Representing an array.
  Array(ArrayImpl),
}

impl<T> std::ops::Index<T> for Value
where
  T: Index,
{
  type Output = Value;

  fn index(&self, index: T) -> &Self::Output {
    static VOID: Value = Value::Void;
    index.index_into(self).unwrap_or(&VOID)
  }
}

impl<T> std::ops::IndexMut<T> for Value
where
  T: Index + Display,
{
  fn index_mut(&mut self, index: T) -> &mut Value {
    let data = index.index_into_mut(self);
    match data {
      Some(c) => c,
      None => {
        panic!("No such element indexed by {}", index)
      },
    }
  }
}

macro_rules! impl_enum_methods {
  ($name:ident, $variant:ident, $ret:ty) => {
    paste! {
      #[doc = "check if value is " $name "."]
      pub fn [<is_ $name>](&self) -> bool {
          matches!(self, Self::$variant(_))
      }

      #[doc = "get content ref of " $name "."]
      pub fn [<as_ $name _ref>](&self) -> Option<&$ret> {
        match self {
          Self::$variant(obj) => Some(obj),
          _ => None,
        }
      }

      #[doc = "get mutable content ref of " $name "."]
      pub fn [<as_ $name _mut>](&mut self) -> Option<&mut $ret> {
        match self {
          Self::$variant(obj) => Some(obj),
          _ => None,
        }
      }

      #[doc = "get content of " $name "."]
      pub fn [<as_ $name>](self) -> Option<$ret> {
        match self {
          Self::$variant(obj) => Some(obj),
          _ => None,
        }
      }
    }
  };
}

impl Value {
  impl_enum_methods!(object, Object, ObjectImpl);
  impl_enum_methods!(array, Array, ArrayImpl);
  impl_enum_methods!(boolean, Boolean, bool);
  impl_enum_methods!(unsigned_number, UnsignedIntegerNumber, u32);
  impl_enum_methods!(signed_number, SignedIntegerNumber, i32);
  impl_enum_methods!(float_number, FloatNumber, f32);

  #[doc = "check if value is void."]
  pub fn is_void(&self) -> bool {
    matches!(self, Self::Void)
  }

  #[doc = "check if value is quoted string."]
  pub fn is_quoted_string(&self) -> bool {
    matches!(self, Self::QuotedString(..))
  }

  #[doc = "check if value is raw string."]
  pub fn is_raw_string(&self) -> bool {
    matches!(self, Self::RawString(..))
  }

  #[doc = "get content ref of string."]
  pub fn as_string_ref(&self) -> Option<&String> {
    match self {
      Self::QuotedString(s) | Self::RawString(s) => Some(s),
      _ => None,
    }
  }

  #[doc = "get mutable content ref of string."]
  pub fn as_string_mut(&mut self) -> Option<&mut String> {
    match self {
      Self::QuotedString(s) | Self::RawString(s) => Some(s),
      _ => None,
    }
  }

  #[doc = "get content of string."]
  pub fn as_string(self) -> Option<String> {
    match self {
      Self::QuotedString(s) | Self::RawString(s) => Some(s),
      _ => None,
    }
  }
}

macro_rules! impl_p_for_unsigned {
  ($($i: ident),*) => {
    $(
      impl Parseable for $i {
        fn parse(v: Value) -> Result<Self, crate::error::CastError> {
          match v {
            Value::UnsignedIntegerNumber(n) => Ok(n as $i),
            _ => Err(crate::error::CastError::IncompatibleType),
          }
        }
      }
    )*
  };
}

impl_p_for_unsigned!(u8, u16, u32, u64, u128);

macro_rules! impl_p_for_signed {
  ($($i: ident),*) => {
    $(
      impl Parseable for $i {
        fn parse(v: Value) -> Result<Self, crate::error::CastError> {
          match v {
            Value::SignedIntegerNumber(n) => Ok(n as $i),
            _ => Err(crate::error::CastError::IncompatibleType),
          }
        }
      }
    )*
  };
}

impl_p_for_signed!(i8, i16, i32, i64, i128);

impl Parseable for bool {
  fn parse(v: Value) -> Result<Self, crate::error::CastError> {
    match v {
      Value::Boolean(b) => Ok(b),
      _ => Err(crate::error::CastError::IncompatibleType),
    }
  }
}

impl Parseable for String {
  fn parse(v: Value) -> Result<Self, crate::error::CastError> {
    match v {
      Value::RawString(s) | Value::QuotedString(s) => Ok(s),
      _ => Err(crate::error::CastError::IncompatibleType),
    }
  }
}

impl Parseable for f32 {
  fn parse(v: Value) -> Result<Self, crate::error::CastError> {
    match v {
      Value::FloatNumber(f) => Ok(f),
      _ => Err(crate::error::CastError::IncompatibleType),
    }
  }
}

impl Parseable for f64 {
  fn parse(v: Value) -> Result<Self, crate::error::CastError> {
    match v {
      Value::FloatNumber(f) => Ok(f as f64),
      _ => Err(crate::error::CastError::IncompatibleType),
    }
  }
}

impl<T: Parseable> Parseable for Box<T> {
  fn parse(v: Value) -> Result<Self, crate::error::CastError> {
    Ok(Box::new(T::parse(v)?))
  }
}

impl Parseable for Box<str> {
  fn parse(v: Value) -> Result<Self, crate::error::CastError> {
    match v {
      Value::RawString(s) | Value::QuotedString(s) => Ok(s.into_boxed_str()),
      _ => Err(crate::error::CastError::IncompatibleType),
    }
  }
}

impl<T: Parseable> Parseable for Vec<T> {
  fn parse(v: Value) -> Result<Self, crate::error::CastError> {
    match v {
      Value::Array(arr) => {
        let mut res = Vec::with_capacity(arr.len());
        for i in arr {
          res.push(T::parse(i)?);
        }
        Ok(res)
      },
      _ => Err(crate::error::CastError::IncompatibleType),
    }
  }
}

impl<T: Parseable> Parseable for Option<T> {
  fn parse(v: Value) -> Result<Self, crate::error::CastError> {
    Ok(Some(T::parse(v)?))
  }
}

impl<T: Parseable> Parseable for Box<[T]> {
  fn parse(v: Value) -> Result<Self, crate::error::CastError> {
    match v {
      Value::Array(arr) => {
        let res: Result<Vec<T>, _> = arr.into_iter().map(T::parse).collect();
        Ok(res?.into_boxed_slice())
      },
      _ => Err(crate::error::CastError::IncompatibleType),
    }
  }
}

impl Parseable for () {
  fn parse(v: Value) -> Result<Self, crate::error::CastError> {
    match v {
      Value::Void => Ok(()),
      Value::Array(arr) if arr.is_empty() => Ok(()),
      _ => Err(crate::error::CastError::IncompatibleType),
    }
  }
}

macro_rules! impl_p_for_array {
  ($($n: expr),+) => {
    $(
      impl<T:Parseable> Parseable for [T; $n] {
        fn parse(v: Value) -> Result<Self, crate::error::CastError> {
          match v {
            Value::Array(arr) => {
              if arr.len() != $n {
                return Err(crate::error::CastError::IncompatibleType);
              }

              let res: Result<Vec<T>, _> = arr.into_iter().map(T::parse).collect();
              res?.try_into().map_err(|_| crate::error::CastError::IncompatibleType)
            },
            _ => Err(crate::error::CastError::IncompatibleType)
          }
        }
      }
    )*
  };
}

impl_p_for_array! {
  1, 2, 3, 4, 5, 6, 7, 8, 9,
  10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
}

macro_rules! impl_g_for_unsigned {
  ($($i: ident),*) => {
    $(
      impl Generable for $i {
        fn generate(self) -> Value {
          Value::UnsignedIntegerNumber(self as u32)
        }

        fn generate_ref(&self) -> Value {
          Value::UnsignedIntegerNumber(*self as u32)
        }
      }
    )*
  };
}
impl_g_for_unsigned!(u8, u16, u32, u64, u128);

macro_rules! impl_g_for_signed {
  ($($i: ident),*) => {
    $(
      impl Generable for $i {
        fn generate(self) -> Value {
          Value::SignedIntegerNumber(self as i32)
        }

        fn generate_ref(&self) -> Value {
          Value::SignedIntegerNumber(*self as i32)
        }
      }
    )*
  };
}

impl_g_for_signed!(i8, i16, i32, i64, i128);

impl Generable for bool {
  fn generate(self) -> Value {
    Value::Boolean(self)
  }

  fn generate_ref(&self) -> Value {
    Value::Boolean(*self)
  }
}

impl Generable for String {
  fn generate(self) -> Value {
    Value::QuotedString(self)
  }

  fn generate_ref(&self) -> Value {
    Value::QuotedString(self.clone())
  }
}

impl Generable for &str {
  fn generate(self) -> Value {
    Value::QuotedString(self.to_string())
  }

  fn generate_ref(&self) -> Value {
    Value::QuotedString(self.to_string())
  }
}

impl Generable for f32 {
  fn generate(self) -> Value {
    Value::FloatNumber(self)
  }

  fn generate_ref(&self) -> Value {
    Value::FloatNumber(*self)
  }
}

impl Generable for f64 {
  fn generate(self) -> Value {
    Value::FloatNumber(self as f32)
  }

  fn generate_ref(&self) -> Value {
    Value::FloatNumber(*self as f32)
  }
}

impl<T: Generable> Generable for Box<T> {
  fn generate(self) -> Value {
    T::generate(*self)
  }

  fn generate_ref(&self) -> Value {
    T::generate_ref(self)
  }
}

impl Generable for Box<str> {
  fn generate(self) -> Value {
    Value::QuotedString(self.to_string())
  }

  fn generate_ref(&self) -> Value {
    Value::QuotedString(self.to_string())
  }
}

impl<T: Generable> Generable for Vec<T> {
  fn generate(self) -> Value {
    Value::Array(self.into_iter().map(T::generate).collect())
  }

  fn generate_ref(&self) -> Value {
    Value::Array(self.iter().map(T::generate_ref).collect())
  }
}

impl<T: Generable> Generable for Option<T> {
  fn generate(self) -> Value {
    if let Some(val) = self { T::generate(val) } else { Value::Void }
  }

  fn generate_ref(&self) -> Value {
    if let Some(val) = self { T::generate_ref(val) } else { Value::Void }
  }
}

impl<T: Generable> Generable for Box<[T]> {
  fn generate(self) -> Value {
    Value::Array(self.into_iter().map(T::generate).collect())
  }

  fn generate_ref(&self) -> Value {
    Value::Array(self.iter().map(T::generate_ref).collect())
  }
}

impl Generable for () {
  fn generate(self) -> Value {
    Value::Void
  }

  fn generate_ref(&self) -> Value {
    Value::Void
  }
}
