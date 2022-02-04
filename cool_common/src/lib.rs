use std::fmt::{Debug, Formatter};
use serde::Serialize;

#[derive(Debug, Clone)]
pub enum Token {
    Class,
    If,
    Then,
    Else,
    Fi,
    Inherits,
    In,
    IsVoid,
    Let,
    Loop,
    Pool,
    While,
    Case,
    Esac,
    New,
    Of,
    Not,

    BoolConst,
    TypeID,
    ObjectID,
    IntConst,
    StrConst,

    OpenRoundBrace,
    CloseRoundBrace,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Colon,

    Semicolon,
    Dot,
    Comma,
    Assign,
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
    Less,
    LessOrEqual,
    Darrow,
    AtSign,
    Tilde,

    Error
}

#[derive(Clone)]
pub struct Lexeme(pub usize, pub Token, pub String);

impl Debug for Lexeme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write!(f, "<{}, {:?}, {:?}>", self.0, self.1, self.2)
        write!(f, "#{}, {:?}, {:?}>", self.0, self.1, self.2)
    }
}

pub struct LexemeSerializer {
    output: String
}

pub fn to_string(value: &Lexeme) -> Result<String> {

}
