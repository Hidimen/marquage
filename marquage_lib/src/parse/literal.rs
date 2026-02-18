use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal<'a> {
  Void,

  RawString(&'a str),
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

  Comment(&'a str),

  End,
}

impl<'a> Literal<'a> {
  pub fn is_end(&self) -> bool {
    match self {
      Self::End => true,
      _ => false,
    }
  }

  pub fn is_raw_string(&self) -> bool {
    match self {
      Self::RawString(..) => true,
      _ => false,
    }
  }

  pub fn get_raw_string_content(&self) -> Option<String> {
    if let Self::RawString(content) = self {
      Some(content.to_string())
    } else {
      None
    }
  }

  pub fn is_quoted_string(&self) -> bool {
    match self {
      Literal::QuotedString(..) => true,
      _ => false,
    }
  }

  pub fn get_quoted_string_content(&self) -> Option<String> {
    if let Self::QuotedString(content) = self {
      Some(content.to_string())
    } else {
      None
    }
  }

  pub fn is_unsigned_int(&self) -> bool {
    match self {
      Self::UnsignedIntegerNumber(_) => true,
      _ => false,
    }
  }

  pub fn is_signed_int(&self) -> bool {
    match self {
      Self::SignedIntegerNumber(_) => true,
      _ => false,
    }
  }

  pub fn is_float(&self) -> bool {
    match self {
      Self::FloatNumber(_) => true,
      _ => false,
    }
  }

  pub fn is_boolean(&self) -> bool {
    match self {
      Self::Boolean(_) => true,
      _ => false,
    }
  }

  pub fn is_void(&self) -> bool {
    match self {
      Self::Void => true,
      _ => false,
    }
  }

  pub fn is_semicolon(&self) -> bool {
    match self {
      Self::Semicolon => true,
      _ => false,
    }
  }

  pub fn is_equal(&self) -> bool {
    match self {
      Self::Equal => true,
      _ => false,
    }
  }

  pub fn is_open_brace(&self) -> bool {
    match self {
      Self::OpenBrace => true,
      _ => false,
    }
  }

  pub fn is_close_brace(&self) -> bool {
    match self {
      Self::CloseBrace => true,
      _ => false,
    }
  }

  pub fn is_open_bracket(&self) -> bool {
    match self {
      Self::OpenBracket => true,
      _ => false,
    }
  }

  pub fn is_close_bracket(&self) -> bool {
    match self {
      Self::CloseBracket => true,
      _ => false,
    }
  }

  pub fn is_open_paren(&self) -> bool {
    match self {
      Self::OpenParen => true,
      _ => false,
    }
  }

  pub fn is_close_paren(&self) -> bool {
    match self {
      Self::CloseParen => true,
      _ => false,
    }
  }

  pub fn is_comma(&self) -> bool {
    match self {
      Self::Comma => true,
      _ => false,
    }
  }

  pub fn is_at(&self) -> bool {
    match self {
      Self::At => true,
      _ => false,
    }
  }

  pub fn is_comment(&self) -> bool {
    match self {
      Self::Comment(_) => true,
      _ => false,
    }
  }
}

impl<'a> Display for Literal<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::At => write!(f, "@"),
      Self::Boolean(b) => write!(f, "{b}"),
      Self::CloseBrace => write!(f, "}}"),
      Self::CloseBracket => write!(f, "]"),
      Self::CloseParen => write!(f, ")"),
      Self::Comma => write!(f, ","),
      Self::Comment(c) => write!(f, "{c}"),
      Self::End => write!(f, "<End of line>"),
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
