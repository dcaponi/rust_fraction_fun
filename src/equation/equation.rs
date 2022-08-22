use std::str::FromStr;

use crate::Arg;

use super::{operand::Operand, errors::ParseError};

pub struct Equation {
    args: Vec<Arg>,
    op: Operand,
}

impl Default for Equation {
    fn default() -> Self {
        Equation { args: vec![Arg{..Default::default()}, Arg{..Default::default()}], op: Operand::Add }
    }
}

impl Equation {
    pub fn solve(self) -> Arg {
        let args: Vec<Arg> = self.args.iter().map(|arg| arg.mixed_to_improp()).collect();

        let mut out = match self.op {
            Operand::Add => Arg{ who: 0, num: args[0].num * args[1].den + args[1].num * args[0].den, den: args[0].den * args[1].den },
            Operand::Sub => Arg{ who: 0, num: args[0].num * args[1].den - args[1].num * args[0].den, den: args[0].den * args[1].den },
            Operand::Mul => Arg{ who: 0, num: args[0].num * args[1].num, den: args[0].den * args[1].den },
            Operand::Div => Arg{ who: 0, num: args[0].num * args[1].den, den: args[0].den * args[1].num },
        }.reduce();

        if out.den < 0 {
            out.den = out.den.abs();
            out.num = out.num * -1;
        }
        out.improp_to_mixed()
    }
}

impl FromStr for Equation {
    type Err = ParseError;
    fn from_str(eq: &str) -> Result<Equation, Self::Err> {
        let mut equation = Equation { ..Default::default() };
        
        let mut parts: Vec<&str> = eq.split_whitespace().collect();

        if parts.len() != 3 {
            return Err(ParseError::TooManyOrTooFewArguments)
        }

        if let Some(arg2) = parts.pop() {
            equation.args[1] = Arg::from_str(arg2).unwrap();
        }

        if let Some(op) = parts.pop() {
            equation.op = Operand::from_str(op).unwrap();
        }

        if let Some(arg1) = parts.pop() {
            equation.args[0] = Arg::from_str(arg1).unwrap();
        }

        Ok(equation)
    }
}


