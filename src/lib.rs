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
    custom_instructions: CustomStatement,
    st: Stack,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            instructions: Some(vec![]),
            custom_instructions: CustomStatement::new(),
            st: Stack(vec![]),
        }
    }

    pub fn stack(&mut self) -> &[Value] {
        &self.st.0[..]
    }

    pub fn eval(&mut self, input: &str) -> Result {
        if input.matches(':').count() == input.matches(';').count() {
            let tokens = input.to_lowercase();
            let tokens: Vec<&str> = tokens.split_whitespace().collect();

            let mut custom_ins = false;

            let ins = self.instructions.get_or_insert(Vec::default());
            let mut custom_ins_str: String = String::default();

            for (_, value) in tokens.into_iter().enumerate() {
                if custom_ins {
                    // Considera usar Rc
                    if value.chars().nth(0).unwrap() == ';' {
                        custom_ins = false;
                        self.custom_instructions
                            .insert_value(custom_ins_str.clone())?;
                    } else {
                        custom_ins_str = custom_ins_str + " " + value;
                    }
                } else {
                    if value.chars().nth(0).unwrap() == ':' {
                        custom_ins = true;
                        custom_ins_str = String::default();
                    } else {
                        match self.custom_instructions.known_command(value) {
                            Some(c) => {
                                for l in c {
                                    ins.push(l);
                                }
                            }
                            None => match Statement::try_from(value) {
                                Ok(st) => {
                                    ins.push(st);
                                }
                                Err(_) => {
                                    return Err(Error::UnknownWord);
                                }
                            },
                        }
                    }
                }
            }

            self.st.create(self.instructions.take().unwrap())
        } else {
            Err(Error::InvalidWord)
        }
        //todo!("result of evaluating '{input}'")
    }
}
