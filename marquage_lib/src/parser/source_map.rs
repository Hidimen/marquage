use crate::parser::span::Span;

#[derive(Debug)]
pub struct SourceMap<'a> {
  code: &'a str,
}

impl<'a> SourceMap<'a> {
  pub fn new(code: &'a str) -> Self {
    Self { code }
  }

  pub fn get_snippet_from_span(&self, span: &Span) -> Option<&'a str> {
    self.code.get(span.offsets.0..span.offsets.1)
  }
}
