/// Config of generators.
pub trait Config {
  /// Get indentation.
  fn indent(&self) -> usize;
  /// Whether enable trailing comma.
  fn following_comma(&self) -> bool;
  /// Whether enable newline in object.
  fn newline_in_object(&self) -> bool;
  /// Whether enable newline in array.
  fn newline_in_array(&self) -> bool;
  /// Amount of space around equal mark.
  fn equal_space(&self) -> usize;
  /// Amount of space in array.
  ///
  /// **Note**: It will not work if `newline_in_array` is true.
  fn array_space(&self) -> usize;
  /// Amount of space in object.
  ///
  /// **Note**: It will not work if `newline_in_object` is true.
  fn object_space(&self) -> usize;
}
