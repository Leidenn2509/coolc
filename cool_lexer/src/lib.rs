#![feature(proc_macro_hygiene)]
extern crate plex;

use std::fmt::{Debug, Formatter};
use std::ops::Range;
use cool_common::{Token, Lexeme};
use plex::lexer;


lexer! {
    fn next_token(text: 'a) -> Token;

    r#"class"# => Token::Class,
    r#"if"# => Token::If,
    r#"then"# => Token::Then,
    r#"else"# => Token::Else,
    r#"fi"# => Token::Fi,
    r#"inherits"# => Token::Inherits,
    r#"in"# => Token::In,
    r#"isvoid"# => Token::IsVoid,
    r#"let"# => Token::Let,
    r#"Loop"# => Token::Loop,
    r#"Pool"# => Token::Pool,
    r#"while"# => Token::While,
    r#"case"# => Token::Case,
    r#"esac"# => Token::Esac,
    r#"new"# => Token::New,
    r#"of"# => Token::Of,
    r#"not"# => Token::Not,

    r#"[A-Z][a-zA-Z0-9_]*"# => Token::TypeID,
    // r#""# => Token::ObjectID,
    // r#""# => Token::IntConst,
    // r#""([^"\\]|\\[\s\S])*""# => Token::StrConst,
    r#""([^"\\]|\\.)*""# => Token::StrConst,

    r#"\("# => Token::OpenRoundBrace,
    r#"\)"# => Token::CloseRoundBrace,
    r#"\{"# => Token::OpenCurlyBrace,
    r#"\}"# => Token::CloseCurlyBrace,
    r#":"# => Token::Colon,
    r#";"# => Token::Semicolon,
    r#"/."# => Token::Dot,
    r#","# => Token::Comma,
    r#"="# => Token::Assign,
    r#"=="# => Token::Equal,
    r#"\+"# => Token::Plus,
    r#"-"# => Token::Minus,
    r#"\*"# => Token::Multiply,
    r#"/"# => Token::Divide,
    r#"<"# => Token::Less,
    r#"<="# => Token::LessOrEqual,
    r#"<-"# => Token::Darrow,
    r#"@"# => Token::AtSign,
    r#"\~"# => Token::Tilde,

    r#"[ \t\n\f]+"# => Token::Error,
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, String, Span);
    fn next(&mut self) -> Option<(Token, String, Span)> {
        loop {
            let (tok, str, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, String::from(&self.original[lo..hi]), Span { lo, hi })
            } else {
                return None;
            };
            match tok {
                // Token::Whitespace | Token::Comment => {
                //     continue;
                // }
                Token::Error => {
                    continue;
                }
                tok => {
                    return Some((tok, str, span));
                }
            }
        }
    }
}


fn line_numbers(str: &mut String) -> Vec<Range<usize>> {
    let mut vec: Vec<usize> = str.match_indices("\n").map(|(n, s)| n).collect();
    vec.insert(0, 0);
    let mut res = vec![];
    for i in 0..vec.len() {
        if i == vec.len() - 1 {
            res.push(Range { start: vec[i], end: str.len() });
            continue;
        }
        res.push(Range { start: vec[i], end: vec[i + 1] });
    }
    res
}

fn find_line_number(start: &usize, table: &Vec<Range<usize>>) -> Option<usize> {
    for (n, range) in table.iter().enumerate() {
        if range.contains(&start) {
            return Some(n);
        }
    }
    None
}

pub fn lex(source: &str) -> Vec<Lexeme> {
    let mut s = String::from(source);
    let numbers = line_numbers(&mut s);
    let lexer = Lexer::new(&s);
    lexer.map(|(token, str, span)|
        Lexeme(find_line_number(&span.lo, &numbers).unwrap(), token.clone(), str)
    ).collect()
}