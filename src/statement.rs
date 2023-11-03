use std::collections::HashMap;

use crate::arithmetic_operation::ArithmeticOperation;
use crate::error::*;
use crate::stack_manipulation::StackManipulation;

#[derive(Debug, Clone, Copy)]
pub enum Statement {
    Operation(ArithmeticOperation),
    StackOrder(StackManipulation),
    Number(Value),
}

impl TryFrom<&str> for Statement {
    type Error = &'static str;
    fn try_from(f: &str) -> std::result::Result<Self, &'static str> {
        match ArithmeticOperation::try_from(f) {
            Ok(ar_op) => Ok(Statement::Operation(ar_op)),
            Err(_) => match StackManipulation::try_from(f) {
                Ok(st_ma) => Ok(Statement::StackOrder(st_ma)),
                Err(_) => match f.parse::<i32>() {
                    Ok(number) => Ok(Statement::Number(number)),
                    Err(_) => Err("Invalid str"),
                },
            },
        }
    }
}

#[derive(Debug, Clone)]

pub struct CustomStatement(HashMap<&'static str, Vec<Statement>>);

impl CustomStatement {
    pub fn new() -> Self {
        CustomStatement(HashMap::default())
    }

    pub fn insert_value(&mut self, value: &str) -> Result {
        todo!()
    }
}
