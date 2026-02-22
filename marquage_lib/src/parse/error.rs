//! Parse module's error.
use std::{error::Error, fmt::Display};

use crate::parse::{literal::Literal, span::Span};

/// Lexer error.
#[derive(Debug)]
pub enum LexerError {
  /// An unexpected literal detected.
  UnexpectedLiteral { literal: String, span: Span },
  /// Invalid float number format.
  NonNumberAfterDot { span: Span },
  /// Unexpected end.
  UnexpectedInterruption,
  /// Unexpected newline in a string.
  UnexpectedNewline { span: Span },
  /// Broken escape string format.
  IncompleteEscape { span: Span },
  /// Unknown escape.
  UndefinedEscape { span: Span },
}

impl Display for LexerError {
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

impl Error for LexerError {}

/// Parser error
#[derive(Debug)]
pub enum ParserError {
  /// Representing [LexerError].
  LexingError(LexerError),
  /// Expect a key.
  ExpectKey(Literal, Span),
  /// Expect a value.
  ExpectValue(Literal, Span),
  /// Expect a brace.
  ExpectBrace(Literal, Span),
  /// Expect an equal mark.
  ExpectEqual(Literal, Span),
  /// Expect a semicolon.
  ExpectSemicolon(Literal, Span),
  /// Expect a comma or a close bracket.
  ExpectCommaOrCloseBracket(Literal, Span),
  /// Unexpected close brace.
  UnexpectedCloseBrace(Span),
}

impl Display for ParserError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::LexingError(e) => write!(f, "{e}"),
      Self::ExpectKey(l, span) => write!(
        f,
        "Expect a key, but found {} in line {line}, column {column} ",
        l,
        line = span.start.0,
        column = span.start.1
      ),
      Self::ExpectValue(l, span) => write!(
        f,
        "Expect a key, but found {} in line {line}, column {column}",
        l,
        line = span.start.0,
        column = span.start.1
      ),
      Self::ExpectBrace(l, span) => write!(
        f,
        "Expect a close brace, but found {} in line {line}, column {column}",
        l,
        line = span.start.0,
        column = span.start.1
      ),
      Self::ExpectEqual(l, span) => write!(
        f,
        "Expect an equal mark, but found {} in line {line}, column {column}",
        l,
        line = span.start.0,
        column = span.start.1
      ),
      Self::ExpectSemicolon(l, span) => write!(
        f,
        "Expect a semicolon, but found {} in line {line}, column {column}",
        l,
        line = span.start.0,
        column = span.start.1
      ),
      Self::ExpectCommaOrCloseBracket(l, span) => write!(
        f,
        "Expect a comma or close bracket, but found {} in line {line}, column {column}",
        l,
        line = span.start.0,
        column = span.start.1
      ),
      Self::UnexpectedCloseBrace(span) => write!(
        f,
        "Unexpected brace in line {line}, column {column}",
        line = span.start.0,
        column = span.start.1
      ),
    }
  }
}

impl From<LexerError> for ParserError {
  fn from(value: LexerError) -> Self {
    Self::LexingError(value)
  }
}

impl Error for ParserError {}
