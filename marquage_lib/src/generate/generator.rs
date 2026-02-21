use crate::data::Value;

pub trait Generator {
  fn generate(self, v: indexmap::IndexMap<String, Value>) -> String;

  fn write(&mut self, data: &[u8]);
  fn write_byte(&mut self, one: u8);
  fn write_space(&mut self, repeat: usize);

  fn write_array(&mut self, v: Vec<Value>, layer: usize);
  fn write_object(
    &mut self, v: indexmap::IndexMap<String, Value>, layer: usize,
  );
  fn write_raw_string(&mut self, v: String);
  fn write_quoted_string(&mut self, v: String);
  fn write_unsigned_integer(&mut self, v: u32);
  fn write_signed_integer(&mut self, v: i32);
  fn write_float(&mut self, v: f32);
  fn write_bool(&mut self, v: bool);
  fn write_void(&mut self);
}
