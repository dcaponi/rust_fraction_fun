use std::str::FromStr;

use super::errors::ParseError;

#[derive(Debug)]
pub enum Operand {
    Add, 
    Sub,
    Mul,
    Div,
}

impl FromStr for Operand {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operand::Add),
            "-" => Ok(Operand::Sub),
            "x" => Ok(Operand::Mul),
            "/" => Ok(Operand::Div),
            _ => Err(ParseError::UnsupportedOperand)
        }
    }
}