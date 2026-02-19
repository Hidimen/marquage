use crate::data::Value;

/// Index data from object or array
pub trait Index {
  fn index_into<'a>(&self, val: &'a Value) -> Option<&'a Value>;
  fn index_into_mut<'a>(&self, val: &'a mut Value) -> Option<&'a mut Value>;
}

impl Index for str {
  fn index_into<'a>(&self, val: &'a Value) -> Option<&'a Value> {
    match val {
      Value::Object(obj) => obj.get(self),
      _ => None,
    }
  }

  fn index_into_mut<'a>(&self, val: &'a mut Value) -> Option<&'a mut Value> {
    match val {
      Value::Object(obj) => obj.get_mut(self),
      _ => None,
    }
  }
}

impl Index for String {
  fn index_into<'a>(&self, val: &'a Value) -> Option<&'a Value> {
    match val {
      Value::Object(obj) => obj.get(self),
      _ => None,
    }
  }

  fn index_into_mut<'a>(&self, val: &'a mut Value) -> Option<&'a mut Value> {
    match val {
      Value::Object(obj) => obj.get_mut(self),
      _ => None,
    }
  }
}

impl Index for usize {
  fn index_into<'a>(&self, val: &'a Value) -> Option<&'a Value> {
    match val {
      Value::Array(arr) => arr.get(*self),
      _ => None,
    }
  }

  fn index_into_mut<'a>(&self, val: &'a mut Value) -> Option<&'a mut Value> {
    match val {
      Value::Array(arr) => arr.get_mut(*self),
      _ => None,
    }
  }
}

impl<T> Index for &T
where
  T: ?Sized + Index,
{
  fn index_into<'a>(&self, val: &'a Value) -> Option<&'a Value> {
    (**self).index_into(val)
  }

  fn index_into_mut<'a>(&self, val: &'a mut Value) -> Option<&'a mut Value> {
    (**self).index_into_mut(val)
  }
}
