use cool_lexer::{Lexeme, lex};

fn main() {
    let source = "class Main {

};";
    let lexemes: Vec<Lexeme> = lex(source);

    for lexeme in lexemes {
        println!("{:?}", lexeme);
    }
}
