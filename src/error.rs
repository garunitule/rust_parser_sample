use crate::lexer::Token;
use crate::utility::{Annot, Loc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

pub type LexError = Annot<LexErrorKind>;

impl LexError {
    pub fn invalid_char(c: char, loc: Loc) -> Self {
        Self::new(LexErrorKind::InvalidChar(c), loc)
    }

    pub fn eof(loc: Loc) -> Self {
        Self::new(LexErrorKind::Eof, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    UnexpectedToken(Token),
    NotExpression(Token),
    NotOperator(Token),
    UnclosedOpenParen(Token),
    // 式の解析が終わったのにトークンが残っている
    RedundantExpression(Token),
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    Lexer(LexError),
    Parser(ParseError),
}

impl From<LexError> for Error {
    fn from(e: LexError) -> Self {
        Error::Lexer(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parser(e)
    }
}
