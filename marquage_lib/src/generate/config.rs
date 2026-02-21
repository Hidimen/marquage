pub trait Config {
  fn indent(&self) -> usize;
  fn following_comma(&self) -> bool;
  fn newline_in_object(&self) -> bool;
  fn newline_in_array(&self) -> bool;
  fn equal_space(&self) -> usize;
  fn array_space(&self) -> usize;
  fn object_space(&self) -> usize;
}
