#[derive(Debug, Clone, Copy)]
pub struct Position(usize, usize);

impl Position {
  pub fn new(line: usize, column: usize) -> Self {
    Self(line, column)
  }

  #[inline(always)]
  pub fn add_line_by(&mut self, count: usize) {
    self.0 += count;
  }

  #[inline(always)]
  pub fn set_line_to(&mut self, count: usize) {
    self.0 = count;
  }

  #[inline(always)]
  pub fn add_column_by(&mut self, count: usize) {
    self.1 += count;
  }

  #[inline(always)]
  pub fn subtract_column_by(&mut self, count: usize) {
    self.1 -= count;
  }

  #[inline(always)]
  pub fn set_column_to(&mut self, count: usize) {
    self.1 = count;
  }

  #[inline(always)]
  pub fn get_line(&self) -> usize {
    self.0
  }

  #[inline(always)]
  pub fn get_column(&self) -> usize {
    self.0
  }

  #[inline(always)]
  pub fn is_same_line(&self, line: usize) -> bool {
    self.0 == line
  }
}

impl Into<(usize, usize)> for Position {
  fn into(self) -> (usize, usize) {
    (self.0, self.1)
  }
}

impl From<(usize, usize)> for Position {
  fn from(value: (usize, usize)) -> Self {
    Self(value.0, value.1)
  }
}
