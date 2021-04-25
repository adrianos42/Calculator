use super::{command::Command, CalculatorEngine};
use num::{Rational, Zero};

// Routines to perform standard operations &|^~<<>>+-/*% and pwr.
impl CalculatorEngine {
    pub fn do_operation(&self, operation: Command, lhs: Rational, rhs: Rational) -> Rational {
        // Remove any variance in how 0 could be represented in rat e.g. -0, 0/n, etc.
        let mut result = if lhs != Rational::zero() {
            lhs
        } else {
            Rational::zero()
        };

        result = match operation {
            //Command::And => result &= rhs,
            //Command::Or => result |= rhs,
            //Command::Xor => result ^= rhs,
            //Command::Nand => result = (result & rhs) & self.get_chop_number(),
            //Command::Nor => result = (result | rhs) & self.get_chop_number(),
            Command::Rshf => {
                //let msb: bool = ((w64bits >> (self.bit_width - 1)) & 1) as bool;
                Rational::zero()
            }
            Command::Rshfl => Rational::zero(),
            Command::Lshf => Rational::zero(),
            Command::Add => result + rhs,
            Command::Sub => rhs - result,
            Command::Mul => result * rhs,
            Command::Div | Command::Mod => Rational::zero(),
            Command::Pwr => Rational::zero(),
            Command::Root => Rational::zero(),
            Command::LogBaseY => Rational::zero(),
            _ => panic!("Wrong operation argument"),
        };

        result
    }
}
