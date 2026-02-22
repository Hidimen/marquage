use crate::data::Value;

/// Base of every generator.
pub trait Generator {
  /// Generate a string.
  ///
  /// # Returns
  /// Content is determined on exact map data.
  fn generate(self, v: indexmap::IndexMap<String, Value>) -> String;

  /// Write bytes.
  fn write(&mut self, data: &[u8]);
  /// Write one byte.
  fn write_byte(&mut self, one: u8);
  /// Write spaces.
  fn write_space(&mut self, repeat: usize);

  /// Write an array.
  fn write_array(&mut self, v: Vec<Value>, layer: usize);
  /// Write an object.
  fn write_object(
    &mut self, v: indexmap::IndexMap<String, Value>, layer: usize,
  );
  /// Write a raw string.
  fn write_raw_string(&mut self, v: String);
  /// Write a quoted string.
  fn write_quoted_string(&mut self, v: String);
  /// Write an unsigned number.
  fn write_unsigned_integer(&mut self, v: u32);
  /// Write a signed number.
  fn write_signed_integer(&mut self, v: i32);
  /// Write a float number.
  fn write_float(&mut self, v: f32);
  /// Write a boolean.
  fn write_bool(&mut self, v: bool);
  /// Write void.
  fn write_void(&mut self);
}
