#[derive(Debug)]
pub struct SourceMap<'a> {
  code: &'a str,
  offset: usize,
  offsets: Vec<usize>,
}

impl<'a> SourceMap<'a> {
  pub fn new(code: &'a str) -> Self {
    let mut offsets = Vec::with_capacity(code.len() / 2);
    let bytes = code.as_bytes();
    let mut cursor = 0;
    while cursor < code.len() {
      offsets.push(cursor);

      cursor += match bytes[cursor] {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 1,
      }
    }

    offsets.push(code.len());

    Self { code, offset: 0, offsets }
  }

  pub fn advance(
    &mut self,
  ) -> Option<(&'a str, /* represents the offset of character*/ usize)> {
    if self.is_end() {
      return None;
    }

    let start = self.offsets[self.offset];
    self.offset += 1;
    let end = self.offsets[self.offset];

    Some((&self.code[start..end], self.offset - 1))
  }

  pub fn get_by_offset(&self, start: usize, end: usize) -> &'a str {
    assert!(start <= end, "start should be less than or equal to end");
    assert!(end <= self.offsets.len() - 1, "end is out of bounds");

    let start_offset = self.offsets[start];
    let end_offset = self.offsets[end];

    &self.code[start_offset..end_offset]
  }

  pub fn peek(&self) -> Option<&'a str> {
    if self.is_end() {
      return None;
    }

    Some(&self.code[self.offsets[self.offset]..self.offsets[self.offset + 1]])
  }

  pub fn consume(&mut self) {
    if !self.is_end() {
      self.offset += 1;
    }
  }

  pub fn get_offset(&self) -> usize {
    self.offset
  }

  pub fn move_to(&mut self, dest: usize) {
    if dest < self.offsets.len() - 1 {
      self.offset = dest;
    }
  }

  pub fn back(&mut self) {
    if self.offset != 0 {
      self.offset -= 1;
    }
  }

  pub fn is_end(&self) -> bool {
    self.offset >= self.offsets.len() - 1
  }
}
