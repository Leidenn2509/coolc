extern crate coolc;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

use coolc::lexer::{CoolLexer, Lexer};

fn main() {
    let args: env::Args = env::args();
    if args.len() <= 1 {
        // panic!("No file names in args.")
        return;
    }
    let mut tokens_by_file: HashMap<String, Vec<String>> = HashMap::new();
    for arg in args.skip(1) {
        let tokens = lex(&arg);
        tokens_by_file.insert(arg, tokens);
    }
    for (file_name, tokens) in tokens_by_file {
        println!("file: {}\n{:?}", file_name, tokens);
    }
}

fn lex(file_name: &String) -> Vec<String> {
    let file = fs::File::open(file_name)
        // .expect("Can't open input file");
        .unwrap_or_else(|e| panic!("Can't open input file {}. {:?}", file_name, e));

    let mut reader = BufReader::new(file);

    let mut lexer = CoolLexer::new(reader);
    let lexemes = lexer.lex();
    println!("lexemes: {:?}", lexemes);
    todo!();

    let mut line = String::new();
    let mut res: Vec<String> = Vec::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(length) if length == 0 => {
                break;
            }
            Ok(_) => {
                res.push(line.clone());
                line.clear();
            }
            Err(_) => {
                panic!("Cant read from file");
            }
        }
    }
    res
}