use crate::{
  data::Value,
  generate::{Config, Generator},
};

/// A built-in config of [PrettyGenerator].
#[derive(Default)]
pub struct PrettyConfig;

impl Config for PrettyConfig {
  fn equal_space(&self) -> usize {
    1
  }

  fn following_comma(&self) -> bool {
    true
  }

  fn indent(&self) -> usize {
    2
  }

  fn newline_in_array(&self) -> bool {
    false
  }

  fn newline_in_object(&self) -> bool {
    true
  }

  fn array_space(&self) -> usize {
    1
  }

  fn object_space(&self) -> usize {
    0
  }
}

/// A built-in generator that output a pretty format.
pub struct PrettyGenerator {
  config: PrettyConfig,
  data: Vec<u8>,
}

impl PrettyGenerator {
  /// Create a [PrettyGenerator]
  pub fn create(c: PrettyConfig) -> Self {
    Self { config: c, data: Vec::new() }
  }
}

impl Generator for PrettyGenerator {
  fn generate(mut self, v: indexmap::IndexMap<String, Value>) -> String {
    self.write_object(v, 0);
    unsafe { String::from_utf8_unchecked(self.data) }
  }

  fn write(&mut self, data: &[u8]) {
    self.data.extend_from_slice(data);
  }

  fn write_byte(&mut self, one: u8) {
    self.data.push(one);
  }

  fn write_space(&mut self, repeat: usize) {
    for _ in 0..repeat {
      self.write_byte(b' ');
    }
  }

  fn write_array(&mut self, v: Vec<Value>, layer: usize) {
    let len = v.len();
    self.write_byte(b'[');
    if !self.config.newline_in_array() {
      self.write_space(self.config.array_space());
    } else {
      self.write_byte(b'\n');
    }
    for (index, val) in v.into_iter().enumerate() {
      if self.config.newline_in_array() {
        self.write_space(self.config.indent() * layer);
      }
      match val {
        Value::Array(arr) => {
          self.write_array(arr, layer + 1);
        },
        Value::Boolean(b) => self.write_bool(b),
        Value::FloatNumber(f) => self.write_float(f),
        Value::Object(obj) => {
          self.write_object(obj, layer + 1);
        },
        Value::QuotedString(s) => self.write_quoted_string(s),
        Value::RawString(s) => self.write_raw_string(s),
        Value::SignedIntegerNumber(n) => self.write_signed_integer(n),
        Value::UnsignedIntegerNumber(n) => self.write_unsigned_integer(n),
        Value::Void => self.write_void(),
      }

      if index == len - 1 {
        if self.config.following_comma() {
          self.write_byte(b',');
        }
      } else {
        self.write_byte(b',');
        if self.config.newline_in_array() {
          self.write_byte(b'\n');
        } else {
          self.write_space(self.config.array_space());
        }
      }
    }

    if !self.config.newline_in_array() {
      self.write_space(self.config.array_space());
    } else {
      self.write_space(self.config.indent() * (layer - 1));
    }
    self.write_byte(b']');
  }

  fn write_object(
    &mut self, v: indexmap::IndexMap<String, Value>, layer: usize,
  ) {
    if layer != 0 {
      self.write_byte(b'{');
    }
    if self.config.newline_in_object() {
      self.write_byte(b'\n');
    } else {
      self.write_space(self.config.object_space());
    }
    for (key, val) in v {
      if self.config.newline_in_object() {
        self.write_space(self.config.indent() * layer);
      }
      self.write_raw_string(key);
      self.write_space(self.config.equal_space());
      self.write_byte(b'=');
      self.write_space(self.config.equal_space());
      match val {
        Value::Array(arr) => {
          self.write_array(arr, layer + 1);
          self.write_byte(b';');
        },
        Value::Boolean(b) => {
          self.write_bool(b);
          self.write_byte(b';');
        },
        Value::FloatNumber(f) => {
          self.write_float(f);
          self.write_byte(b';');
        },
        Value::Object(obj) => {
          self.write_object(obj, layer + 1);
        },
        Value::QuotedString(s) => {
          self.write_quoted_string(s);
          self.write_byte(b';');
        },
        Value::RawString(s) => {
          self.write_raw_string(s);
          self.write_byte(b';');
        },
        Value::SignedIntegerNumber(n) => {
          self.write_signed_integer(n);
          self.write_byte(b';');
        },
        Value::UnsignedIntegerNumber(n) => {
          self.write_unsigned_integer(n);
          self.write_byte(b';');
        },
        Value::Void => {
          self.write_void();
          self.write_byte(b';');
        },
      }
      if self.config.newline_in_object() {
        self.write_byte(b'\n');
      } else {
        self.write_space(self.config.object_space());
      }
    }

    if layer != 0 {
      if self.config.newline_in_object() {
        self.write_space(self.config.indent() * (layer - 1));
      }
      self.write_byte(b'}');
    }
  }

  fn write_bool(&mut self, v: bool) {
    if v {
      self.write(b"true");
    } else {
      self.write(b"false");
    }
  }

  fn write_float(&mut self, v: f32) {
    self.write(v.to_string().as_bytes());
  }

  fn write_quoted_string(&mut self, v: String) {
    let mut bytes = Vec::with_capacity(v.len() + 2);
    bytes.push(b'"');
    bytes.extend_from_slice(v.as_bytes());
    bytes.push(b'"');
    self.write(&bytes);
  }

  fn write_raw_string(&mut self, v: String) {
    self.write(v.as_bytes());
  }

  fn write_signed_integer(&mut self, v: i32) {
    self.write(v.to_string().as_bytes());
  }

  fn write_unsigned_integer(&mut self, v: u32) {
    self.write(v.to_string().as_bytes());
  }

  fn write_void(&mut self) {
    self.write(b"void");
  }
}
