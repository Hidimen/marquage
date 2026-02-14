use std::num::NonZeroUsize;

use crate::{
  map::{ArrayImpl, ObjectImpl},
  serializer::formatter::NativeFormatter,
  value::{StringType, Value},
};

use super::{error::SerializerError, formatter::Formatter};

/// Tool struct helping stringify data
///
/// # Example
///
pub struct Serializer {
  buffer: Vec<u8>,
}

impl Serializer {
  pub fn new() -> Self {
    Self { buffer: Vec::new() }
  }

  pub fn stringify(
    mut self, data: Value, formatter: Option<Box<dyn Formatter>>,
  ) -> Result<String, SerializerError> {
    let format = match formatter {
      Some(f) => f,
      None => Box::new(NativeFormatter::new(2, true, true, NonZeroUsize::new(1).unwrap(), 1)),
    };
    match data {
      Value::Object(obj) => {
        if let Err(e) = self.stringify_object(obj, 0, &format) {
          return Err(e);
        }
        unsafe { Ok(String::from_utf8_unchecked(self.buffer)) }
      },
      _ => Err(SerializerError::UnsupportedValueType(data)),
    }
  }

  fn stringify_object(
    &mut self, obj: ObjectImpl, layer: usize, formatter: &Box<dyn Formatter>,
  ) -> Result<(), SerializerError> {
    let mut iter = obj.into_iter().peekable();
    while let Some((k, v)) = iter.next() {
      //indentation
      self.try_write_raw(formatter.write_object_indentation(layer));
      //key
      self.write_raw_string(k);
      self.write_raw(&formatter.write_object_separator());
      //value
      match v {
        Value::String(data, ty) => {
          match ty {
            StringType::DoubleQuoted => self.write_double_quoted_string(data),
            StringType::SingleQuoted => self.write_single_quoted_string(data),
            StringType::Raw => self.write_raw_string(data),
          }
          self.write_semicolon();
        },
        Value::Boolean(data) => {
          self.write_boolean(data);
          self.write_semicolon();
        },
        Value::UnsignedIntegerNumber(data) => {
          self.write_unsigned_number(data);
          self.write_semicolon();
        },
        Value::SignedIntegerNumber(data) => {
          self.write_signed_number(data);
          self.write_semicolon();
        },
        Value::FloatNumber(data) => {
          self.write_float_number(data);
          self.write_semicolon();
        },
        Value::Void => {
          self.write_void();
          self.write_semicolon();
        },
        Value::Array(arr) => {
          self.buffer.push(b'[');
          self.try_write_newline(formatter.write_newline_in_array());
          if let Err(e) = self.stringify_array(arr, layer + 1, formatter) {
            return Err(e);
          }
          self.try_write_raw(formatter.write_array_separator());
          self.try_write_raw(formatter.write_object_indentation(layer));
          self.buffer.push(b']');
          self.write_semicolon();
        },
        Value::Object(o) => {
          self.buffer.push(b'{');
          self.try_write_newline(formatter.write_newline_in_object());
          if let Err(e) = self.stringify_object(o, layer + 1, formatter) {
            return Err(e);
          }
          self.try_write_raw(formatter.write_object_indentation(layer));
          self.buffer.push(b'}');
        },
      }
      self.try_write_newline(formatter.write_newline_in_object());
    }

    Ok(())
  }

  fn stringify_array(
    &mut self, arr: ArrayImpl, layer: usize, formatter: &Box<dyn Formatter>,
  ) -> Result<(), SerializerError> {
    let mut iter = arr.into_iter().peekable();
    self.try_write_raw(formatter.write_array_separator());
    while let Some(v) = iter.next() {
      self.try_write_raw(formatter.write_array_indentation(layer));
      match v {
        Value::String(data, ty) => match ty {
          StringType::DoubleQuoted => self.write_double_quoted_string(data),
          StringType::SingleQuoted => self.write_single_quoted_string(data),
          StringType::Raw => self.write_raw_string(data),
        },
        Value::Boolean(data) => {
          self.write_boolean(data);
        },
        Value::UnsignedIntegerNumber(data) => {
          self.write_unsigned_number(data);
        },
        Value::SignedIntegerNumber(data) => {
          self.write_signed_number(data);
        },
        Value::FloatNumber(data) => {
          self.write_float_number(data);
        },
        Value::Void => {
          self.write_void();
        },
        Value::Array(arr) => {
          self.buffer.push(b'[');
          self.try_write_newline(formatter.write_newline_in_array());
          if let Err(e) = self.stringify_array(arr, layer + 1, formatter) {
            return Err(e);
          }
          self.try_write_raw(formatter.write_array_indentation(layer));
          self.buffer.push(b']');
        },
        Value::Object(o) => {
          self.buffer.push(b'{');
          self.try_write_newline(formatter.write_newline_in_object());
          if let Err(e) = self.stringify_object(o, layer + 1, formatter) {
            return Err(e);
          }
          self.try_write_raw(formatter.write_object_indentation(layer));
          self.buffer.push(b'}');
        },
      }
      if iter.peek().is_some() {
        self.write_comma();
      }
      self.try_write_newline(formatter.write_newline_in_array());
      self.try_write_raw(formatter.write_array_separator());
    }
    Ok(())
  }

  fn write_semicolon(&mut self) {
    self.buffer.push(b';');
  }

  fn write_comma(&mut self) {
    self.buffer.push(b',');
  }

  fn try_write_newline(&mut self, data: Option<u8>) {
    if data.is_some() {
      self.buffer.push(data.unwrap());
    }
  }

  fn write_raw(&mut self, raw: &[u8]) {
    self.buffer.extend_from_slice(raw);
  }

  fn try_write_raw(&mut self, raw: Option<Vec<u8>>) {
    if raw.is_some() {
      self.buffer.extend_from_slice(&raw.unwrap());
    }
  }

  fn write_raw_string(&mut self, data: String) {
    self.buffer.extend_from_slice(data.as_bytes());
  }

  fn write_double_quoted_string(&mut self, data: String) {
    self.buffer.push(b'"');
    for c in data.chars() {
      match c {
        '"' => self.buffer.extend_from_slice(b"\\\""),
        '\\' => self.buffer.extend_from_slice(b"\\\\"),
        '\n' => self.buffer.extend_from_slice(b"\\n"),
        '\r' => self.buffer.extend_from_slice(b"\\r"),
        _ => self.buffer.push(c as u8),
      }
    }
    self.buffer.push(b'"');
  }

  fn write_single_quoted_string(&mut self, data: String) {
    self.buffer.push(b'\'');
    for c in data.chars() {
      match c {
        '\'' => self.buffer.extend_from_slice(b"\\\'"),
        '\\' => self.buffer.extend_from_slice(b"\\\\"),
        '\n' => self.buffer.extend_from_slice(b"\\n"),
        '\r' => self.buffer.extend_from_slice(b"\\r"),
        _ => self.buffer.push(c as u8),
      }
    }
    self.buffer.push(b'\'');
  }

  fn write_boolean(&mut self, data: bool) {
    self.write_raw(data.to_string().as_bytes());
  }

  fn write_unsigned_number(&mut self, data: u32) {
    self.write_raw(data.to_string().as_bytes());
  }

  fn write_signed_number(&mut self, data: i32) {
    self.write_raw(data.to_string().as_bytes());
  }

  fn write_float_number(&mut self, data: f32) {
    self.write_raw(data.to_string().as_bytes());
  }

  fn write_void(&mut self) {
    self.write_raw("void".as_bytes());
  }
}
