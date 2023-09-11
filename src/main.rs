use interpreter::Interpreter;
use lexer::Lexer;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod interpreter;
mod lexer;
mod token;
mod types;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        panic!("Args is not valid");
    }
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lexer = Lexer::new(contents);
    let tokens = lexer.lex();

    let mut i = Interpreter::new();
    i.parse(tokens);

    Ok(())
}
