mod token_matcher;

use std::cmp::min;
use std::io::{BufRead, Read, BufReader};
use std::str::SplitWhitespace;
use crate::lexical_structure::{Lexeme, Token};
use token_matcher::match_token;
//
// lexer! {
//     fn next_token(text) -> Lexeme;
//
// }
//
// pub trait Lexer {
//     // fn lex(&mut self, reader: impl std::io::Read);
//     // fn next_token(&mut self) -> Token;
//     fn next_lexeme(&mut self) -> Option<Lexeme>;
//     fn lex(&mut self) -> Vec<Lexeme>;
// }
//
// pub struct CoolLexer<T> {
//     reader: BufReader<T>,
//     current_line: String,
//     current_line_num: usize,
//     current_pos_in_line: usize,
//     lexemes: Vec<Lexeme>,
// }
//
// impl<T> CoolLexer<T> {
//     pub fn new(mut reader: impl Read) -> Self {
//         let mut content = String::new();
//         reader.lines()
//         reader.read_to_string(&mut content);
//
//         CoolLexer {
//             content,
//             current_line: 0,
//             current_pos_in_line: 0,
//             lexemes: Vec::new(),
//         }
//     }
// }
//
//
// impl Lexer for CoolLexer {
//     fn next_lexeme(&mut self) -> Option<Lexeme> {
//         let mut words = self.content[self.current_pos..].split_whitespace();
//         let word = words.next();
//
//
//         let line_number = self.current_pos;
//         self.current_pos += word.unwrap().len();
//         word.map(|str| Lexeme(
//             line_number,
//             match_token(str),
//             str.to_string(),
//         ))
//     }
//
//     fn lex(&mut self) -> Vec<Lexeme> {
//         while self.current_pos != self.content.len() {
//             let lexeme = self.next_lexeme();
//             self.lexemes.push(lexeme.unwrap());
//         }
//
//         self.lexemes.clone()
//     }
// }
