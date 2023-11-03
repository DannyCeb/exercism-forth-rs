#[allow(unused)]
use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticOperations {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl TryFrom<&str> for ArithmeticOperations {
    type Error = &'static str;
    fn try_from(f: &str) -> std::result::Result<Self, &'static str> {
        match f {
            "+" => Ok(ArithmeticOperations::Addition),
            "-" => Ok(ArithmeticOperations::Subtraction),
            "*" => Ok(ArithmeticOperations::Multiplication),
            "/" => Ok(ArithmeticOperations::Division),
            _ => Err("Invalid char"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StackManipulation {
    Dup,
    Drop,
    Swap,
    Over,
}

impl TryFrom<&str> for StackManipulation {
    type Error = &'static str;
    fn try_from(f: &str) -> std::result::Result<Self, &'static str> {
        match &f.to_lowercase()[..] {
            "dup" => Ok(StackManipulation::Dup),
            "drop" => Ok(StackManipulation::Drop),
            "swap" => Ok(StackManipulation::Swap),
            "Over" => Ok(StackManipulation::Over),
            _ => Err("Invalid str"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Statements {
    Operation(ArithmeticOperations),
    StackOrder(StackManipulation),
    Number(Value),
}

impl TryFrom<&str> for Statements {
    type Error = &'static str;
    fn try_from(f: &str) -> std::result::Result<Self, &'static str> {
        match ArithmeticOperations::try_from(f) {
            Ok(ar_op) => Ok(Statements::Operation(ar_op)),
            Err(_) => match StackManipulation::try_from(f) {
                Ok(st_ma) => Ok(Statements::StackOrder(st_ma)),
                Err(_) => match f.parse::<i32>() {
                    Ok(number) => Ok(Statements::Number(number)),
                    Err(_) => Err("Invalid str"),
                },
            },
        }
    }
}

struct Stack(Vec<Value>);

impl Stack {
    fn create(&mut self, instructions: Vec<Statements>) -> Result {
        for (_, v) in instructions.into_iter().enumerate() {
            self.ex(v)?;
        }

        Ok(())
    }

    fn ex(&mut self, statement: Statements) -> Result {
        match statement {
            Statements::Operation(op) => self.ex_arithmetic_operation(op),
            Statements::StackOrder(so) => self.ex_stack_order(so),
            Statements::Number(n) => {
                self.0.push(n);
                Ok(())
            }
        }
    }

    fn ex_arithmetic_operation(&mut self, op: ArithmeticOperations) -> Result {
        if self.0.len() >= 2 {
            let n1 = self.0.pop().unwrap();
            let n2 = self.0.pop().unwrap();
            match op {
                ArithmeticOperations::Addition => {
                    self.0.push(n1 + n2);
                }
                ArithmeticOperations::Subtraction => {
                    self.0.push(n2 - n1);
                }
                ArithmeticOperations::Division => {
                    if n1 != 0 {
                        self.0.push((n2 as f64 / n1 as f64) as i32);
                    } else {
                        return Err(Error::DivisionByZero);
                    }
                }
                ArithmeticOperations::Multiplication => {
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

pub struct Forth {
    instructions: Option<Vec<Statements>>,
    stack: Stack, //dic_ins: HashMap<String, Statements>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
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
