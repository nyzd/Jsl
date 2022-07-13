use stack::Stack;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod builtin;

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

struct Interpreter {
    pub stack: Stack,
    pub functions: Vec<Macro>,
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
                &"put" => {
                    println!("{}", self.stack.pop());
                }
                &"putstr" => loop {
                    let pop = self.stack.pop();
                    if pop == 0.0 {
                        break;
                    } else {
                        print!("{}", char::from_u32(pop as u32).unwrap());
                    }
                },
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

                    self.functions
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

                _ => {
                    // maybe its a macro name ?
                    match self
                        .functions
                        .iter()
                        .position(|f| f.name == word.to_string())
                    {
                        Some(ok) => {
                            self.parse(self.functions[ok].body.clone());
                        }
                        None => {}
                    };

                    // Or built in function ?
                    if builtin::is_built_in(word) {
                        builtin::run_built_in(word, self.stack.pop());
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
            let size = match args.get(3) {
                Some(num) => num.parse::<usize>().unwrap(),
                None => 8,
            };
            if arg == "--stack" {
                println!("\n{:?}", &i.stack.items[0..size]);
            }
        }
        None => {}
    }

    Ok(())
}
