use stack::Stack;
use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;

#[derive(Debug)]
struct Function {
    name: String,
    body: String,
//    args: Vec<String>,
}

impl Function {
    pub fn new(name: String, body: String) -> Self {
        Self {
            name,
            body,
        }
    }
}

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

struct Interpreter {
    pub stack: Stack,
    pub functions: Vec<Function>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            functions: vec![],
        }
    }

    fn parse(&mut self, source: String) {
        let aschar: Vec<&str> = source.split_whitespace().collect();
        let mut iter = aschar.iter();
        let mut index = 0;
    
        while let Some(word) = iter.next() {
            if is_string_numeric(word.to_string()) {
                self.stack.push(word.parse::<u8>().unwrap());
            }

            match word {
                //&"0" | &"1" | &"2" | &"3" | &"4" | &"5" | &"6" | &"7" | &"8" | &"9" => {}
                &"add" => {
                    let push = self.stack.pop() + self.stack.pop();
                    self.stack.push(push);
                }
                &"minus" => {
                    let push = self.stack.pop() - self.stack.pop();
                    self.stack.push(push);
                }
                &"div" => {
                    let push = self.stack.pop() / self.stack.pop();
                    self.stack.push(push);
                }
                &"mul" => {
                    let push = self.stack.pop() * self.stack.pop();
                    self.stack.push(push);
                }
                &"print" => {
                    println!("{}", self.stack.pop());
                }
                &"fn" => {
                    // find function name
                    let fn_name = aschar[index + 1];
                    let mut fn_body = String::new();

                    index += 2;
                
                    iter.next();
                    iter.next();

                    while aschar[index] != "end" {
                        fn_body.push_str(&(aschar[index].to_owned() + " "));
                        index += 1;
                        iter.next();
                    }

                    self.functions.push(Function::new(
                        fn_name.to_string(),
                        fn_body,
                    ));
                }
                
                &"eq" => {
                    // Pop items from stack
                    let b = self.stack.pop() == self.stack.pop();
                    let res = if b == true {
                        1
                    } else {
                        0
                    };
                    self.stack.push(res);
                }
                
                &"noteq" => {
                    // Pop items from stack
                    let b = self.stack.pop() != self.stack.pop();
                    let res = if b == true {
                        1
                    } else {
                        0
                    };
                    self.stack.push(res);
                }

                &"then" => {
                    let stk = self.stack.pop();
                    if stk == 1 {
                        // Run next code
                        self.parse(aschar[index].to_string());
                    } else {
                        iter.next();
                    }
                }

                &"dup" => {
                    // Duplicate top of stack
                    let item = self.stack.pop();
                    
                    self.stack.push(item);    
                    self.stack.push(item);    
                }

                &"true" => {
                    self.stack.push(1);
                }

                &"false" => {
                    self.stack.push(0);
                }

                _ => {
                    // maybe its a function name ?
                    match self.functions.iter().position(|f| f.name == word.to_string()) {
                        Some(ok) => {
                            self.parse(self.functions[ok].body.clone());
                        },
                        None => {}
                    };
                }
            }

            println!("{:?}", self.stack.items);
            index += 1;
        }
    }
}

fn main() -> io::Result<()> { 
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        panic!("Args is not valid"); 
    }
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut i = Interpreter::new();
    i.parse(contents);

    Ok(())
}
