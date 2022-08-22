
#[derive(Debug)]
pub enum ParseError {
    DivByZeroError,
    UnexpectedError,
    UnsupportedOperand,
    ParseIntError,
    TooManyOrTooFewArguments,
}
