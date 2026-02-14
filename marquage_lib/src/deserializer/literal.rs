use crate::value::{StringType, Value};

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
  Void,

  String(String, StringType),

  Boolean(bool),

  FloatNumber(f32),
  UnsignedIntegerNumber(u32),
  SignedIntegerNumber(i32),

  OpenBrace,
  CloseBrace,

  OpenParen,
  CloseParen,

  OpenBracket,
  CloseBracket,

  Semicolon,
  Comma,

  Reference(String),
  Dereference(String),

  Comment,

  End,
}

impl Literal {
  pub fn is_end(&self) -> bool {
    match self {
      Literal::End => true,
      _ => false,
    }
  }

  pub fn is_string(&self) -> bool {
    match self {
      Literal::String(..) => true,
      _ => false,
    }
  }

  pub fn get_string_content(&self) -> Option<String> {
    if let Self::String(content, _) = self { Some(content.to_string()) } else { None }
  }

  pub fn is_unsigned_int(&self) -> bool {
    match self {
      Literal::UnsignedIntegerNumber(_) => true,
      _ => false,
    }
  }

  pub fn is_signed_int(&self) -> bool {
    match self {
      Literal::SignedIntegerNumber(_) => true,
      _ => false,
    }
  }

  pub fn is_float(&self) -> bool {
    match self {
      Literal::FloatNumber(_) => true,
      _ => false,
    }
  }

  pub fn is_boolean(&self) -> bool {
    match self {
      Literal::Boolean(_) => true,
      _ => false,
    }
  }

  pub fn is_void(&self) -> bool {
    match self {
      Literal::Void => true,
      _ => false,
    }
  }

  pub fn is_reference(&self) -> bool {
    match self {
      Literal::Reference(_) => true,
      _ => false,
    }
  }

  pub fn get_reference_content(&self) -> Option<String> {
    if let Self::Reference(content) = self { Some(content.to_string()) } else { None }
  }

  pub fn is_dereference(&self) -> bool {
    match self {
      Literal::Dereference(_) => true,
      _ => false,
    }
  }

  pub fn get_dereference_content(&self) -> Option<String> {
    if let Self::Dereference(content) = self { Some(content.to_string()) } else { None }
  }

  pub fn is_semicolon(&self) -> bool {
    match self {
      Literal::Semicolon => true,
      _ => false,
    }
  }

  pub fn is_open_brace(&self) -> bool {
    match self {
      Literal::OpenBrace => true,
      _ => false,
    }
  }

  pub fn is_close_brace(&self) -> bool {
    match self {
      Literal::CloseBrace => true,
      _ => false,
    }
  }

  pub fn is_open_bracket(&self) -> bool {
    match self {
      Literal::OpenBracket => true,
      _ => false,
    }
  }

  pub fn is_close_bracket(&self) -> bool {
    match self {
      Literal::CloseBracket => true,
      _ => false,
    }
  }

  pub fn is_open_paren(&self) -> bool {
    match self {
      Literal::OpenParen => true,
      _ => false,
    }
  }

  pub fn is_close_paren(&self) -> bool {
    match self {
      Literal::CloseParen => true,
      _ => false,
    }
  }

  pub fn is_comma(&self) -> bool {
    match self {
      Literal::Comma => true,
      _ => false,
    }
  }

  pub fn is_comment(&self) -> bool {
    match self {
      Literal::Comment => true,
      _ => false,
    }
  }
}

impl Into<Value> for Literal {
  fn into(self) -> Value {
    match self {
      Self::String(content, ty) => Value::String(content, ty),
      Self::UnsignedIntegerNumber(content) => Value::UnsignedIntegerNumber(content),
      Self::SignedIntegerNumber(content) => Value::SignedIntegerNumber(content),
      Self::FloatNumber(content) => Value::FloatNumber(content),
      Self::Boolean(content) => Value::Boolean(content),
      Self::Void => Value::Void,
      _ => Value::String(String::from(""), StringType::DoubleQuoted),
    }
  }
}
