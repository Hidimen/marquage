use crate::parser::position::Position;

#[derive(Debug, Clone, Copy)]
pub struct Span {
  pub(crate) start: Position,
  pub(crate) end: Position,
  pub(crate) offsets: (usize, usize),
}

impl Span {
  pub fn new(start: Position, end: Position, offsets: (usize, usize)) -> Self {
    assert!(start <= end, "start position is bigger than end position");
    Self { start, end, offsets  }
  }

  fn merge_offsets(&self, other: &Self) -> Option<(usize, usize)>{
    if self.offsets.1 >= other.offsets.0 && self.offsets.1 >= other.offsets.0 {
      Some((self.offsets.0.min(other.offsets.0), self.offsets.1.max(other.offsets.1)))
    }else{
      None
    }
  }

  pub fn combine(&mut self, other: &Self) -> Option<Self> {
    let new_offsets = self.merge_offsets(other);
    if let Some(offsets) = new_offsets {
      Some(Self {
        start: self.start.min(other.start),
        end: self.end.max(other.end),
        offsets
      })
    }else{
      None
    }
  }
}