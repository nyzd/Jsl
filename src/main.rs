use rustyline::error::ReadlineError;
use rustyline::Editor;
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
    match args.get(1) {
        Some(arg) => {
            if arg == &"run".to_string() {
                let mut file = File::open(&args[2])?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                let lexer = Lexer::new(contents);
                let tokens = lexer.lex();

                let mut i = Interpreter::new();
                i.parse(tokens);

                match args.get(3) {
                    Some(arg) => {
                        if arg == "--stack" {
                            println!("\n{:?}", &i.stack);
                        }

                        if arg == "--memory" {
                            println!("\n{:?}", &i.memory);
                        }
                    }
                    None => {}
                }
            }
        }
        None => {
            let mut rl = Editor::<()>::new().unwrap();
            let mut i = Interpreter::new();
            loop {
                let readline = rl.readline("Jsl >> ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        let lexer = Lexer::new(line.clone());
                        let tokens = lexer.lex();

                        i.parse(tokens);

                        if line != "".to_string() {
                            println!("{:?}", i.stack)
                        }
                    }
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
