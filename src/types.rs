use crate::{interpreter::StackType, token::Token};

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub body: Vec<Token>,
}

impl Macro {
    pub fn new(name: String, body: Vec<Token>) -> Self {
        Self { name, body }
    }
}

#[derive(Debug, Clone)]
pub struct Let {
    pub name: String,
    pub value: StackType,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub memory: Vec<Let>,
    pub body: Vec<Token>,
}

impl Function {
    pub fn new(name: String, memory: Vec<Let>, body: Vec<Token>) -> Self {
        Self { name, memory, body }
    }
}
