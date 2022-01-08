mod lexer;

use logos::Logos;
use lexer::{Token, Lexeme};

// #[derive(Debug)]
// struct Lexeme {
//     token: Token,
//     n: usize,
// }

// #[derive(Logos, Debug, PartialEq)]
// enum Token {
//     // Tokens can be literal strings, of any length.
//     #[token("fast")]
//     Fast,
//
//     #[token(".")]
//     Period,
//
//     // Or regular expressions.
//     #[regex("[a-zA-Z]+")]
//     Text,
//
//     // Logos requires one token variant to handle errors,
//     // it can be named anything you wish.
//     #[error]
//     // We can also use this variant to define whitespace,
//     // or any other matches we wish to skip.
//     #[regex(r"[ \t\n\f]+", logos::skip)]
//     Error,
// }


fn main() {
    let source = "class Main {

};";
    // let tokens = Token::lexer(source).spanned();
    let mut lex = Token::lexer(source);
    let mut lexemes: Vec<Lexeme> = vec![];
    while let Some(token) = lex.next() {
        let span = lex.span();
        let slice = lex.slice();
        let n = source[..span.start].matches("\n").count() + 1;
        lexemes.push(Lexeme(n, token, String::from(slice)))
    }

    // let lexemes = tokens.map(|(token, span)| {
    //     let n = source[..span.start].matches("\n").count();
    //     Lexeme(
    //         n,
    //         token,
    //
    //     )
    // });
    for lexeme in lexemes {
        println!("{:?}", lexeme);
    }
}
