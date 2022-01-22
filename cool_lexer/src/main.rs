use cool_common::Lexeme;
use cool_lexer::lex;
use std::fs;

fn main() {
    let source = fs::read_to_string("/home/alrai/projects/coolc/cool_lexer/res/main.cool").unwrap();
    let lexemes: Vec<Lexeme> = lex(source.as_str());

    for lexeme in lexemes {
        println!("{:?}", lexeme);
    }
}
