use std::{error::Error, fmt::Display};

use crate::parse::span::Span;

#[derive(Debug)]
pub enum LexerError<'a> {
  UnexpectedLiteral { literal: &'a str, span: Span },
  NonNumberAfterDot { span: Span },
  UnexpectedInterruption,
  UnexpectedNewline { span: Span },
  IncompleteEscape { span: Span },
  UndefinedEscape { span: Span },
}

impl<'a> Display for LexerError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::UnexpectedLiteral { literal, span } => {
        write!(
          f,
          "Unexpected literal {literal} occurred in line {line}, column {column}",
          line = span.start.0,
          column = span.start.1
        )
      },
      Self::NonNumberAfterDot { span } => {
        write!(
          f,
          "Non number after a dot occurred in line {line}, column {column}",
          line = span.start.0,
          column = span.start.1
        )
      },
      Self::UnexpectedInterruption => {
        write!(f, "Unexpected interruption occurred")
      },
      Self::UnexpectedNewline { span } => {
        write!(
          f,
          "Unexpected newline occurred in line {line}, column {column}",
          line = span.start.0,
          column = span.start.1
        )
      },
      Self::IncompleteEscape { span } => {
        write!(
          f,
          "Incomplete escape occurred in line {line}, column {column}",
          line = span.start.0,
          column = span.start.1
        )
      },
      Self::UndefinedEscape { span } => {
        write!(
          f,
          "Undefined escape occurred in line {line}, column {column}",
          line = span.start.0,
          column = span.start.1
        )
      },
    }
  }
}

impl<'a> Error for LexerError<'a>{}

#[derive(Debug)]
pub enum ParserError<'a> {
  LexingError(LexerError<'a>)
}

impl<'a> Display for ParserError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::LexingError(e) => write!(f, "{e}"),
    }
  }
}

impl<'a> Error for ParserError<'a>{}