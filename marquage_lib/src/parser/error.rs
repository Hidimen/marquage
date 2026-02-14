use std::fmt::Display;

use crate::parser::{source_map::SourceMap, span::Span};

macro_rules! display_error {
  ($f:ident, $err_msg: literal, $help: expr, $span: expr, $source_map: expr) => {{
    let snippet = $source_map.get_snippet_from_span($span);
    if let Some(c) = snippet {
      if let Some(help) = $help {
        write!(
          $f,
          r###"error: {err_msg}
>>> {code}
|   {underline}
|
| help: {help}"###,
          err_msg = $err_msg,
          code = c,
          underline = "^".repeat(c.chars().count()),
        )
      } else {
        write!(
          $f,
          r###"error: {err_msg}
>>> {code}
|   {underline}"###,
          err_msg = $err_msg,
          code = c,
          underline = "^".repeat(c.chars().count())
        )
      }
    } else {
      write!($f, "error: {}", $err_msg)
    }
  }};
}

#[derive(Debug)]
pub enum LexerError<'a> {
  UnexpectedLiteral { help: Option<String>, span: Span, source_map: SourceMap<'a> },
}

impl<'a> Display for LexerError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::UnexpectedLiteral { help, span, source_map } => {
        display_error!(f, "unexpected literal", help, span, source_map)
      },
    }
  }
}
