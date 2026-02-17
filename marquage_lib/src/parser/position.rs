#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Position(pub usize, pub usize); // line, column

impl Position {
  /// Add column counter by given number.
  ///
  /// # Returns
  /// It will return legacy position.
  pub fn add_column_by(&mut self, step: usize) -> Position {
    let cache = *self;
    self.1 += step;
    cache
  }

  /// Add line counter by given number and set column counter to 1.
  ///
  /// # Returns
  /// It will return legacy position.
  pub fn add_line_by(&mut self, step: usize) -> Position {
    let cache = *self;
    self.0 += step;
    self.1 = 1;
    cache
  }

  /// Subtract column counter by given number.
  ///
  /// # Returns
  /// It will return legacy position.
  pub fn subtract_column_by(&mut self, step: usize) -> Position {
    let cache = *self;
    self.1 -= step;
    cache
  }

  /// Subtract line counter by given number and set column counter to 1.
  ///
  /// # Returns
  /// It will return legacy position.
  pub fn subtract_line_by(&mut self, step: usize) -> Position {
    let cache = *self;
    self.0 += step;
    self.1 = 1;
    cache
  }

  pub fn add_column(&mut self) {
    self.1 += 1;
  }
}

impl From<(usize, usize)> for Position {
  fn from(value: (usize, usize)) -> Self {
    Position(value.0, value.1)
  }
}
