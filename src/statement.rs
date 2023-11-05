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

pub struct CustomStatement(HashMap<String, Vec<Statement>>);

impl CustomStatement {
    pub fn new() -> Self {
        CustomStatement(HashMap::default())
    }

    pub fn insert_value(&mut self, line: String) -> Result {
        let v: Vec<&str> = line.split_whitespace().collect();

        let aux_key = v[0];
        match v.get(0) {
            Some(cad) => match cad.parse::<i32>() {
                Ok(_) => {
                    return Err(Error::InvalidWord);
                }
                _ => {}
            },
            None => {}
        }
        let mut aux_pop = true;
        if v[0] != v[1] {
            *self.0.entry(v[0].to_string()).or_insert(Vec::default()) = Vec::default();
            aux_pop = false;
        }

        for (i, v) in v.into_iter().enumerate() {
            if i > 0 {
                let commands = match self.known_command(v) {
                    Some(c) => c,
                    None => match Statement::try_from(v) {
                        Ok(s) => Vec::from([s]),
                        Err(_) => {
                            return Err(Error::InvalidWord);
                        }
                    },
                };
                let dic_pos = self.0.get_mut(aux_key).unwrap();
                for l in commands {
                    if aux_pop {
                        dic_pos.pop().unwrap();
                        aux_pop = false;
                    }
                    dic_pos.push(l);
                }
            }
        }
        self.0.shrink_to_fit();
        println!("{:#?}", self.0);
        Ok(())
    }

    pub fn known_command(&self, command: &str) -> Option<Vec<Statement>> {
        match self.0.get(command) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }
}
