use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
  Void,

  RawString(String),
  QuotedString(String),

  Boolean(bool),

  FloatNumber(f32),
  UnsignedIntegerNumber(u32),
  SignedIntegerNumber(i32),

  OpenBrace,  // {
  CloseBrace, // }

  OpenParen,  // (
  CloseParen, // )

  OpenBracket,  // [
  CloseBracket, // ]

  Semicolon, // ;
  Comma,     // ,

  At,    // @
  Equal, // =

  Comment(String),

  End,
}

impl Literal {
  pub fn is_end(&self) -> bool {
    matches!(self, Self::End)
  }

  pub fn is_raw_string(&self) -> bool {
    matches!(self, Self::RawString(..))
  }

  pub fn get_string(self) -> Option<String> {
    if let Self::RawString(s) | Self::QuotedString(s) = self {
      Some(s)
    } else {
      None
    }
  }

  pub fn is_quoted_string(&self) -> bool {
    matches!(self, Literal::QuotedString(..))
  }

  pub fn is_unsigned_int(&self) -> bool {
    matches!(self, Self::UnsignedIntegerNumber(_))
  }

  pub fn is_signed_int(&self) -> bool {
    matches!(self, Self::SignedIntegerNumber(_))
  }

  pub fn is_float(&self) -> bool {
    matches!(self, Self::FloatNumber(_))
  }

  pub fn is_boolean(&self) -> bool {
    matches!(self, Self::Boolean(_))
  }

  pub fn is_void(&self) -> bool {
    matches!(self, Self::Void)
  }

  pub fn is_semicolon(&self) -> bool {
    matches!(self, Self::Semicolon)
  }

  pub fn is_equal(&self) -> bool {
    matches!(self, Self::Equal)
  }

  pub fn is_open_brace(&self) -> bool {
    matches!(self, Self::OpenBrace)
  }

  pub fn is_close_brace(&self) -> bool {
    matches!(self, Self::CloseBrace)
  }

  pub fn is_open_bracket(&self) -> bool {
    matches!(self, Self::OpenBracket)
  }

  pub fn is_close_bracket(&self) -> bool {
    matches!(self, Self::CloseBracket)
  }

  pub fn is_open_paren(&self) -> bool {
    matches!(self, Self::OpenParen)
  }

  pub fn is_close_paren(&self) -> bool {
    matches!(self, Self::CloseParen)
  }

  pub fn is_comma(&self) -> bool {
    matches!(self, Self::Comma)
  }

  pub fn is_at(&self) -> bool {
    matches!(self, Self::At)
  }

  pub fn is_comment(&self) -> bool {
    matches!(self, Self::Comment(_))
  }
}

impl Display for Literal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::At => write!(f, "@"),
      Self::Boolean(b) => write!(f, "{b}"),
      Self::CloseBrace => write!(f, "}}"),
      Self::CloseBracket => write!(f, "]"),
      Self::CloseParen => write!(f, ")"),
      Self::Comma => write!(f, ","),
      Self::Comment(c) => write!(f, "{c}"),
      Self::End => write!(f, "<END OF LINE>"),
      Self::Equal => write!(f, "="),
      Self::FloatNumber(n) => write!(f, "{n}"),
      Self::OpenBrace => write!(f, "{{"),
      Self::OpenBracket => write!(f, "["),
      Self::OpenParen => write!(f, "("),
      Self::QuotedString(s) => write!(f, "{s}"),
      Self::RawString(s) => write!(f, "{s}"),
      Self::Semicolon => write!(f, ";"),
      Self::SignedIntegerNumber(n) => write!(f, "{n}"),
      Self::UnsignedIntegerNumber(n) => write!(f, "{n}"),
      Self::Void => write!(f, "void"),
    }
  }
}
