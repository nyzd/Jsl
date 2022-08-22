use crate::{token::Token, types::*};
use stack::Stack;

pub enum MemoryScope {
    Function,
    Global,
}

pub struct Interpreter {
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

    pub fn parse(&mut self, tokens: Vec<Token>) {
        let mut iter = tokens.iter();
        let mut index = 0;
        while let Some(token) = iter.next() {
            match token {
                Token::Number(n) => {
                    self.stack.push(n.to_owned());
                }
                Token::Add => {
                    let push = self.stack.pop() + self.stack.pop();
                    self.stack.push(push);
                }
                Token::Minus => {
                    let push = self.stack.pop() - self.stack.pop();
                    self.stack.push(push);
                }
                Token::Div => {
                    let push = self.stack.pop() / self.stack.pop();
                    self.stack.push(push);
                }
                Token::Mul => {
                    let push = self.stack.pop() * self.stack.pop();
                    self.stack.push(push);
                }
                Token::Swap => {
                    let i1 = self.stack.pop();
                    let i2 = self.stack.pop();

                    self.stack.push(i1);
                    self.stack.push(i2);
                }
                Token::Rot => {
                    let i1 = self.stack.pop();
                    let i2 = self.stack.pop();
                    let i3 = self.stack.pop();

                    self.stack.push(i1);
                    self.stack.push(i2);
                    self.stack.push(i3);
                }
                Token::Put => {
                    println!("{}", self.stack.pop());
                }
                Token::Putc => {
                    let pop = self.stack.pop();
                    print!("{}", char::from_u32(pop as u32).unwrap());
                }
                Token::Macro(m) => {
                    self.macros.push(m.to_owned());
                }

                Token::Eq => {
                    // Pop items from stack
                    let b = self.stack.pop() == self.stack.pop();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(res);
                }

                Token::Noteq => {
                    // Pop items from stack
                    let b = self.stack.pop() != self.stack.pop();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(res);
                }

                Token::Bigger => {
                    let b = self.stack.pop() < self.stack.pop();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(res);
                }

                Token::Smaller => {
                    let b = self.stack.pop() > self.stack.pop();
                    let res = if b == true { 1.0 } else { 0.0 };
                    self.stack.push(res);
                }

                Token::Then => {
                    let stk = self.stack.pop();
                    if stk == 1.0 {
                        // Run next code
                        let f: Vec<Token> = vec![tokens.get(index).unwrap().to_owned()];
                        self.parse(f);
                    } else {
                        iter.next();
                    }
                }

                Token::Dup => {
                    // Duplicate top of stack
                    let item = self.stack.pop();

                    self.stack.push(item);
                    self.stack.push(item);
                }
                Token::True => {
                    self.stack.push(1.);
                }

                Token::False => {
                    self.stack.push(0.);
                }

                Token::Drop => {
                    self.stack.pop();
                }

                Token::Str(content) => {
                    // get word as a ASCII
                    for byte in content.as_bytes().iter().rev() {
                        self.stack.push((*byte).into());
                    }
                }

                Token::Times(tks) => {
                    // Run code x times
                    let x = self.stack.pop() as u32;
                    for _i in 0..x {
                        self.parse(tks.to_vec());
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
                            value: self.stack.pop(),
                        })
                    }
                    MemoryScope::Global => self.memory.push(Let {
                        name: name.to_string(),
                        value: self.stack.pop(),
                    }),
                },

                Token::Set(let_name) => match self.mem_scope {
                    MemoryScope::Function => {
                        let function = &mut self.functions[self.function_time];

                        match function.memory.iter().position(|l| &l.name == let_name) {
                            Some(l) => {
                                let f = &mut self.functions[self.function_time];
                                f.memory[l].set_value(self.stack.pop());
                            }
                            None => panic!("Let is not defined!"),
                        }
                    }
                    MemoryScope::Global => {
                        match self.memory.iter().position(|l| &l.name == let_name) {
                            Some(l) => self.memory[l].value = self.stack.pop(),
                            None => panic!("Let is not defined!"),
                        }
                    }
                },

                Token::Mempop => {
                    match self.memory.pop() {
                        Some(x) => self.stack.push(x.value),
                        None => self.stack.push(0.0),
                    };
                }

                Token::Memusage => {
                    // return length of created variables
                    self.stack.push(self.memory.len() as f64);
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
                            arg.value = self.stack.pop();
                        }
                        self.mem_scope = MemoryScope::Function;
                        self.parse(self.functions.get(ok).unwrap().to_owned().body);
                        self.mem_scope = MemoryScope::Global;
                    }
                    None => {}
                },

                Token::Ident(name) => {
                    let mut error = false;
                    match self.mem_scope {
                        MemoryScope::Global => {
                            match self.memory.iter().position(|l| &l.name == name) {
                                Some(ok) => {
                                    self.stack.push(self.memory[ok].value);
                                }
                                None => error = true,
                            }
                        }

                        MemoryScope::Function => {
                            let args = &self.functions[self.function_time].memory;
                            match args.iter().position(|l| &l.name == name) {
                                Some(ok) => {
                                    error = false;
                                    self.stack.push(args[ok].value);
                                }

                                None => error = true,
                            }
                        }
                    }

                    match self.macros.iter().position(|f| &f.name == name) {
                        Some(ok) => {
                            error = false;
                            self.parse(self.macros[ok].body.clone());
                        }
                        None => error = true,
                    };

                    if error {
                        panic!("Token is not defined : {:?}", token);
                    }
                }
            }

            index += 1;
        }
    }
}
