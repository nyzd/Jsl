use crate::{token::Token, types::*};
use std::{
    ops::{Add, Div, Mul, Rem, Sub},
    path::Path,
};

trait Size {
    fn get_size(&self) -> usize;
}

pub enum MemoryScope {
    Function,
    Global,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum StackType {
    Float(f64),
    String(String),
    Array(Vec<StackType>),
}

impl Size for StackType {
    fn get_size(&self) -> usize {
        match self {
            StackType::Float(f) => *f as usize,
            StackType::Array(vec) => vec.len(),
            StackType::String(string) => string.len(),
        }
    }
}

impl StackType {
    fn print(self) {
        match self {
            Self::Float(f) => println!("{}", f),
            Self::String(str) => println!("{}", str),
            Self::Array(vec) => println!("{:?}", vec),
        }
    }
}

impl Add for StackType {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(f) => match rhs {
                Self::Float(rf) => Self::Float(f + rf),
                _ => panic!("Cant Div"),
            },
            _ => panic!("Cant Div"),
        }
    }
}

impl Sub for StackType {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(f) => match rhs {
                Self::Float(rf) => Self::Float(f - rf),
                _ => panic!("Cant Div"),
            },
            _ => panic!("Cant Div"),
        }
    }
}

impl Div for StackType {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(f) => match rhs {
                Self::Float(rf) => Self::Float(f / rf),
                _ => panic!("Cant Div"),
            },

            _ => panic!("Cant Div"),
        }
    }
}

impl Mul for StackType {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(f) => match rhs {
                Self::Float(rf) => Self::Float(f * rf),
                _ => panic!("Cant Div"),
            },
            _ => panic!("Cant Div"),
        }
    }
}

impl Rem for StackType {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(f) => match rhs {
                Self::Float(rf) => Self::Float(f % rf),
                _ => panic!("Cant Div"),
            },
            _ => panic!("Cant Div"),
        }
    }
}

pub struct Interpreter {
    pub stack: Vec<StackType>,
    pub macros: Vec<Macro>,
    pub memory: Vec<Let>,
    pub functions: Vec<Function>,
    pub mem_scope: MemoryScope,
    pub function_time: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(255),
            macros: vec![],
            memory: vec![],
            functions: vec![],
            mem_scope: MemoryScope::Global,
            function_time: 0,
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) {
        let mut iter = tokens.iter();

        while let Some(token) = iter.next() {
            match token {
                Token::Number(n) => {
                    self.stack.push(StackType::Float(n.to_owned()));
                }
                Token::Add => {
                    let push = self.stack.pop().unwrap() + self.stack.pop().unwrap();
                    self.stack.push(push);
                }
                Token::Minus => {
                    let push = self.stack.pop().unwrap() - self.stack.pop().unwrap();
                    self.stack.push(push);
                }
                Token::Div => {
                    let push = self.stack.pop().unwrap() / self.stack.pop().unwrap();
                    self.stack.push(push);
                }
                Token::Mul => {
                    let push = self.stack.pop().unwrap() * self.stack.pop().unwrap();
                    self.stack.push(push);
                }
                Token::Mod => {
                    let push = self.stack.pop().unwrap() % self.stack.pop().unwrap();
                    self.stack.push(push);
                }
                Token::Swap => {
                    let i1 = self.stack.pop().unwrap();
                    let i2 = self.stack.pop().unwrap();

                    self.stack.push(i1);
                    self.stack.push(i2);
                }
                Token::Rot => {
                    let i1 = self.stack.pop().unwrap();
                    let i2 = self.stack.pop().unwrap();
                    let i3 = self.stack.pop().unwrap();

                    self.stack.push(i1);
                    self.stack.push(i2);
                    self.stack.push(i3);
                }
                Token::Put => self.stack.pop().unwrap().print(),
                Token::Macro(m) => {
                    self.macros.push(m.to_owned());
                }
                Token::Eq => {
                    // Pop items from stack
                    let b = self.stack.pop().unwrap() == self.stack.pop().unwrap();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(StackType::Float(res));
                }

                Token::Noteq => {
                    // Pop items from stack
                    let b = self.stack.pop() != self.stack.pop();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(StackType::Float(res));
                }

                Token::Bigger => {
                    let b = self.stack.pop() < self.stack.pop();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(StackType::Float(res));
                }

                Token::Smaller => {
                    let b = self.stack.pop() > self.stack.pop();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(StackType::Float(res));
                }

                // TODO: Dont support functions.
                Token::Then(tokens) => {
                    let stk = self.stack.pop().unwrap();
                    if stk == StackType::Float(1.0) {
                        // Run next code
                        self.parse(tokens.to_owned());
                    } else {
                        iter.next();
                    }
                }

                Token::Dup => {
                    // Duplicate top of stack
                    let item = self.stack.pop().unwrap();

                    self.stack.push(item.clone());
                    self.stack.push(item);
                }
                Token::True => {
                    self.stack.push(StackType::Float(1.));
                }

                Token::False => {
                    self.stack.push(StackType::Float(0.));
                }

                Token::Drop => {
                    self.stack.pop();
                }

                Token::Str(content) => {
                    self.stack.push(StackType::String(content.to_owned()));
                }

                Token::Times(tks) => {
                    // Run code x times
                    let x = self.stack.pop().unwrap();
                    match x {
                        StackType::Float(x) => {
                            for _i in 0..x as u32 {
                                self.parse(tks.to_vec());
                            }
                        }

                        _ => panic!("Cant iter over Not float type"),
                    }
                }

                Token::Import(tks) => {
                    self.parse(tks.to_vec());
                }

                Token::Let(name) => match self.mem_scope {
                    MemoryScope::Function => {
                        let f = &mut self.functions[self.function_time];
                        f.memory.push(Let {
                            name: name.to_string(),
                            value: self.stack.pop().unwrap(),
                        })
                    }
                    MemoryScope::Global => self.memory.push(Let {
                        name: name.to_string(),
                        value: self.stack.pop().unwrap(),
                    }),
                },

                Token::Set(let_name) => match self.mem_scope {
                    MemoryScope::Function => {
                        let function = &mut self.functions[self.function_time];

                        match function.memory.iter().position(|l| &l.name == let_name) {
                            Some(l) => function.memory[l].value = self.stack.pop().unwrap(),
                            None => panic!("Let is not defined!"),
                        }
                    }
                    MemoryScope::Global => {
                        match self.memory.iter().position(|l| &l.name == let_name) {
                            Some(l) => self.memory[l].value = self.stack.pop().unwrap(),
                            None => panic!("Let is not defined!"),
                        }
                    }
                },

                Token::Mempop => {
                    match self.memory.pop() {
                        Some(x) => self.stack.push(x.value),
                        None => {}
                    };
                }

                Token::Memusage => {
                    // return length of created variables
                    self.stack.push(StackType::Float(self.memory.len() as f64));
                }

                Token::Function(func) => {
                    self.functions.push(func.to_owned());
                }

                Token::Call(name) => match self.functions.iter().position(|f| &f.name == name) {
                    Some(ok) => {
                        self.function_time = ok;
                        let f = &mut self.functions[self.function_time];
                        let mut args = f.memory.iter_mut();
                        while let Some(arg) = args.next() {
                            arg.value = self.stack.pop().unwrap();
                        }
                        self.mem_scope = MemoryScope::Function;
                        self.parse(self.functions.get(ok).unwrap().to_owned().body);
                        self.mem_scope = MemoryScope::Global;
                    }
                    None => {}
                },

                Token::Array(tokens) => {
                    let mut parser = Self::new();
                    parser.parse(tokens.to_owned());

                    self.stack.push(StackType::Array(parser.stack))
                }

                Token::Ident(name) => {
                    match self.mem_scope {
                        MemoryScope::Global => {
                            match self.memory.iter().position(|l| &l.name == name) {
                                Some(ok) => {
                                    self.stack.push(self.memory[ok].value.clone());
                                }
                                None => {}
                            }
                        }

                        MemoryScope::Function => {
                            let args = &self.functions[self.function_time].memory;
                            match args.iter().position(|l| &l.name == name) {
                                Some(ok) => {
                                    self.stack.push(args[ok].value.clone());
                                }

                                None => {}
                            }
                        }
                    }

                    match self.macros.iter().position(|m| &m.name == name) {
                        Some(ok) => {
                            self.parse(self.macros[ok].body.clone());
                        }
                        None => {}
                    };

                    self.match_rstd(name);
                }
            }
        }
    }

    fn match_rstd(&mut self, name: &str) {
        // rstd functions
        match name {
            "fs::readFile" => {
                let file_path = self.stack.pop().unwrap();
                match file_path {
                    StackType::String(str) => {
                        let content = rstd::fs::read_file(Path::new(&str).to_path_buf()).unwrap();
                        self.stack.push(StackType::String(content));
                    }
                    _ => panic!("Cant pass not string type to readFile"),
                }
            }
            "fs::createFile" => {
                let file_content = self.stack.pop().unwrap();
                let file_path = self.stack.pop().unwrap();

                match file_path {
                    StackType::String(path) => match file_content {
                        StackType::String(content) => {
                            rstd::fs::create_file(Path::new(&path).to_path_buf(), content).unwrap();
                        }

                        _ => panic!("Invalid item type"),
                    },
                    _ => panic!("Invalid item type"),
                };
            }
            "length" => {
                let item = self.stack.pop().unwrap().get_size() as f64;
                self.stack.push(StackType::Float(item));
            }
            "array::nth" => {
                let nth = self.stack.pop().unwrap();
                let array = self.stack.pop().unwrap();

                match array {
                    StackType::Array(arr) => match nth {
                        StackType::Float(f) => {
                            self.stack.push(arr.get(f as usize).unwrap().to_owned());
                        }
                        _ => panic!("index must be a f64 type"),
                    },

                    _ => panic!("Cant get nth from not array type"),
                }
            }
            "array::pop" => {
                let array = self.stack.pop().unwrap();

                match array {
                    StackType::Array(mut arr) => {
                        self.stack.push(arr.pop().unwrap());
                        self.stack.push(StackType::Array(arr));
                    }

                    _ => panic!("Cant get pop from not array type"),
                }
            }
            "array::push" => {
                let value = self.stack.pop().unwrap();
                let array = self.stack.pop().unwrap();

                match array {
                    StackType::Array(a) => {
                        let mut new_array = a.clone();
                        new_array.push(value);
                        self.stack.push(StackType::Array(new_array));
                    }
                    _ => panic!("You cant push Value to not array value"),
                }
            }
            _ => {}
        }
    }
}
