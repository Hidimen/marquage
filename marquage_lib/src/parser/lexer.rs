use crate::parser::{error::LexerError, literal::Literal, position::Position, span::Span, str_cursor::StrCursor, token::Token};

pub struct Lexer<'lex> {
  cursor: StrCursor<'lex>,
  pos: Position,
  offset: usize,
  checkpoint: Option<usize>,
}

impl<'lex> Lexer<'lex> {
  pub fn new<'raw>(raw: &'raw str) -> Self where 'raw: 'lex {
    Self {
      cursor: StrCursor::new(raw),
      pos: Position(1, 1),
      offset: 0,
      checkpoint: None
    }
  }

  pub fn lex(&mut self) -> Result<Vec<Literal<'_>>, LexerError<'_>> {
    let vec = Vec::new();
    while let Some((s, offsets)) = self.skipping_advance(){
      let legacy_pos = self.pos.increase_column_by(1);
      match s {
        "{" => self.create_token(Literal::OpenBrace, legacy_pos, self.pos, offsets),
        "}" => self.create_token(Literal::CloseBrace, legacy_pos, self.pos, offsets),
        "[" => self.create_token(Literal::OpenBracket, legacy_pos, self.pos, offsets),
        "]" => self.create_token(Literal::CloseBracket, legacy_pos, self.pos, offsets),
        "(" => self.create_token(Literal::OpenParen, legacy_pos, self.pos, offsets),
        ")" => self.create_token(Literal::CloseParen, legacy_pos, self.pos, offsets),
        ";" => self.create_token(Literal::Semicolon, legacy_pos, self.pos, offsets),
        "," => self.create_token(Literal::Comma, legacy_pos, self.pos, offsets),
        "@" => self.create_token(Literal::At, legacy_pos, self.pos, offsets),
        other if self.is_digital(other) => todo!(),
        other => todo!()
      };
    }

    Ok(vec)
  }

  pub fn create_token(&self, literal: Literal<'lex>, start: Position, end: Position, offsets: (usize, usize)) -> Token<'lex> {
    Token::new(literal, Span::new(start, end, offsets))
  }

  pub fn handle_string(&mut self) -> Result<Literal, LexerError> {
    todo!()
  }

  #[inline]
  pub fn advance(&mut self) -> Option<(&'lex str, (usize, usize))> {
    self.cursor.advance_with_offsets()
  }

  pub fn skipping_advance(&mut self) -> Option<(&'lex str, (usize, usize))>{
    while let Some((s, offsets)) = self.cursor.advance_with_offsets() {
      if s == " " || s == "\t" || s == "\r" {
        continue;
      }else{
        return Some((s, offsets))
      }
    }

    None
  }

  fn is_digital(&self, s:&'lex str) -> bool {
    s == "0" || s == "1" || s == "2" || s == "3" || s == "4" ||
    s == "5" || s == "6" || s == "7" || s == "8" || s == "9" 
  }
}