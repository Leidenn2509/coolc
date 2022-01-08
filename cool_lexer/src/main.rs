use logos::Logos;

#[derive(Debug)]
struct Lexeme {
    token: Token,
    n: usize,
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("fast")]
    Fast,

    #[token(".")]
    Period,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}


fn main() {
    let source = "class Main {

};";
    let tokens = Token::lexer(source).spanned();
    let lexemes = tokens.map(|(token, span)| {
        let n = source[..span.start].matches("\n").count();
        Lexeme {
            token,
            n,
        }
    });
    for lexeme in lexemes {
        println!("{:?}", lexeme);
    }
}
