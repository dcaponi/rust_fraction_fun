use std::{fmt::Display, str::FromStr, num::ParseIntError};

use super::errors::ParseError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arg {
    pub who: i32,
    pub num: i32,
    pub den: i32,
}

impl Default for Arg {
    fn default() -> Self {
        Arg{who: 0, num: 0, den: 0}
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.num {
            0 => write!(f, "{}", self.who),
            _ => {
                match self.who {
                    0 => write!(f, "{}/{}", self.num, self.den),
                    _ => write!(f, "{}_{}/{}", self.who, self.num, self.den)
                }
                
            }
        }
    }
}

impl FromStr for Arg {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Arg, Self::Err> {
        let r: Result<Vec<i32>, ParseIntError> = s.split("_")
            .flat_map(|p| p.split("/"))
            .map(|c| c.parse::<i32>())
            .collect();
        match r {
            Ok(t) =>{
                match t.len() {
                    1 => Ok(Arg{who: t[0], num: 0, den: 1}),
                    2 => {
                        if t[1] != 0 {
                            return Ok(Arg{who: 0, num: t[0], den: t[1]})
                        }
                        Err(ParseError::DivByZeroError)
                    },
                    3 => {
                        if t[1] != 0 {
                            return Ok(Arg{who: t[0], num: t[1], den: t[2]})
                        }
                        Err(ParseError::DivByZeroError)
                    },
                    _ => Err(ParseError::UnexpectedError)   
                }
            },
            _ => Err(ParseError::ParseIntError)
        }
    }
}


impl Arg {
    pub fn mixed_to_improp(self) -> Arg {
        match self.who != 0 {
            true => Arg {who: 0, num: self.den * self.who + self.num, den: self.den},
            false => self,
        }
    }
    pub fn improp_to_mixed(self) -> Arg {
        match self.who == 0 && self.num.abs() >= self.den.abs() {
            true => {
                let red_arg = self.reduce();
                Arg {who: red_arg.num / red_arg.den, num: red_arg.num % red_arg.den, den: red_arg.den}
            },
            false => self,
        }
    }
    pub fn reduce(self) -> Arg {
        let mut arg = self.mixed_to_improp();
        let g = Self::gcd(arg.num.abs(), arg.den.abs());
        arg.num /= g;
        arg.den /= g;
        arg
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a
        }
        Self::gcd(b, a % b)
    }
}