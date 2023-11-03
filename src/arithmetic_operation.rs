#[derive(Debug, Clone, Copy)]
pub enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl TryFrom<&str> for ArithmeticOperation {
    type Error = &'static str;
    fn try_from(f: &str) -> std::result::Result<Self, &'static str> {
        match f {
            "+" => Ok(ArithmeticOperation::Addition),
            "-" => Ok(ArithmeticOperation::Subtraction),
            "*" => Ok(ArithmeticOperation::Multiplication),
            "/" => Ok(ArithmeticOperation::Division),
            _ => Err("Invalid char"),
        }
    }
}
