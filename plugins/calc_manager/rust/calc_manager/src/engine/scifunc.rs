use super::{command::Command, CalculatorEngine};
use num::{Rational, Zero};

impl CalculatorEngine {
    pub fn sci_calc_functions(&self, rational: Rational, command: Command) -> Rational {
        Rational::zero()
    }

    pub fn display_error(&self, error: u64) {}
}
