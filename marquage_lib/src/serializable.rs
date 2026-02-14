use indexmap::IndexMap;

use crate::value::{StringType, Value};

/// A data type that implements this can be serialized
pub trait Serializable {
  /// Serialize the data itself into [`Value`](crate::value::Value)
  fn serialize(&self) -> Value;
}

macro_rules! impl_unsigned_primitive {
  ($ty:ident) => {
    impl Serializable for $ty {
      fn serialize(&self) -> Value {
        Value::UnsignedIntegerNumber(*self as u32)
      }
    }
  };
}

macro_rules! impl_signed_primitive {
  ($ty:ident) => {
    impl Serializable for $ty {
      fn serialize(&self) -> Value {
        Value::SignedIntegerNumber(*self as i32)
      }
    }
  };
}

impl_unsigned_primitive!(u8);
impl_unsigned_primitive!(u16);
impl_unsigned_primitive!(u32);
impl_unsigned_primitive!(u64);
impl_unsigned_primitive!(usize);
impl_signed_primitive!(i8);
impl_signed_primitive!(i16);
impl_signed_primitive!(i32);
impl_signed_primitive!(i64);
impl_signed_primitive!(isize);

impl Serializable for f32 {
  fn serialize(&self) -> Value {
    Value::FloatNumber(*self)
  }
}

impl Serializable for bool {
  fn serialize(&self) -> Value {
    Value::Boolean(*self)
  }
}

impl Serializable for String {
  fn serialize(&self) -> Value {
    Value::String(self.clone(), StringType::DoubleQuoted)
  }
}

impl<'a> Serializable for &'a str {
  fn serialize(&self) -> Value {
    Value::String(self.to_string(), StringType::DoubleQuoted)
  }
}

impl<T> Serializable for IndexMap<String, T>
where
  T: Serializable,
{
  fn serialize(&self) -> Value {
    Value::Object(self.iter().map(|(k, v)| (k.to_owned(), v.serialize())).collect())
  }
}

impl<T: Serializable> Serializable for Vec<T> {
  fn serialize(&self) -> Value {
    Value::Array(self.iter().map(|k| k.serialize()).collect())
  }
}

impl<T: Serializable> Serializable for Option<T> {
  fn serialize(&self) -> Value {
    if let Some(val) = self { val.serialize() } else { Value::Void }
  }
}

impl Serializable for () {
  fn serialize(&self) -> Value {
    Value::Void
  }
}

impl<T: Serializable> Serializable for Box<T> {
  fn serialize(&self) -> Value {
    self.as_ref().serialize()
  }
}
