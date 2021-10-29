mod lexer;
mod lexical_structure;

use lexer::Lexer;

fn main() {
    lexer::lex();
    lexer::my_lexer::COOLLexer().lex();
}
