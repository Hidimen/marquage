use std::fmt::Display;

use indexmap::IndexMap;

use super::index::Index;

use paste::paste;

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

  String(String, StringType),

  Boolean(bool),

  FloatNumber(f32),
  UnsignedIntegerNumber(u32),
  SignedIntegerNumber(i32),

  Object(IndexMap<String, Value>),

  Array(Vec<Value>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum StringType {
  DoubleQuoted,
  SingleQuoted,
  Raw,
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
    match self {
      Self::Void => true,
      _ => false,
    }
  }

  pub fn is_string(&self) -> bool {
    match self {
      Self::String(..) => true,
      _ => false,
    }
  }

  pub fn as_string_ref(&self) -> Option<&String> {
    match self {
      Self::String(s, _) => Some(s),
      _ => None,
    }
  }

  pub fn as_string_mut(&mut self) -> Option<&mut String> {
    match self {
      Self::String(s, _) => Some(s),
      _ => None,
    }
  }

  pub fn as_string(self) -> Option<String> {
    match self {
      Self::String(s, _) => Some(s),
      _ => None,
    }
  }
}
