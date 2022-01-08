use logos::Logos;

#[derive(Logos, Debug, Clone)]
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
    #[regex(r"^[A-Z][a-zA-Z0-9_]*")]
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
    // Colon,
    // Semicolon,
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
    // Error
}

#[derive(Clone)]
pub struct Lexeme(pub usize, pub Token, pub String);

impl Debug for Lexeme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {:?}, {:?}>", self.0, self.1, self.2)
    }
}

pub fn lex(source: String) -> Vec<Lexeme> {

}