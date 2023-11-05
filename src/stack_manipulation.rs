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
            "over" => Ok(StackManipulation::Over),
            _ => Err("Invalid str"),
        }
    }
}
