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

#[derive(Debug)]
struct Function {
    name: String,
    args: Vec<Let>,
    body: String,
}

impl Function {
    pub fn new(name: String, args: Vec<Let>, body: String) -> Self {
        Self {
            name,
            args,
            body
        }
    }
}

enum MemoryScope {
    Function,
    Global
}

struct Interpreter {
    pub stack: Stack,
    pub macros: Vec<Macro>,
    pub memory: Vec<Let>,
    pub functions: Vec<Function>,
    pub mem_scope: MemoryScope,
    pub function_time: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            macros: vec![],
            memory: vec![],
            functions: vec![],
            mem_scope: MemoryScope::Global,
            function_time: 0,
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
                        Self::next(&mut iter, &mut index);
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

                    Self::next(&mut iter, &mut index);

                    // get word as a ASCII
                    for byte in content.as_bytes().iter().rev() {
                        self.stack.push((*byte).into());
                    }
                }

                &"times" => {
                    // Run code x times
                    let x = self.stack.pop() as u32;
                    let mut times_body = String::new();

                    Self::next(&mut iter, &mut index);

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
                        "math" => include_str!("../std/math.jsl").to_string(),
                        "std" => include_str!("../std/std.jsl").to_string(),
                        "memory" => include_str!("../std/memory.jsl").to_string(),
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

                    Self::next(&mut iter, &mut index);

                    self.memory.push(Let {
                        name: let_name.to_string(),
                        value: self.stack.pop(),
                    })
                }

                &"set" => {
                    // Global variable
                    // creation of a new let
                    let let_name = aschar[index + 1];

                    Self::next(&mut iter, &mut index);

                    match self.memory.iter().position(|l| l.name == let_name) {
                        Some(l) => self.memory[l].value = self.stack.pop(),
                        None => panic!("Let is not defined!"),
                    }
                }

                &"lets" => {
                    Self::next(&mut iter, &mut index);
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

                &"fn" => {
                    // first find function name
                    let fn_name = aschar[index + 1];
                    Self::next(&mut iter, &mut index);

                    let mut fn_args: Vec<Let>= vec![];

                    Self::next(&mut iter, &mut index);
                    while aschar[index] != "do" {
                        fn_args.push(Let { name: aschar[index].to_string(), value: 0.0 });
                        Self::next(&mut iter, &mut index);
                    }

                    let mut fn_body = String::new();

                    Self::next(&mut iter, &mut index);
                    while aschar[index] != "end" {
                        fn_body.push_str(&(aschar[index].to_owned() + " "));
                        Self::next(&mut iter, &mut index);
                    }

                    self.functions.push(Function::new(fn_name.to_string(), fn_args, fn_body));
                }

                _ => {
                    // maybe its a macro name ?
                    match self.macros.iter().position(|f| f.name == word.to_string()) {
                        Some(ok) => {
                            self.parse(self.macros[ok].body.clone());
                        }
                        None => {}
                    };

                    match self.mem_scope {
                        MemoryScope::Global => {
                            match self.memory.iter().position(|l| l.name == word.to_string()) {
                                Some(ok) => {
                                    self.stack.push(self.memory[ok].value);
                                }
                                None => {}
                            }
                        }

                        MemoryScope::Function => {
                            let args = &self.functions[self.function_time].args;
                            match args.iter().position(|l| l.name == word.to_string()) {
                                Some(ok) => {
                                    self.stack.push(args[ok].value);
                                }

                                None => {}
                            }
                        }
                    }

                    match self.functions.iter().position(|f| f.name == word.to_string()) {
                        Some(ok) => {
                            self.function_time = ok;
                            let mut args = self.functions[ok].args.iter_mut();
                            while let Some(arg) = args.next() {
                                arg.value = self.stack.pop();
                            }
                            self.mem_scope = MemoryScope::Function;
                            self.parse(self.functions[ok].body.to_string());
                            self.mem_scope = MemoryScope::Global;
                        }
                        None => {}
                    }
                }
            }

            index += 1;
        }

    }

    fn next<T>(iter: &mut T, index: &mut usize)
    where 
        T: Iterator
    {
        *index += 1;
        iter.next();
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
