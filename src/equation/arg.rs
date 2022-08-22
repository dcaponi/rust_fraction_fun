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
            true => {
                if self.who < 0 {
                    return Arg {who: 0, num: (self.den * self.who.abs() + self.num) * -1, den: self.den};

                } else {
                    Arg {who: 0, num: self.den * self.who + self.num, den: self.den}
                }
            },
            false => self,
        }
    }
    pub fn improp_to_mixed(self) -> Arg {
        match self.who == 0 && self.num.abs() >= self.den.abs() {
            true => {
                let red_arg = self.reduce();
                Arg { who: red_arg.num / red_arg.den, num: (red_arg.num % red_arg.den).abs(), den: red_arg.den.abs()}
            },
            false => self,
        }
    }
    pub fn reduce(self) -> Arg {
        let mut arg = self.mixed_to_improp();
        let g = Self::gcd(arg.num.abs(), arg.den.abs());
        arg.num /= g;
        arg.den /= g;
        if arg.den < 0 {
            arg.num *= -1;
            arg.den = arg.den.abs();
        }
        arg
    }

    fn gcd(a: i32, b: i32) -> i32 {
        match b {
            0 => a,
            _ => Self::gcd(b, a % b)
        }
    }

 
}

#[test]
fn test_mixed_to_improp() {
    let red = Arg{who: 0, num: 1, den: 2}.mixed_to_improp();
    let big_frac = Arg{who: 0, num: 2, den: 4}.mixed_to_improp();
    let mix = Arg{who: 3, num: 2, den: 4}.mixed_to_improp();
    let who = Arg{who: 2, num: 0, den: 1}.mixed_to_improp();
    let neg = Arg{who: -1, num: 1, den: 2}.mixed_to_improp();
    let improp = Arg{who: 0, num: 9, den: 4}.mixed_to_improp();
    
    assert_eq!(red, Arg{who: 0, num: 1, den: 2});
    assert_eq!(big_frac, Arg{who: 0, num: 2, den: 4}); // mixed_to_improp will not reduce for you
    assert_eq!(mix, Arg{who: 0, num: 14, den: 4}); // mixed_to_improp will not reduce for you
    assert_eq!(who, Arg{who: 0, num: 2, den: 1});
    assert_eq!(neg, Arg{who: 0, num: -3, den: 2});
    assert_eq!(improp, Arg{who: 0, num: 9, den: 4});
}

#[test]
fn test_improp_to_mixed() {
    let red = Arg{who: 0, num: 1, den: 2}.improp_to_mixed();
    let big_frac = Arg{who: 0, num: 2, den: 4}.improp_to_mixed();
    let mix = Arg{who: 3, num: 2, den: 4}.improp_to_mixed();
    let improp = Arg{who: 0, num: 9, den: 4}.improp_to_mixed();
    let neg = Arg{who: 0, num: -3, den: 2}.improp_to_mixed();
    
    assert_eq!(red, Arg{who: 0, num: 1, den: 2});
    assert_eq!(big_frac, Arg{who: 0, num: 2, den: 4}); // improp_to_mixed will not reduce for you
    assert_eq!(mix, Arg{who: 3, num: 2, den: 4}); // improp_to_mixed will not reduce for you
    assert_eq!(improp, Arg{who: 2, num: 1, den: 4});
    assert_eq!(neg, Arg{who: -1, num: 1, den: 2});
}

#[test]
fn test_reduce() {
    let red = Arg{who: 0, num: 1, den: 2}.reduce();
    let big_frac = Arg{who: 0, num: 2, den: 4}.reduce();
    let mix = Arg{who: 3, num: 2, den: 4}.reduce();
    let improp = Arg{who: 0, num: -18, den: 4}.reduce();
    let dumb = Arg{who: 0, num: -18, den: -4}.reduce();
    
    assert_eq!(red, Arg{who: 0, num: 1, den: 2});
    assert_eq!(big_frac, Arg{who: 0, num: 1, den: 2});
    assert_eq!(mix, Arg{who: 0, num: 7, den: 2});
    assert_eq!(improp, Arg{who: 0, num: -9, den: 2});
    assert_eq!(dumb, Arg{who: 0, num: 9, den: 2});
}

#[test]
fn test_gcd() {
    assert_eq!(Arg::gcd(3, 5), 1);
    assert_eq!(Arg::gcd(30, 6), 6);
}

