use crate::{token::Token, types::*};
use std::ops::{Add, Div, Mul, Rem, Sub};

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
    pub memory: Vec<Let>,
    pub functions: Vec<Function>,
    pub mem_scope: MemoryScope,
    pub function_time: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(255),
            memory: vec![],
            functions: vec![],
            mem_scope: MemoryScope::Global,
            function_time: 0,
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) {
        let mut iter = tokens.iter();

        while let Some(token) = iter.next() {
            //println!("{:?}", token);
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

                Token::Then => {
                    let stk = self.stack.pop().unwrap();

                    if stk == StackType::Float(1.0) {
                        // Run next code
                        let scope = iter.next().unwrap();

                        match scope {
                            Token::Scope(tokens) => {
                                self.parse(tokens.to_owned());
                            }

                            _ => panic!("Expected scope after then"),
                        }
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

                Token::Times => {
                    // Run code x times
                    let x = self.stack.pop().unwrap();
                    match x {
                        StackType::Float(x) => {
                            let next_token = iter.next().unwrap();

                            for _i in 0..x as u32 {
                                match next_token {
                                    Token::Scope(tokens) => {
                                        self.parse(tokens.to_vec());
                                    }
                                    _ => panic!("Expected scope after times"),
                                }
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
                    let new_token = iter.next().unwrap();
                    let mut function = func.clone();
                    match new_token {
                        Token::Scope(tokens) => {
                            function.set_scope(tokens.to_vec());
                        }

                        _ => panic!("Expected scope after function args"),
                    }
                    self.functions.push(function);
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
                        self.parse(self.functions.get(ok).unwrap().to_owned().scope);
                        self.mem_scope = MemoryScope::Global;
                    }
                    None => {}
                },

                Token::Array(tokens) => {
                    let mut parser = Self::new();
                    parser.parse(tokens.to_owned());

                    self.stack.push(StackType::Array(parser.stack))
                }

                Token::Scope(tokens) => {
                    self.parse(tokens.to_owned());
                }

                Token::Ident(name) => match self.mem_scope {
                    MemoryScope::Global => match self.memory.iter().position(|l| &l.name == name) {
                        Some(ok) => {
                            self.stack.push(self.memory[ok].value.clone());
                        }
                        None => {}
                    },

                    MemoryScope::Function => {
                        let args = &self.functions[self.function_time].memory;
                        match args.iter().position(|l| &l.name == name) {
                            Some(ok) => {
                                self.stack.push(args[ok].value.clone());
                            }

                            None => {}
                        }
                    }
                },
            }
        }
    }
}
