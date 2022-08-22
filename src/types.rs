use crate::token::Token;

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
    pub value: f64,
}

impl Let {
    pub fn set_value(&mut self, new_value: f64) -> () {
        self.value = new_value;
    }
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
