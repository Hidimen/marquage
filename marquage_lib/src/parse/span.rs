use crate::parse::position::Position;

#[derive(Debug, Clone, Copy)]
pub struct Span {
  pub(crate) start: Position,
  pub(crate) end: Position,
  // For offset, it is a left-closed and right-open interval.
  pub(crate) offset: (usize, usize),
}

impl Span {
  pub fn new(start: Position, end: Position, offset: (usize, usize)) -> Self {
    assert!(start <= end, "start is bigger than end");
    assert!(offset.0 <= offset.1, "start offset is bigger than end offset");
    Self { start, end, offset }
  }

  fn merge_offset(&self, other: &Self) -> Option<(usize, usize)> {
    if self.offset.1 >= other.offset.0 && other.offset.1 >= self.offset.0 {
      Some((
        self.offset.0.min(other.offset.0),
        self.offset.1.max(other.offset.1),
      ))
    } else {
      None
    }
  }

  pub fn is_same_line(&self, other: &Self) -> bool {
    self.end.0 == other.end.0
  }

  pub fn combine(&self, other: &Self) -> Option<Self> {
    let new_offset = self.merge_offset(other);
    new_offset.map(|offset| Self {
      start: self.start.min(other.start),
      end: self.end.max(other.end),
      offset,
    })
  }
}
