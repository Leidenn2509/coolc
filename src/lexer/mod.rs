mod token_matcher;

use std::cmp::min;
use std::io::{BufRead, Read};
use std::str::SplitWhitespace;
use crate::lexical_structure::{Lexeme, Token};
use token_matcher::match_token;

pub trait Lexer {
    // fn lex(&mut self, reader: impl std::io::Read);
    // fn next_token(&mut self) -> Token;
    fn next_lexeme(&mut self) -> Option<Lexeme>;
    fn lex(&mut self) -> Vec<Lexeme>;
}

pub struct CoolLexer {
    content: String,
    current_pos: usize,
    lexemes: Vec<Lexeme>,
}

impl CoolLexer {
    pub fn new(mut reader: impl Read) -> Self {
        let mut content = String::new();
        reader.read_to_string(&mut content);
        CoolLexer {
            content,
            current_pos: 0,
            lexemes: Vec::new(),
        }
    }
}


impl Lexer for CoolLexer {
    fn next_lexeme(&mut self) -> Option<Lexeme> {
        let mut words = self.content[self.current_pos..].split_whitespace();
        let word = words.next();


        let line_number = self.current_pos;
        self.current_pos += word.unwrap().len();
        word.map(|str| Lexeme(
            line_number,
            match_token(str),
            str.to_string(),
        ))
    }

    fn lex(&mut self) -> Vec<Lexeme> {
        while self.current_pos != self.content.len() {
            let lexeme = self.next_lexeme();
            self.lexemes.push(lexeme.unwrap());
        }

        self.lexemes.clone()
    }
}
