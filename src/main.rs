use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

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

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Let {
    name: String,
    value: StackType,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Function {
    name: String,
    memory: Vec<Let>,
    body: String,
}

impl Function {
    pub fn new(name: String, memory: Vec<Let>, body: String) -> Self {
        Self { name, memory, body }
    }
}

enum MemoryScope {
    Function,
    Global,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum BoxedType {
    BoxedType(Box<BoxedType>),
    BoxedFn(Box<Function>),
    StackTypeBoxed(Box<StackType>),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum StackType {
    Number(f64),
    Boxed(BoxedType),
}

impl Add for StackType {
    type Output = StackType;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => match rhs {
                Self::Number(rn) => StackType::Number(n + rn),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl Sub for StackType {
    type Output = StackType;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => match rhs {
                Self::Number(rn) => StackType::Number(n - rn),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl Mul for StackType {
    type Output = StackType;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => match rhs {
                Self::Number(rn) => StackType::Number(n * rn),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl Div for StackType {
    type Output = StackType;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => match rhs {
                Self::Number(rn) => StackType::Number(n / rn),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl Display for StackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            _ => todo!(),
        }
    }
}

struct Interpreter {
    pub stack: Vec<StackType>,
    pub macros: Vec<Macro>,
    pub memory: Vec<Let>,
    pub functions: Vec<Function>,
    pub mem_scope: MemoryScope,
    pub function_time: usize,
}

#[derive(Debug)]
struct ParseError(String);

impl Interpreter {
    pub fn get_pop(&mut self) -> Result<StackType, ParseError> {
        let Some(result )= self.stack.pop() else {
            return Err(ParseError("Cant get inside the type".to_string()));
        };

        Ok(result)
    }

    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(250),
            macros: vec![],
            memory: vec![],
            functions: vec![],
            mem_scope: MemoryScope::Global,
            function_time: 0,
        }
    }

    fn parse(&mut self, source: String) -> Result<(), ParseError> {
        let aschar: Vec<&str> = source.split_whitespace().collect();
        let mut iter = aschar.iter();
        let mut index = 0;

        while let Some(word) = iter.next() {
            if is_string_numeric(word.to_string()) {
                self.stack
                    .push(StackType::Number(word.parse::<f64>().unwrap()));
            }

            match word {
                //&"0" | &"1" | &"2" | &"3" | &"4" | &"5" | &"6" | &"7" | &"8" | &"9" => {}
                &"add" => {
                    let push = self.get_pop()? + self.get_pop()?;
                    self.stack.push(push);
                }
                &"minus" => {
                    let push = self.get_pop()? - self.get_pop()?;
                    self.stack.push(push);
                }
                &"div" => {
                    let push = self.get_pop()? / self.get_pop()?;
                    self.stack.push(push);
                }
                &"mul" => {
                    let push = self.get_pop()? * self.get_pop()?;
                    self.stack.push(push);
                }
                &"swap" => {
                    let i1 = self.get_pop()?;
                    let i2 = self.get_pop()?;

                    self.stack.push(i1);
                    self.stack.push(i2);
                }
                &"rot" => {
                    let i1 = self.get_pop()?;
                    let i2 = self.get_pop()?;
                    let i3 = self.get_pop()?;

                    self.stack.push(i1);
                    self.stack.push(i2);
                    self.stack.push(i3);
                }
                &"put" => {
                    println!("{}", self.get_pop()?);
                }
                &"putc" => {
                    let get_pop = self.get_pop()?;
                    match get_pop {
                        StackType::Number(n) => {
                            print!("{}", char::from_u32(n as u32).unwrap());
                        }
                        _ => todo!(),
                    }
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
                    // get_pop items from stack
                    let b = self.get_pop()? == self.get_pop()?;
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(StackType::Number(res));
                }

                &"noteq" => {
                    // get_pop items from stack
                    let b = self.get_pop()? != self.get_pop()?;
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(StackType::Number(res));
                }

                &"bigger" => {
                    let b = self.get_pop()? < self.get_pop()?;
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(StackType::Number(res));
                }

                &"smaller" => {
                    let b = self.get_pop()? > self.get_pop()?;
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(StackType::Number(res));
                }

                &"then" => {
                    let stk = self.get_pop()?;
                    if stk == StackType::Number(1.0) {
                        // Run next code
                        self.parse(aschar[index].to_string())?;
                    } else {
                        iter.next();
                    }
                }

                &"dup" => {
                    // Duplicate top of stack
                    let item = self.get_pop()?;

                    self.stack.push(item.clone());
                    self.stack.push(item);
                }
                &"2dup" => {
                    // Duplicate top of stack
                    let item1 = self.get_pop()?;
                    let item2 = self.get_pop()?;

                    self.stack.push(item1.clone());
                    self.stack.push(item2.clone());
                    self.stack.push(item1);
                    self.stack.push(item2);
                }

                &"true" => {
                    self.stack.push(StackType::Number(1.));
                }

                &"false" => {
                    self.stack.push(StackType::Number(0.));
                }

                &"drop" => {
                    self.get_pop()?;
                }

                &"str" => {
                    // Next element in word will be a string
                    let content = aschar[index + 1];

                    Self::next(&mut iter, &mut index);

                    // get word as a ASCII
                    for byte in content.as_bytes().iter().rev() {
                        self.stack.push(StackType::Number((*byte).into()));
                    }
                }

                &"times" => {
                    // Run code x times
                    let x = self.get_pop()?;
                    let mut times_body = String::new();

                    Self::next(&mut iter, &mut index);

                    // Copy body
                    while aschar[index] != "done" {
                        times_body.push_str(&(aschar[index].to_owned() + " "));
                        index += 1;
                        iter.next();
                    }

                    match x {
                        StackType::Number(n) => {
                            for _i in 0..(n as u32) {
                                self.parse(times_body.clone())?;
                            }
                        }

                        _ => todo!(),
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
                    self.parse(result)?;

                    iter.next();
                }

                &"let" => {
                    // Global variable
                    // creation of a new let
                    let let_name = aschar[index + 1];

                    Self::next(&mut iter, &mut index);

                    let value = self.get_pop()?;

                    match self.mem_scope {
                        MemoryScope::Function => {
                            self.functions[self.function_time].memory.push(Let {
                                name: let_name.to_string(),
                                value,
                            })
                        }
                        MemoryScope::Global => self.memory.push(Let {
                            name: let_name.to_string(),
                            value,
                        }),
                    }
                }

                &"set" => {
                    // Global variable
                    // creation of a new let
                    let let_name = aschar[index + 1];

                    Self::next(&mut iter, &mut index);

                    let value = self.get_pop()?;

                    match self.mem_scope {
                        MemoryScope::Function => {
                            let function = &mut self.functions[self.function_time];

                            match function.memory.iter().position(|l| l.name == let_name) {
                                Some(l) => function.memory[l].value = value,
                                None => panic!("Let is not defined!"),
                            }
                        }
                        MemoryScope::Global => {
                            match self.memory.iter().position(|l| l.name == let_name) {
                                Some(l) => self.memory[l].value = value,
                                None => panic!("Let is not defined!"),
                            }
                        }
                    }
                }

                &"fn" => {
                    // first find function name
                    let fn_name = aschar[index + 1];
                    Self::next(&mut iter, &mut index);

                    let mut fn_args: Vec<Let> = vec![];

                    Self::next(&mut iter, &mut index);
                    while aschar[index] != "do" {
                        fn_args.push(Let {
                            name: aschar[index].to_string(),
                            value: StackType::Number(0.0),
                        });
                        Self::next(&mut iter, &mut index);
                    }

                    let mut fn_body = String::new();

                    Self::next(&mut iter, &mut index);
                    while aschar[index] != "end" {
                        fn_body.push_str(&(aschar[index].to_owned() + " "));
                        Self::next(&mut iter, &mut index);
                    }

                    self.functions
                        .push(Function::new(fn_name.to_string(), fn_args, fn_body));
                }

                &"box" => {
                    let data = self.get_pop()?;
                    let boxed = match data {
                        StackType::Boxed(bx) => BoxedType::BoxedType(Box::new(bx)),
                        StackType::Number(num) => {
                            BoxedType::StackTypeBoxed(Box::new(StackType::Number(num)))
                        }
                    };

                    self.stack.push(StackType::Boxed(boxed));
                }

                &"boxfn" => {
                    let data = self.functions.pop();
                    let boxed: Box<Function> = Box::new(data.unwrap());
                    self.stack.push(StackType::Boxed(BoxedType::BoxedFn(boxed)));
                }

                &"unbox" => {
                    let boxed = self.get_pop()?;
                    match boxed {
                        StackType::Boxed(boxed_t) => match boxed_t {
                            BoxedType::BoxedType(bt) => {
                                self.stack.push(StackType::Boxed(bt.as_ref().clone()))
                            }
                            BoxedType::BoxedFn(bf) => self.functions.push(bf.as_ref().clone()),
                            BoxedType::StackTypeBoxed(stb) => self.stack.push(stb.as_ref().clone()),
                        },
                        _ => panic!("unbox just works for boxed types"),
                    };
                }

                _ => {
                    // maybe its a macro name ?
                    match self.macros.iter().position(|f| f.name == word.to_string()) {
                        Some(ok) => {
                            self.parse(self.macros[ok].body.clone())?;
                        }
                        None => {}
                    };

                    match self.mem_scope {
                        MemoryScope::Global => {
                            match self.memory.iter().position(|l| l.name == word.to_string()) {
                                Some(ok) => {
                                    self.stack.push(self.memory[ok].value.clone());
                                }
                                None => {}
                            }
                        }

                        MemoryScope::Function => {
                            let args = &self.functions[self.function_time].memory;
                            match args.iter().position(|l| l.name == word.to_string()) {
                                Some(ok) => {
                                    self.stack.push(args[ok].value.clone());
                                }

                                None => {}
                            }
                        }
                    }

                    match self
                        .functions
                        .iter()
                        .position(|f| f.name == word.to_string())
                    {
                        Some(ok) => {
                            let value = self.get_pop()?;
                            self.function_time = ok;
                            let mut args = self.functions[ok].memory.iter_mut();
                            while let Some(arg) = args.next() {
                                arg.value = value.clone();
                            }
                            self.mem_scope = MemoryScope::Function;
                            self.parse(self.functions[ok].body.to_string())?;
                            self.mem_scope = MemoryScope::Global;
                        }
                        None => {}
                    }
                }
            }

            index += 1;
        }

        Ok(())
    }

    fn next<T>(iter: &mut T, index: &mut usize)
    where
        T: Iterator,
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
    i.parse(contents).unwrap();

    match args.get(2) {
        Some(arg) => {
            if arg == "--memory" {
                println!("\n{:?}", &i.memory);
            }
            if arg == "--stack" {
                println!("\n{:?}", &i.stack);
            }
        }
        None => {}
    }

    Ok(())
}
