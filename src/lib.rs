mod arithmetic_operation;
pub mod error;
mod stack;
mod stack_manipulation;
mod statement;

use crate::error::{Error, Result, Value};
use crate::stack::Stack;
use crate::statement::{CustomStatement, Statement};

pub struct Forth<'a> {
    instructions: Option<Vec<Statement>>,
    custom_instructions: CustomStatement<'a>,
    stack: Stack, //dic_ins: HashMap<String, Statement>,
}

impl<'a> Forth<'a> {
    pub fn new() -> Forth<'a> {
        Self {
            instructions: Some(vec![]),
            custom_instructions: CustomStatement::new(),
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
}
