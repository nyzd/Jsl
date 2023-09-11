use crate::{interpreter::StackType, token::Token};

#[derive(Debug, Clone)]
pub struct Let {
    pub name: String,
    pub value: StackType,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<Let>,
    pub memory: Vec<Let>,
    pub scope: Vec<Token>,
}

impl Function {
    pub fn new(name: String, args: Vec<Let>) -> Self {
        Self {
            name,
            args,
            memory: vec![],
            scope: vec![]
        }
    }

    pub fn set_scope(&mut self, tokens: Vec<Token>) {
        self.scope = tokens;
    }
}
