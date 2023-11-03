mod arithmetic_operation;
pub mod error;
mod stack;
mod stack_manipulation;
mod statement;

use crate::error::{Error, Result, Value};
use crate::stack::Stack;
use crate::statement::{CustomStatement, Statement};

pub struct Forth {
    instructions: Option<Vec<Statement>>,
    stack: Stack, //dic_ins: HashMap<String, Statement>,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            instructions: Some(vec![]),
            stack: Stack(vec![]),
        }
    }

    pub fn stack(&mut self) -> &[Value] {
        todo!()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        self.stack.create(self.instructions.take().unwrap())
        //todo!("result of evaluating '{input}'")
    }

    pub fn custom_def(&mut self, input: &str) -> Result {
        todo!()
    }
}
