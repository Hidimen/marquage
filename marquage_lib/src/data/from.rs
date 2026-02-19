use indexmap::IndexMap;

use crate::data::Value;

macro_rules! impl_from_for_unsigned {
    ($($ty:ident),*) => {
      $(
        impl From<$ty> for Value {
          fn from(value: $ty) -> Self{
            Value::UnsignedIntegerNumber(value as u32)
          }
        }
      )*
    };
}

macro_rules! impl_from_for_signed {
    ($($ty:ident),*) => {
      $(
        impl From<$ty> for Value {
          fn from(value: $ty) -> Self{
            Value::SignedIntegerNumber(value as i32)
          }
        }
      )*
    };
}

impl_from_for_unsigned! {
  u8, u16, u32, u64
}

impl_from_for_signed! {
  i8, i16, i32, i64
}

impl From<bool> for Value {
  fn from(value: bool) -> Self {
    Value::Boolean(value)
  }
}

impl From<String> for Value {
  fn from(value: String) -> Self {
    Value::QuotedString(value)
  }
}

impl<'a> From<&'a str> for Value {
  fn from(value: &'a str) -> Self {
    Value::QuotedString(value.into())
  }
}

impl From<()> for Value {
  fn from(_value: ()) -> Self {
    Value::Void
  }
}

impl<T> From<IndexMap<String, T>> for Value
where
  T: Into<Value>,
{
  fn from(value: IndexMap<String, T>) -> Self {
    Value::Object(value.into_iter().map(|(k, v)| (k, v.into())).collect())
  }
}

impl<T> From<Vec<T>> for Value
where
  T: Into<Value>,
{
  fn from(value: Vec<T>) -> Self {
    Value::Array(value.into_iter().map(|val| val.into()).collect())
  }
}
