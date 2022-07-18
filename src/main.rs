use stack::Stack;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Macro {
    name: String,
    body: String,
}

impl Macro {
    pub fn new(name: String, body: String) -> Self {
        Self { name, body }
    }
}

fn is_string_numeric(str: String) -> bool {
    let mut result = false;
    for c in str.chars() {
        if !c.is_numeric() {
            result = false;
        }
    }

    if str.parse::<f64>().is_ok() {
        result = true;
    }

    return result;
}

#[derive(Debug)]
struct Let {
    name: String,
    value: f64,
}

struct Interpreter {
    pub stack: Stack,
    pub macros: Vec<Macro>,
    pub memory: Vec<Let>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            macros: vec![],
            memory: vec![],
        }
    }

    fn parse(&mut self, source: String) {
        let aschar: Vec<&str> = source.split_whitespace().collect();
        let mut iter = aschar.iter();
        let mut index = 0;

        while let Some(word) = iter.next() {
            if is_string_numeric(word.to_string()) {
                self.stack.push(word.parse::<f64>().unwrap());
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
                &"swap" => {
                    let i1 = self.stack.pop();
                    let i2 = self.stack.pop();

                    self.stack.push(i1);
                    self.stack.push(i2);
                }
                &"rot" => {
                    let i1 = self.stack.pop();
                    let i2 = self.stack.pop();
                    let i3 = self.stack.pop();

                    self.stack.push(i1);
                    self.stack.push(i2);
                    self.stack.push(i3);
                }
                &"put" => {
                    println!("{}", self.stack.pop());
                }
                &"putc" => {
                    let pop = self.stack.pop();
                    print!("{}", char::from_u32(pop as u32).unwrap());
                }
                &"macro" => {
                    // find function name
                    let macro_name = aschar[index + 1];
                    let mut macro_body = String::new();

                    index += 2;

                    iter.next();
                    iter.next();

                    while aschar[index] != "end" {
                        macro_body.push_str(&(aschar[index].to_owned() + " "));
                        index += 1;
                        iter.next();
                    }

                    self.macros
                        .push(Macro::new(macro_name.to_string(), macro_body));
                }

                &"eq" => {
                    // Pop items from stack
                    let b = self.stack.pop() == self.stack.pop();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(res);
                }

                &"noteq" => {
                    // Pop items from stack
                    let b = self.stack.pop() != self.stack.pop();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(res);
                }

                &"bigger" => {
                    let b = self.stack.pop() < self.stack.pop();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(res);
                }

                &"smaller" => {
                    let b = self.stack.pop() > self.stack.pop();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(res);
                }

                &"then" => {
                    let stk = self.stack.pop();
                    if stk == 1.0 {
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
                &"2dup" => {
                    // Duplicate top of stack
                    let item1 = self.stack.pop();
                    let item2 = self.stack.pop();

                    self.stack.push(item1);
                    self.stack.push(item2);
                    self.stack.push(item1);
                    self.stack.push(item2);
                }

                &"true" => {
                    self.stack.push(1.);
                }

                &"false" => {
                    self.stack.push(0.);
                }

                &"drop" => {
                    self.stack.pop();
                }

                &"str" => {
                    // Next element in word will be a string
                    let content = aschar[index + 1];

                    index += 1;
                    iter.next();

                    // get word as a ASCII
                    for byte in content.as_bytes().iter().rev() {
                        self.stack.push((*byte).into());
                    }
                }

                &"times" => {
                    // Run code x times
                    let x = self.stack.pop() as u32;
                    let mut times_body = String::new();

                    index += 1;

                    // Copy body
                    while aschar[index] != "done" {
                        times_body.push_str(&(aschar[index].to_owned() + " "));
                        index += 1;
                        iter.next();
                    }

                    for _i in 0..x {
                        self.parse(times_body.clone());
                    }
                }

                &"import" => {
                    let file_name = aschar[index + 1];

                    index += 1;

                    // Check filename for std
                    let result = match file_name {
                        "math" => include_str!("./std/math.jsl").to_string(),
                        "std" => include_str!("./std/std.jsl").to_string(),
                        _ => {
                            // read file
                            let mut file = File::open(file_name).unwrap();
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).unwrap();

                            contents
                        }
                    };
                    self.parse(result);

                    iter.next();
                }

                &"let" => {
                    // Global variable
                    // creation of a new let
                    let let_name = aschar[index + 1];

                    index += 1;
                    iter.next();

                    self.memory.push(Let {
                        name: let_name.to_string(),
                        value: self.stack.pop(),
                    })
                }

                &"set" => {
                    // Global variable
                    // creation of a new let
                    let let_name = aschar[index + 1];

                    index += 1;
                    iter.next();

                    match self.memory.iter().position(|l| l.name == let_name) {
                        Some(l) => self.memory[l].value = self.stack.pop(),
                        None => panic!("Let is not defined!"),
                    }
                }

                &"lets" => {
                    index += 1;
                    iter.next();
                    while aschar[index] != "ok" {
                        // Create a new let in memory
                        self.memory.push(Let {
                            name: aschar[index].to_string(),
                            value: self.stack.pop(),
                        });
                        index += 1;
                        iter.next();
                    }
                }

                &"mempop" => {
                    match self.memory.pop() {
                        Some(x) => self.stack.push(x.value),
                        None => self.stack.push(0.0),
                    };
                }

                &"memusage" => {
                    // return length of created variables
                    self.stack.push(self.memory.len() as f64);
                }

                _ => {
                    // maybe its a macro name ?
                    match self.macros.iter().position(|f| f.name == word.to_string()) {
                        Some(ok) => {
                            self.parse(self.macros[ok].body.clone());
                        }
                        None => {}
                    };

                    match self.memory.iter().position(|l| l.name == word.to_string()) {
                        Some(ok) => {
                            self.parse(self.memory[ok].value.to_string());
                        }
                        None => {}
                    }
                }
            }

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
