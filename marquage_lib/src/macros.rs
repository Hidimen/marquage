/// Build a value.
///
/// # Example
/// ```rust
/// # use marquage_lib::{marquage, data::Value, Map};
/// let val = Value::Object(Map::from([
///   ("hello".into(), Value::QuotedString("world".into()))
/// ]));
///
/// let v = marquage!{
///   hello = "world";
/// };
/// assert_eq!(v, val);
/// ```
///
/// **Note**: Macro syntax is similar to `marquage` syntax, but semicolon after an object is a must.
#[macro_export]
macro_rules! marquage {
  ($($item:ident = $content:tt;)*) => {
    $crate::data::Value::Object({
      let mut map = $crate::Map::new();
      $(map.insert(stringify!($item).to_string(), $crate::marquage_impl!(@value $content));)*
      map
    })
  }
}

#[macro_export]
#[doc(hidden)]
macro_rules! marquage_impl {
  (@value ()) => {
    $crate::value::Value::Void
  };
  (@value None) => {
    $crate::value::Value::Void
  };
  (@value $val:literal) => {
    $val.into()
  };
  (@value {}) => {
    $crate::data::Value::Object({
      let map = $crate::Map::new();;
      map
    })
  };
  (@value { $( $key:ident = $value: tt;)* }) => {
    $crate::data::Value::Object({
      let mut map = $crate::Map::new();;
      $(map.insert(stringify!($key).to_string(), $crate::marquage_impl!(@value $value));)*
      map
    })
  };
  (@value [];) => {
    $crate::data::Value::Array({
      vec![]
    })
  };
  (@value [ $($element:tt),*$(,)? ]) => {
    $crate::data::Value::Array({
      vec![$($crate::marquage_impl!(@value $element),)*]
    })
  }
}
