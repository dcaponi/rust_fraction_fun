
use std::io::{self};
use std::str::FromStr;

use equation::arg::Arg;
use crate::equation::equation::Equation;

mod equation;
fn main() {
    println!("### Welcome to Fraction Fun ###");
    println!("Enter a fraction equation where arguments follow the convention W_n/d or n/d:");
    println!("Valid operators are + - x /");
    println!("Example: -2_4/7 x 3/8");
    println!("Example: 4/7 - -3");

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let out = Equation::from_str(&buffer).unwrap().solve();
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let out = Equation::from_str("1 + 1").unwrap().solve().to_string();
        assert_eq!(out, "2");
    }

    #[test]
    #[should_panic]
    fn zero_denominator_breaks() {
        Equation::from_str("1 - 1/0").unwrap().solve().to_string();
    }

    #[test]
    #[should_panic]
    fn too_many_args_breaks() {
        Equation::from_str("1 - 1/1 + 2").unwrap().solve().to_string();
    }

    #[test]
    #[should_panic]
    fn too_few_args_breaks() {
        Equation::from_str("1 -").unwrap().solve().to_string();
    }

    #[test]
    #[should_panic]
    fn invalid_operand_breaks() {
        Equation::from_str("1 ^ 2").unwrap().solve().to_string();
    }

    #[test]
    #[should_panic]
    fn div_by_zero_breaks() {
        Equation::from_str("1 / 0").unwrap().solve().to_string();
    }

    #[test]
    fn it_adds() {
        let frac = Equation::from_str("1/2 + 1/2").unwrap().solve().to_string();
        let mix = Equation::from_str("1_1/2 + 1/2").unwrap().solve().to_string();
        let neg_mix = Equation::from_str("1_1/2 + -1/2").unwrap().solve().to_string();
        let neg_denom = Equation::from_str("1/2 + 1/-2").unwrap().solve().to_string();
        let zero_num = Equation::from_str("1/2 + 0/2").unwrap().solve().to_string();
        assert_eq!(frac, "1");
        assert_eq!(mix, "2");
        assert_eq!(neg_mix, "1");
        assert_eq!(neg_denom, "0");
        assert_eq!(zero_num, "1/2");
    }

    #[test]
    fn it_suptracts() {
        let frac = Equation::from_str("1/2 - 1/2").unwrap().solve().to_string();
        let mix = Equation::from_str("1_1/2 - 1/2").unwrap().solve().to_string();
        let neg_mix = Equation::from_str("1_1/2 - -1/2").unwrap().solve().to_string();
        let neg_denom = Equation::from_str("1/2 - 1/-2").unwrap().solve().to_string();
        let zero_num = Equation::from_str("1/2 - 0/2").unwrap().solve().to_string();
        assert_eq!(frac, "0");
        assert_eq!(mix, "1");
        assert_eq!(neg_mix, "2");
        assert_eq!(neg_denom, "1");
        assert_eq!(zero_num, "1/2");
    }

     #[test]
    fn it_multiplies() {
        let frac = Equation::from_str("1/2 x 1/2").unwrap().solve().to_string();
        let mix = Equation::from_str("1_1/2 x 1/2").unwrap().solve().to_string();
        let neg_mix = Equation::from_str("1_1/2 x -1/2").unwrap().solve().to_string();
        let neg_denom = Equation::from_str("1/2 x 1/-2").unwrap().solve().to_string();
        let zero_num = Equation::from_str("1/2 x 0/2").unwrap().solve().to_string();
        assert_eq!(frac, "1/4");
        assert_eq!(mix, "3/4");
        assert_eq!(neg_mix, "-3/4");
        assert_eq!(neg_denom, "-1/4");
        assert_eq!(zero_num, "0");
    }

    #[test]
    fn it_divides() {
        let frac = Equation::from_str("1/2 / 1/2").unwrap().solve().to_string();
        let mix = Equation::from_str("1_1/2 / 1/2").unwrap().solve().to_string();
        let neg_mix = Equation::from_str("1_1/2 / -1/2").unwrap().solve().to_string();
        let neg_denom = Equation::from_str("1/2 / 1/-2").unwrap().solve().to_string();
        let zero_num = Equation::from_str("0/2 / 1/2").unwrap().solve().to_string();
        assert_eq!(frac, "1");
        assert_eq!(mix, "3");
        assert_eq!(neg_mix, "-3");
        assert_eq!(neg_denom, "-1");
        assert_eq!(zero_num, "0");
    }

}