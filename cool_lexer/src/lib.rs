use std::fmt::{Debug, Formatter};
use logos::Logos;
use serde::{Serialize, Deserialize};


#[derive(Logos, Serialize, Deserialize, Debug, Clone)]
pub enum Token {
    #[token("class")]
    Class,

    #[token("if")]
    If,

    #[token("then")]
    Then,

    #[token("else")]
    Else,

    #[token("fi")]
    Fi,

    #[token("inherits")]
    Inherits,

    #[token("in")]
    In,

    #[token("isvoid")]
    IsVoid,

    #[token("let")]
    Let,

    #[token("loop")]
    Loop,

    #[token("pool")]
    Pool,

    #[token("while")]
    While,

    #[token("case")]
    Case,

    #[token("esac")]
    Esac,

    #[token("new")]
    New,

    #[token("of")]
    Of,

    #[token("not")]
    Not,
    //
    // BoolConst,
    #[regex("[A-Z][a-zA-Z0-9_]*")]
    TypeID,
    // ObjectID,
    // IntConst,
    // StrConst,
    //
    // OpenRoundBrace,
    // CloseRoundBrace,

    #[token("{")]
    OpenCurlyBrace,

    #[token("}")]
    CloseCurlyBrace,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,
    // Dot,
    // Comma,
    // Assign,
    // Equal,
    // Plus,
    // Minus,
    // Multiply,
    // Divide,
    // Less,
    // LessOrEqual,
    // Darrow,
    // AtSign,
    // Tilde,
    //
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Lexeme(pub usize, pub Token, pub String);

impl Debug for Lexeme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {:?}, {:?}>", self.0, self.1, self.2)
    }
}

pub fn lex(source: &str) -> Vec<Lexeme> {
    let mut lex = Token::lexer(source);
    let mut lexemes: Vec<Lexeme> = vec![];
    while let Some(token) = lex.next() {
        let span = lex.span();
        let slice = lex.slice();
        let n = source[..span.start].matches("\n").count() + 1;
        lexemes.push(Lexeme(n, token, String::from(slice)))
    }

    println!("{}", Token::Class.serialize());

    lexemes
}