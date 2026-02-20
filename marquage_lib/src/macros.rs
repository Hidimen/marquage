#[macro_export]
macro_rules! marquage {
  ($($item:ident = $content:tt;)*) => {
    $crate::data::Value::Object({
      let mut map = indexmap::IndexMap::new();
      $(map.insert(stringify!($item).to_string(), marquage_impl!(@value $content));)*
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
      let map = indexmap::IndexMap::new();
      map
    })
  };
  (@value { $( $key:ident = $value: tt;)* }) => {
    $crate::data::Value::Object({
      let mut map = indexmap::IndexMap::new();
      $(map.insert(stringify!($key).to_string(), marquage_impl!(@value $value));)*
      map
    })
  };
  (@value [];) => {
    $crate::value::Value::Array({
      vec![]
    })
  };
  (@value [ $($element:tt),*$(,)? ]) => {
    $crate::value::Value::Array({
      vec![$(marquage_impl!(@value $element),)*]
    })
  }
}
