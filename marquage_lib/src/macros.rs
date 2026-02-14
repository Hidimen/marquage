#[macro_export]
/// Help create Value instance quickly
///
/// # Example
/// ```no_run
/// config! {
///   name "Marquage";
///   license "MIT";
/// }
/// ```
macro_rules! config {
  ($($item:ident $content:tt;)*) => {
    $crate::value::Value::Object({
      let mut map = $crate::map::ObjectImpl::new();
      $(map.insert(stringify!($item).to_string(), config_internal!(@value $content));)*
      map
    })
  }
}

#[macro_export]
#[doc(hidden)]
macro_rules! config_internal {
  (@value ()) => {
    $crate::value::Value::Void
  };
  (@value None) => {
    $crate::value::Value::Void
  };
  (@value $val:literal) => {
    $val.into()
  };
  (@value { $( $key:ident $value: tt ;)* }) => {
    $crate::value::Value::Object({
      let mut map = $crate::map::ObjectImpl::new();
      $(map.insert(stringify!($key).to_string(), config_internal!(@value $value));)*
      map
    })
  };
  (@value [ $($element:tt),*$(,)? ]) => {
    $crate::value::Value::Array({
      vec![$(config_internal!(@value $element),)*]
    })
  }
}
