use super::{radix_type::RadixType, CalculatorEngine, NumWidth};
use num::*;

impl CalculatorEngine {
    pub fn set_radix_type_and_num_width(&mut self, radix_type: Option<RadixType>, num_width: Option<NumWidth>) {
        if self.interger_mode {}
    }

    pub fn try_toggle_bit(&mut self, rational: &Rational, bit_no: usize) -> bool {
        false
    }

    pub fn radix_from_radi_type(&self, radix_type: RadixType) -> u64 {
        match radix_type {
            RadixType::Binary => 2,
            RadixType::Octal => 8,
            RadixType::Decimal => 10,
            RadixType::Hexadecimal => 16,
        }
    }

    pub fn update_max_int_digits(&self) {}

    pub fn change_base_constants(&self, radix: u64, max_int_digits: usize, precision: u64) {}

    pub fn base_or_precision_changed(&self) {}
}
