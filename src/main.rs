use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod interpreter;
mod lexer;
mod token;
mod types;

use interpreter::Interpreter;
use lexer::*;

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

    match args.get(2) {
        Some(arg) => {
            if arg == "--stack" {
                let size = match args.get(3) {
                    Some(num) => num.parse::<usize>().unwrap(),
                    None => 8,
                };
                println!("\n{:?}", &i.stack.items[0..size]);
            }

            if arg == "--memory" {
                println!("\n{:?}", &i.memory);
            }
        }
        None => {}
    }

    Ok(())
}
