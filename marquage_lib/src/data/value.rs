use std::fmt::Display;

use indexmap::IndexMap;
use paste::paste;

use crate::Parseable;

use super::index::Index;

/// Values that represent data structure in `Marquage`
///
/// |[`Value`]|Data Structure|
/// |:-:|:-:|
/// |Void|void|
/// |String|"string"<br>string<br>'string'|
/// |Boolean|true<br>false|
/// |FloatNumber|0.1<br>-0.1|
/// |UnsignedIntegerNumber|1|
/// |SignedIntegerNumber|-1|
/// |Object|{}|
/// |Array|[]|
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Void,

  QuotedString(String),
  RawString(String),

  Boolean(bool),

  FloatNumber(f32),
  UnsignedIntegerNumber(u32),
  SignedIntegerNumber(i32),

  Object(IndexMap<String, Value>),

  Array(Vec<Value>),
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
        pub fn [<is_ $name>](&self) -> bool {
            matches!(self, Self::$variant(_))
        }

        pub fn [<as_ $name _ref>](&self) -> Option<&$ret> {
            match self {
                Self::$variant(obj) => Some(obj),
                _ => None,
            }
        }

        pub fn [<as_ $name _mut>](&mut self) -> Option<&mut $ret> {
            match self {
                Self::$variant(obj) => Some(obj),
                _ => None,
            }
        }

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
  impl_enum_methods!(object, Object, IndexMap<String,Value>);
  impl_enum_methods!(array, Array, Vec<Value>);
  impl_enum_methods!(boolean, Boolean, bool);
  impl_enum_methods!(unsigned_number, UnsignedIntegerNumber, u32);
  impl_enum_methods!(signed_number, SignedIntegerNumber, i32);
  impl_enum_methods!(float_number, FloatNumber, f32);

  pub fn is_void(&self) -> bool {
    matches!(self, Self::Void)
  }

  pub fn is_quoted_string(&self) -> bool {
    matches!(self, Self::QuotedString(..))
  }

  pub fn is_raw_string(&self) -> bool {
    matches!(self, Self::RawString(..))
  }

  pub fn as_string_ref(&self) -> Option<&String> {
    match self {
      Self::QuotedString(s) | Self::RawString(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_string_mut(&mut self) -> Option<&mut String> {
    match self {
      Self::QuotedString(s) | Self::RawString(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_string(self) -> Option<String> {
    match self {
      Self::QuotedString(s) | Self::RawString(s) => Some(s),
      _ => None,
    }
  }
}

macro_rules! impl_for_unsigned {
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

impl_for_unsigned!(u8, u16, u32, u64, u128);

macro_rules! impl_for_signed {
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

impl_for_signed!(i8, i16, i32, i64, i128);

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

macro_rules! impl_for_array {
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

impl_for_array! {
  1, 2, 3, 4, 5, 6, 7, 8, 9,
  10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
}
