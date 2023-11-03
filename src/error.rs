pub type Result = std::result::Result<(), Error>;
pub type Value = i32;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}
