mod token;
mod parser;
mod ast;
mod lex;
mod eval;
mod env;

use std::{env as fs_env, fs};
use std::path::Iter;
use logos::Logos;
use crate::lex::lex;
use crate::token::LogosToken;
use crate::parser::Parser;

fn main() {
    let dir = fs_env::current_dir().unwrap();
    let mut input = fs::read_to_string(format!(
        "{}/examples/simple.lir",
        dir.as_path().to_str().unwrap()
    )).expect("Something went wrong reading the file");

    // let mut stream = lex(input.as_str());
    // while let Some(t) = stream.next() {
    //     println!("{:?}", t)
    // }
    let mut parser = Parser::new(
        lex(input.as_str()), input.as_str());
    let mut ast = parser.parse();
    for n in ast {
        println!("{:?}", n);
    }
}
