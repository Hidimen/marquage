pub struct StrCursor<'cursor> {
  text: &'cursor str,
  pos: usize,
  offsets: Vec<usize>
}

impl<'cursor> StrCursor<'cursor> {
  pub fn new(raw: &'cursor str) -> Self {
    let mut offsets = Vec::with_capacity(raw.len() / 2);
    let bytes = raw.as_bytes();
    let mut cursor = 0;
    while cursor < raw.len() {
      offsets.push(cursor);

      cursor += match bytes[cursor] {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 1
      }
    }

    offsets.push(raw.len());
    
    Self {
      text: raw,
      pos:0,
      offsets
    }
  }

  #[allow(dead_code)]
  pub fn advance(&mut self) -> Option<&'cursor str> {
    if self.is_end() {
      return None;
    }

    let legacy_pos = self.offsets[self.pos];
    self.pos += 1;
    let next_pos = self.offsets[self.pos];
    
    Some(&self.text[legacy_pos..next_pos])
  }

  pub fn advance_with_offsets(&mut self) -> Option<(&'cursor str, (usize, usize))> {
    if self.is_end() {
      return None;
    }

    let legacy_pos = self.offsets[self.pos];
    self.pos += 1;
    let next_pos = self.offsets[self.pos];
    
    Some((&self.text[legacy_pos..next_pos], (legacy_pos, next_pos)))
  }

  pub fn is_end(&self) -> bool {
    self.pos >= self.offsets.len() - 1
  }
}