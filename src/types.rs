use crate::{interpreter::StackType, token::Token};

#[derive(Debug, Clone)]
pub struct Let {
    pub name: String,
    pub value: StackType,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub memory: Vec<Let>,
    pub scope: Vec<Token>,
}

impl Function {
    pub fn new(name: String, memory: Vec<Let>) -> Self {
        Self {
            name,
            memory,
            scope: vec![],
        }
    }

    pub fn set_scope(&mut self, scope: Vec<Token>) {
        self.scope = scope;
    }
}
