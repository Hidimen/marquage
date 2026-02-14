use std::num::NonZeroUsize;

pub trait Formatter {
  fn write_newline_in_object(&self) -> Option<u8>;
  fn write_newline_in_array(&self) -> Option<u8>;
  fn write_object_indentation(&self, layer: usize) -> Option<Vec<u8>>;
  fn write_array_indentation(&self, layer: usize) -> Option<Vec<u8>>;
  fn write_object_separator(&self) -> Vec<u8>;
  fn write_array_separator(&self) -> Option<Vec<u8>>;
}

pub struct NativeFormatter {
  indentation: usize,
  newline_in_array: bool,
  newline_in_object: bool,
  object_separator: NonZeroUsize,
  array_separator: usize,
}

impl NativeFormatter {
  pub fn new(
    indentation: usize, newline_in_array: bool, newline_in_object: bool,
    object_separator: NonZeroUsize, array_separator: usize,
  ) -> Self {
    Self { indentation, newline_in_array, newline_in_object, object_separator, array_separator }
  }
}

impl Formatter for NativeFormatter {
  fn write_object_indentation(&self, layer: usize) -> Option<Vec<u8>> {
    if self.newline_in_object { Some(vec![b' '; layer * self.indentation]) } else { None }
  }

  fn write_array_indentation(&self, layer: usize) -> Option<Vec<u8>> {
    if self.newline_in_array { Some(vec![b' '; layer * self.indentation]) } else { None }
  }

  fn write_newline_in_array(&self) -> Option<u8> {
    if self.newline_in_array { Some(b'\n') } else { None }
  }

  fn write_newline_in_object(&self) -> Option<u8> {
    if self.newline_in_array { Some(b'\n') } else { None }
  }

  fn write_array_separator(&self) -> Option<Vec<u8>> {
    if !self.newline_in_array { Some(vec![b' '; self.array_separator]) } else { None }
  }

  fn write_object_separator(&self) -> Vec<u8> {
    vec![b' '; self.object_separator.into()]
  }
}
