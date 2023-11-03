use crate::arithmetic_operation::ArithmeticOperation;
use crate::error::{Error, Result, Value};
use crate::stack_manipulation::StackManipulation;
use crate::statement::Statement;

pub struct Stack(pub Vec<Value>);

impl Stack {
    pub fn create(&mut self, instructions: Vec<Statement>) -> Result {
        for (_, v) in instructions.into_iter().enumerate() {
            self.ex(v)?;
        }

        Ok(())
    }

    fn ex(&mut self, statement: Statement) -> Result {
        match statement {
            Statement::Operation(op) => self.ex_arithmetic_operation(op),
            Statement::StackOrder(so) => self.ex_stack_order(so),
            Statement::Number(n) => {
                self.0.push(n);
                Ok(())
            }
        }
    }

    fn ex_arithmetic_operation(&mut self, op: ArithmeticOperation) -> Result {
        if self.0.len() >= 2 {
            let n1 = self.0.pop().unwrap();
            let n2 = self.0.pop().unwrap();
            match op {
                ArithmeticOperation::Addition => {
                    self.0.push(n1 + n2);
                }
                ArithmeticOperation::Subtraction => {
                    self.0.push(n2 - n1);
                }
                ArithmeticOperation::Division => {
                    if n1 != 0 {
                        self.0.push((n2 as f64 / n1 as f64) as i32);
                    } else {
                        return Err(Error::DivisionByZero);
                    }
                }
                ArithmeticOperation::Multiplication => {
                    self.0.push(n1 * n2);
                }
            };
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn ex_stack_order(&mut self, so: StackManipulation) -> Result {
        if self.0.len() > 0 {
            match so {
                StackManipulation::Drop => {
                    self.0.pop();
                }
                StackManipulation::Dup => self.0.push(self.0[self.0.len() - 1]),
                StackManipulation::Over => {
                    if self.0.len() > 1 {
                        self.0.push(self.0[self.0.len() - 2])
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                StackManipulation::Swap => {
                    if self.0.len() > 1 {
                        let n1 = self.0.pop().unwrap();
                        let n2 = self.0.pop().unwrap();
                        self.0.push(n1);
                        self.0.push(n2);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
            };

            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }
}
