use num::*;

pub const MAX_STRLEN: usize = 84;

const C_NUM_MAX_DIGITS: usize = MAX_STRLEN;
const C_EXP_MAX_DIGITS: usize = 4;

pub struct CalcNumSec {
    value: String,
    is_negative: bool,
}

impl CalcNumSec {
    fn new() -> Self {
        CalcNumSec {
            value: String::new(),
            is_negative: false,
        }
    }

    pub fn clear(&mut self) {
        self.value.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn get_is_negative(&self) -> bool {
        self.is_negative
    }

    pub fn set_is_negative(&mut self, value: bool) {
        self.is_negative = value;
    }
}

pub struct CalcInput {
    has_exponent: bool,
    has_decimal: bool,
    dec_pt_index: usize,
    dec_symbol: char,
    base: CalcNumSec,
    exponent: CalcNumSec,
}

impl CalcInput {
    pub fn new() -> Self {
        Self::new_with_symbol('.')
    }

    pub fn new_with_symbol(dec_symbol: char) -> Self {
        CalcInput {
            base: CalcNumSec::new(),
            exponent: CalcNumSec::new(),
            dec_pt_index: 0,
            dec_symbol: dec_symbol,
            has_decimal: false,
            has_exponent: false,
        }
    }

    pub fn clear(&mut self) {
        self.base.clear();
        self.exponent.clear();
        self.has_exponent = false;
        self.has_decimal = false;
        self.dec_pt_index = 0;
    }

    pub fn try_toggle_sign(&mut self, is_integer_mode: bool, max_num_str: &str) -> bool {
        if self.base.is_empty() {
            self.base.is_negative = false;
            self.exponent.is_negative = false;
        } else if self.has_exponent {
            self.exponent.is_negative = !self.exponent.is_negative;
        } else {
            if is_integer_mode && self.base.is_negative {
                if self.base.value.len() >= max_num_str.len()
                    && self.base.value.chars().last().unwrap() > max_num_str.chars().last().unwrap()
                {
                    return false;
                }
            }

            self.base.is_negative = !self.base.is_negative;
        }

        true
    }

    pub fn try_add_digit(
        &mut self,
        value: u64,
        radix: u64,
        is_integer_mode: bool,
        max_num_str: &str,
        word_bit_width: usize,
        max_digits: usize,
    ) -> bool {
        // Convert from an integer into a character.
        // This includes both normal digits and alpha 'digits' for radixes > 10.
        let digit: char = if value < 10 {
            ('0' as u8 + value as u8) as char
        } else {
            ('A' as u8 + value as u8 - 10) as char
        };

        let mut num_sec: &mut CalcNumSec;
        let mut max_count: usize;

        if self.has_exponent {
            num_sec = &mut self.exponent;
            max_count = C_EXP_MAX_DIGITS;
        } else {
            num_sec = &mut self.base;
            max_count = max_digits;

            // Don't include the decimal point in the count. In that way you can enter the maximum allowed precision.
            // Precision doesn't include decimal point.
            if self.has_decimal {
                max_count += 1;
            }

            // First leading 0 is not counted in input restriction as the output can be of that form
            // See Numberto_string algorithm. REVIEW: We don't have such input restriction mimicking based on output of Numberto_string for exponent
            // Numberto_string can give 10 digit exponent, but we still restrict the exponent here to be only 4 digits.
            if !num_sec.is_empty() && num_sec.value.chars().nth(0).unwrap() == '0' {
                max_count += 1;
            }
        }

        if num_sec.is_empty() && value == 0 {
            return true;
        }

        if num_sec.value.len() < max_count {
            num_sec.value.push(digit);
            return true;
        }

        // if we are in integer mode, within the base, and we're on the last digit then
        // there are special cases where we can actually add one more digit.
        if is_integer_mode && num_sec.value.len() == max_count && self.has_exponent {
            let mut allow_extra_digit = false;

            if radix == 0 {
                match word_bit_width % 3 {
                    1 => allow_extra_digit = num_sec.value.chars().nth(0).unwrap() == '1',
                    2 => allow_extra_digit = num_sec.value.chars().nth(0).unwrap() <= '3',
                    _ => panic!("Wrong bit width."),
                }
            } else if radix == 10 {
                // If value length is at least the max, we know we can't add another digit.
                if num_sec.value.len() < max_num_str.len() {
                    // Compare value to substring of maxNumStr of value.size() length.
                    // If cmpResult > 0:
                    // eg. max is "127", and the current number is "20". first digit itself says we are out.
                    // Additional digit is not possible.

                    // If cmpResult < 0:
                    // Success case. eg. max is "127", and current number is say "11". The second digit '1' being <
                    // corresponding digit '2', means all digits are possible to append, like 119 will still be < 127.

                    // If cmpResult == 0:
                    // Undecided still. The case when max is "127", and current number is "12". Look for the new number being 7 or less to allow.
                    if num_sec
                        .value
                        .as_str()
                        .lt(&max_num_str[..num_sec.value.len()])
                    {
                        allow_extra_digit = true;
                    } else if num_sec.value == max_num_str[..num_sec.value.len()] {
                        let last_char = max_num_str.chars().last().unwrap();
                        if digit <= last_char {
                            allow_extra_digit = true;
                        } else if num_sec.is_negative && digit <= (last_char as u8 + 1) as char {
                            // Negative value case, eg. max is "127", and current number is "-12".
                            //Then 8 is also valid, as the range is always from -(max+1)...max in signed mode.
                            allow_extra_digit = true;
                        }
                    }
                }
            }

            if allow_extra_digit {
                num_sec.value.push(digit);
                return true;
            }
        }

        false
    }

    pub fn try_add_decimal_pt(&mut self) -> bool {
        if self.has_decimal || self.has_exponent {
            return false;
        }

        if self.base.is_empty() {
            self.base.value.push('0');
        }

        self.dec_pt_index = self.base.value.len();
        self.base.value.push(self.dec_symbol);
        self.has_decimal = true;

        true
    }

    pub fn has_decimal_pt(&self) -> bool {
        self.has_decimal
    }

    pub fn try_begin_exponent(&mut self) -> bool {
        self.try_add_decimal_pt();

        if self.has_exponent {
            return false;
        }

        self.has_exponent = true;
        true
    }

    pub fn backspace(&mut self) {
        if self.has_exponent {
            if !self.exponent.is_empty() {
                if self.exponent.value.pop().is_none() {
                    self.exponent.clear();
                }
            } else {
                self.has_exponent = false;
            }
        } else {
            if !self.base.is_empty() {
                if self.base.value == "0" {
                    self.base.value.pop().unwrap();
                }
            }

            if self.base.value.len() <= self.dec_pt_index {
                // Backed up over decimal point.
                self.has_decimal = false;
                self.dec_pt_index = 0;
            }

            if self.base.is_empty() {
                self.base.clear();
            }
        }
    }

    pub fn set_decimal_symbol(&mut self, dec_symbol: char) {
        if self.dec_symbol != self.dec_symbol {
            self.dec_symbol = self.dec_symbol;

            if self.has_decimal {
                self.base.value.insert(self.dec_pt_index, self.dec_symbol);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.base.is_empty() && !self.has_exponent && self.exponent.is_empty() && !self.has_decimal
    }

    pub fn to_string(&self, radix: u64) -> String {
        if (self.base.value.len() > MAX_STRLEN)
            || (self.has_exponent && self.exponent.value.len() > MAX_STRLEN)
        {
            return String::new();
        }

        let mut result = String::new();

        if self.base.is_negative {
            result.push('-');
        }

        if self.base.is_empty() {
            result.push('0');
        } else {
            result += self.base.value.as_str();
        }

        if self.has_exponent {
            // Add a decimal point if it is not already there.
            if !self.has_decimal {
                result.push(self.dec_symbol);
            }

            result.push(if radix == 10 { 'e' } else { '^' });

            if self.exponent.is_empty() {
                result.push('0');
            } else {
                result += self.exponent.value.as_str();
            }
        }

        // Base and Exp can each be up to C_NUM_MAX_DIGITS in length, plus 4 characters for sign, dec, exp, and expSign.
        if result.len() > C_NUM_MAX_DIGITS * 2 + 4 {
            result.clear();
        }

        result
    }

    pub fn to_rational(
        &self,
        radix: u64,
        precision: crate::engine::CalculatorPrecision,
    ) -> Rational {
        // TODO
        Rational::zero()
    }
}

#[cfg(test)]
mod cacl_input_tests {
    fn clear() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_toggle_sign(false, "999");
        calc_input.try_add_decimal_pt();
        calc_input.try_add_digit(2, 10, false, "999", 64, 32);
        calc_input.try_begin_exponent();
        calc_input.try_add_digit(3, 10, false, "999", 64, 32);

        assert_eq!(
            "-1.2e+3",
            calc_input.to_string(10),
            "Verify input is correct."
        );

        calc_input.clear();

        //::Logger::WriteMessage(calc_input.to_string(10).c_str());
        assert_eq!(
            "0",
            calc_input.to_string(10),
            "Verify input is 0 after clear."
        );
    }

    fn try_toggle_sign_zero() {
        let mut calc_input = super::CalcInput::new();

        assert!(
            calc_input.try_toggle_sign(false, "999"),
            "Verify toggling 0 succeeds."
        );
        assert_eq!(
            "0",
            calc_input.to_string(10),
            "Verify toggling 0 does not create -0."
        );
    }

    fn try_toggle_sign_exponent() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_begin_exponent();
        calc_input.try_add_digit(2, 10, false, "999", 64, 32);
        assert!(
            calc_input.try_toggle_sign(false, "999"),
            "Verify toggling exponent sign succeeds."
        );
        assert_eq!(
            "1.e-2",
            calc_input.to_string(10),
            "Verify toggling exponent sign does not toggle base sign."
        );
        assert!(
            calc_input.try_toggle_sign(false, "999"),
            "Verify toggling exponent sign succeeds."
        );
        assert_eq!(
            "1.e+2",
            calc_input.to_string(10),
            "Verify toggling negative exponent sign does not toggle base sign."
        );
    }

    fn try_toggle_sign_base() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        assert!(
            calc_input.try_toggle_sign(false, "999"),
            "Verify toggling base sign succeeds."
        );
        assert_eq!(
            "-1",
            calc_input.to_string(10),
            "Verify toggling base sign creates negative base."
        );
        assert!(
            calc_input.try_toggle_sign(false, "999"),
            "Verify toggling base sign succeeds."
        );
        assert_eq!(
            "1",
            calc_input.to_string(10),
            "Verify toggling negative base sign creates positive base."
        );
    }

    fn try_toggle_sign_base_integer_mode() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        assert!(
            calc_input.try_toggle_sign(true, "999"),
            "Verify toggling base sign in integer mode succeeds."
        );
        assert_eq!(
            "-1",
            calc_input.to_string(10),
            "Verify toggling base sign creates negative base."
        );
    }

    fn try_toggle_sign_rollover() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_add_digit(2, 10, false, "999", 64, 32);
        assert!(
            calc_input.try_toggle_sign(true, "127"),
            "Verify toggling base sign in integer mode succeeds."
        );
        calc_input.try_add_digit(8, 10, false, "999", 64, 32);
        assert!(
            !calc_input.try_toggle_sign(true, "127"),
            "Verify toggling base sign in integer mode fails on rollover."
        );
        assert_eq!(
            "-128",
            calc_input.to_string(10),
            "Verify toggling base sign on rollover does not change value."
        );
    }

    fn try_add_digit_leading_zeroes() {
        let mut calc_input = super::CalcInput::new();

        assert!(
            calc_input.try_add_digit(0, 10, false, "999", 64, 32),
            "Verify try_add_digit succeeds."
        );
        assert!(
            calc_input.try_add_digit(0, 10, false, "999", 64, 32),
            "Verify try_add_digit succeeds."
        );
        assert!(
            calc_input.try_add_digit(0, 10, false, "999", 64, 32),
            "Verify try_add_digit succeeds."
        );
        assert_eq!(
            "0",
            calc_input.to_string(10),
            "Verify leading zeros are ignored."
        );
    }

    fn try_add_digit_max_count() {
        let mut calc_input = super::CalcInput::new();

        assert!(
            calc_input.try_add_digit(1, 10, false, "999", 64, 32),
            "Verify try_add_digit for base with length < maxDigits succeeds."
        );
        assert_eq!(
            "1",
            calc_input.to_string(10),
            "Verify adding digit for base with length < maxDigits succeeded."
        );
        assert!(
            !calc_input.try_add_digit(2, 10, false, "999", 64, 1),
            "Verify try_add_digit for base with length > maxDigits fails."
        );
        assert_eq!(
            "1",
            calc_input.to_string(10),
            "Verify digit for base was not added."
        );
        calc_input.try_begin_exponent();
        assert!(
            calc_input.try_add_digit(1, 10, false, "999", 64, 32),
            "Verify try_add_digit for exponent with length < maxDigits succeeds."
        );
        assert!(
            calc_input.try_add_digit(2, 10, false, "999", 64, 32),
            "Verify try_add_digit for exponent with length < maxDigits succeeds."
        );
        assert!(
            calc_input.try_add_digit(3, 10, false, "999", 64, 32),
            "Verify try_add_digit for exponent with length < maxDigits succeeds."
        );
        assert!(
            calc_input.try_add_digit(4, 10, false, "999", 64, 32),
            "Verify try_add_digit for exponent with length < maxDigits succeeds."
        );
        assert!(
            !calc_input.try_add_digit(5, 10, false, "999", 64, 32),
            "Verify try_add_digit for exponent with length > maxDigits fails."
        );
        assert_eq!(
            "1.e+1234",
            calc_input.to_string(10),
            "Verify adding digits for exponent with length < maxDigits succeeded."
        );

        calc_input.clear();
        calc_input.try_add_decimal_pt();
        assert!(
            calc_input.try_add_digit(1, 10, false, "999", 64, 1),
            "Verify decimal point and leading zero does not count toward maxDigits."
        );
        assert_eq!(
            "0.1",
            calc_input.to_string(10),
            "Verify input value checking dec pt and leading zero impact on maxDigits."
        );
    }

    fn try_add_digit_values() {
        let mut calc_input = super::CalcInput::new();

        // Use an arbitrary value > 16 to test that input accepts digits > hexadecimal 0xF.
        // try_add_digit does not validate whether the digit fits within the current radix.
        for i in 0..25 {
            assert!(
                calc_input.try_add_digit(i, 10, false, "999", 64, 32),
                format!("Verify try_add_digit succeeds for {}", i.to_string())
            );
            calc_input.clear();
        }
    }

    fn try_add_digit_rollover_base_check() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        assert!(
            !calc_input.try_add_digit(2, 16, true, "999", 64, 1),
            "Verify try_add_digit rollover fails for bases other than 8,10."
        );
        assert!(
            !calc_input.try_add_digit(1, 2, true, "999", 64, 1),
            "Verify try_add_digit rollover fails for bases other than 8,10."
        );
    }

    fn try_add_digit_rollover_octal_byte() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 8, true, "777", 64, 32);
        assert!(
            calc_input.try_add_digit(2, 8, true, "377", 8, 1),
            "Verify we can add an extra digit in OctalByte if first digit <= 3."
        );

        calc_input.clear();
        calc_input.try_add_digit(4, 8, true, "777", 64, 32);
        assert!(
            !calc_input.try_add_digit(2, 8, true, "377", 8, 1),
            "Verify we cannot add an extra digit in OctalByte if first digit > 3."
        );
    }

    fn try_add_digit_rollover_octal_word() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 8, true, "777", 64, 32);
        assert!(
            calc_input.try_add_digit(2, 8, true, "377", 16, 1),
            "Verify we can add an extra digit in OctalByte if first digit == 1."
        );

        calc_input.clear();
        calc_input.try_add_digit(2, 8, true, "777", 64, 32);
        assert!(
            !calc_input.try_add_digit(2, 8, true, "377", 16, 1),
            "Verify we cannot add an extra digit in OctalByte if first digit > 1."
        );
    }

    fn try_add_digit_rollover_octal_dword() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 8, true, "777", 64, 32);
        assert!(
            calc_input.try_add_digit(2, 8, true, "377", 32, 1),
            "Verify we can add an extra digit in OctalByte if first digit <= 3."
        );

        calc_input.clear();
        calc_input.try_add_digit(4, 8, true, "777", 64, 32);
        assert!(
            !calc_input.try_add_digit(2, 8, true, "377", 32, 1),
            "Verify we cannot add an extra digit in OctalByte if first digit > 3."
        );
    }

    fn try_add_digit_rollover_octal_qword() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 8, true, "777", 64, 32);
        assert!(
            calc_input.try_add_digit(2, 8, true, "377", 64, 1),
            "Verify we can add an extra digit in OctalByte if first digit == 1."
        );

        calc_input.clear();
        calc_input.try_add_digit(2, 8, true, "777", 64, 32);
        assert!(
            !calc_input.try_add_digit(2, 8, true, "377", 64, 1),
            "Verify we cannot add an extra digit in OctalByte if first digit > 1."
        );
    }

    fn try_add_digit_rollover_decimal() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, true, "127", 64, 32);
        assert!(
            !calc_input.try_add_digit(0, 10, true, "1", 8, 1),
            "Verify we cannot add a digit if input size matches maxStr size."
        );
        calc_input.try_add_digit(2, 10, true, "127", 64, 32);
        assert!(
            !calc_input.try_add_digit(2, 10, true, "110", 8, 2),
            "Verify we cannot add a digit if n char comparison > 0."
        );
        assert!(
            calc_input.try_add_digit(7, 10, true, "130", 8, 2),
            "Verify we can add a digit if n char comparison < 0."
        );

        calc_input.clear();
        calc_input.try_add_digit(1, 10, true, "127", 64, 32);
        calc_input.try_add_digit(2, 10, true, "127", 64, 32);
        assert!(
            !calc_input.try_add_digit(8, 10, true, "127", 8, 2),
            "Verify we cannot add a digit if digit exceeds max value."
        );
        assert!(
            calc_input.try_add_digit(7, 10, true, "127", 8, 2),
            "Verify we can add a digit if digit does not exceed max value."
        );

        calc_input.backspace();
        calc_input.try_toggle_sign(true, "127");
        assert!(
            !calc_input.try_add_digit(9, 10, true, "127", 8, 2),
            "Negative value: verify we cannot add a digit if digit exceeds max value."
        );
        assert!(
            calc_input.try_add_digit(8, 10, true, "127", 8, 2),
            "Negative value: verify we can add a digit if digit does not exceed max value."
        );
    }

    fn try_add_decimal_pt_empty() {
        let mut calc_input = super::CalcInput::new();

        assert!(
            !calc_input.has_decimal_pt(),
            "Verify input has no decimal point."
        );
        assert!(
            calc_input.try_add_decimal_pt(),
            "Verify adding decimal to empty input."
        );
        assert!(calc_input.has_decimal_pt(), "Verify input has decimal point.");
        assert_eq!(
            "0.",
            calc_input.to_string(10),
            "Verify decimal on empty input."
        );
    }

    fn try_add_decimal_point_twice() {
        let mut calc_input = super::CalcInput::new();

        assert!(
            !calc_input.has_decimal_pt(),
            "Verify input has no decimal point."
        );
        assert!(
            calc_input.try_add_decimal_pt(),
            "Verify adding decimal to empty input."
        );
        assert!(calc_input.has_decimal_pt(), "Verify input has decimal point.");
        assert!(
            !calc_input.try_add_decimal_pt(),
            "Verify adding decimal point fails if input has decimal point."
        );
    }

    fn try_add_decimal_point_exponent() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_begin_exponent();
        calc_input.try_add_digit(2, 10, false, "999", 64, 32);
        assert!(
            !calc_input.try_add_decimal_pt(),
            "Verify adding decimal point fails if input has exponent."
        );
    }

    fn try_begin_exponent_no_exponent() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        assert!(
            calc_input.try_begin_exponent(),
            "Verify adding exponent succeeds on input without exponent."
        );
        assert_eq!(
            "1.e+0",
            calc_input.to_string(10),
            "Verify exponent present."
        );
    }

    fn try_begin_exponent_with_exponent() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        assert!(
            calc_input.try_begin_exponent(),
            "Verify adding exponent succeeds on input without exponent."
        );
        assert!(
            !calc_input.try_begin_exponent(),
            "Verify cannot add exponent if input already has exponent."
        );
    }

    fn backspace_zero() {
        let mut calc_input = super::CalcInput::new();

        calc_input.backspace();
        assert_eq!(
            "0",
            calc_input.to_string(10),
            "Verify backspace on 0 is still 0."
        );
    }

    fn backspace_single_char() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        assert_eq!(
            "1",
            calc_input.to_string(10),
            "Verify input before backspace."
        );
        calc_input.backspace();
        assert_eq!(
            "0",
            calc_input.to_string(10),
            "Verify input after backspace."
        );
    }

    fn backspace_multi_char() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_add_digit(2, 10, false, "999", 64, 32);
        assert_eq!(
            "12",
            calc_input.to_string(10),
            "Verify input before backspace."
        );
        calc_input.backspace();
        assert_eq!(
            "1",
            calc_input.to_string(10),
            "Verify input after backspace."
        );
    }

    fn backspace_decimal() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_add_decimal_pt();
        assert_eq!(
            "1.",
            calc_input.to_string(10),
            "Verify input before backspace."
        );
        assert!(calc_input.has_decimal_pt(), "Verify input has decimal point.");
        calc_input.backspace();
        assert_eq!(
            "1",
            calc_input.to_string(10),
            "Verify input after backspace."
        );
        assert!(
            !calc_input.has_decimal_pt(),
            "Verify decimal point was removed."
        );
    }

    fn backspace_multi_char_decimal() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_add_decimal_pt();
        calc_input.try_add_digit(2, 10, false, "999", 64, 32);
        calc_input.try_add_digit(3, 10, false, "999", 64, 32);
        assert_eq!(
            "1.23",
            calc_input.to_string(10),
            "Verify input before backspace."
        );
        calc_input.backspace();
        assert_eq!(
            "1.2",
            calc_input.to_string(10),
            "Verify input after backspace."
        );
    }

    fn backspace_zero_decimal_without_prefix_zeros() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(0, 10, false, "999", 64, 32);
        calc_input.try_add_decimal_pt();
        assert_eq!(
            "0.",
            calc_input.to_string(10),
            "Verify input before backspace."
        );
        calc_input.backspace();
        calc_input.try_add_digit(0, 10, false, "999", 64, 32);
        assert_eq!(
            "0",
            calc_input.to_string(10),
            "Verify input after backspace."
        )
    }

    fn set_decimal_symbol() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_decimal_pt();
        assert_eq!(
            "0.",
            calc_input.to_string(10),
            "Verify default decimal point."
        );
        calc_input.set_decimal_symbol(',');
        assert_eq!("0,", calc_input.to_string(10), "Verify new decimal point.");
    }

    fn to_string_empty() {
        let mut calc_input = super::CalcInput::new();

        assert_eq!(
            "0",
            calc_input.to_string(10),
            "Verify to_string of empty value."
        );
    }

    fn to_string_negative() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_toggle_sign(false, "999");
        assert_eq!(
            "-1",
            calc_input.to_string(10),
            "Verify to_string of negative value."
        );
    }

    fn to_string_exponent_base10() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_begin_exponent();
        assert_eq!(
            "1.e+0",
            calc_input.to_string(10),
            "Verify to_string of empty base10 exponent."
        );
    }

    fn to_string_exponent_base8() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_begin_exponent();
        assert_eq!(
            "1.^+0",
            calc_input.to_string(8),
            "Verify to_string of empty base8 exponent."
        );
    }

    fn to_string_exponent_negative() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 8, false, "999", 64, 32);
        calc_input.try_begin_exponent();
        calc_input.try_toggle_sign(false, "999");
        assert_eq!(
            "1.e-0",
            calc_input.to_string(10),
            "Verify to_string of empty negative exponent."
        );
    }

    fn to_string_exponent_positive() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_begin_exponent();
        calc_input.try_add_digit(2, 10, false, "999", 64, 32);
        calc_input.try_add_digit(3, 10, false, "999", 64, 32);
        calc_input.try_add_digit(4, 10, false, "999", 64, 32);
        assert_eq!(
            "1.e+234",
            calc_input.to_string(10),
            "Verify to_string of exponent with value."
        );
    }

    fn to_string_integer() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        assert_eq!(
            "1",
            calc_input.to_string(10),
            "Verify to_string of integer value hides decimal."
        );
    }

    fn to_string_base_too_ong() {
        let mut calc_input = super::CalcInput::new();

        let mut max_str = String::new();

        for i in 0..super::MAX_STRLEN {
            max_str.push('1');
            calc_input.try_add_digit(1, 10, false, max_str.as_str(), 64, 100);
        }

        let result = calc_input.to_string(10);
        assert!(
            result.is_empty(),
            "Verify to_string of base value that is too large yields empty string."
        );
    }

    fn to_string_exponent_too_long() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_begin_exponent();

        let mut max_str = "11".to_owned();
        let mut exponent_capped = false;

        for i in 0..super::MAX_STRLEN + 1 {
            max_str.push('1');
            if !calc_input.try_add_digit(1, 10, false, &max_str, 64, super::MAX_STRLEN + 25) {
                exponent_capped = true;
            }
        }

        let result = calc_input.to_string(10);

        // try_add_digit caps the exponent length to C_EXP_MAX_DIGITS = 4, so to_string() succeeds.
        // If that cap is removed, to_string() should return an empty string.
        if exponent_capped {
            assert_eq!(
                "1.e+1111", result,
                "Verify to_string succeeds; exponent length is capped at C_EXP_MAX_DIGITS."
            );
        } else {
            assert!(
                result.is_empty(),
                "Verify to_string of exponent value that is too large yields empty string."
            );
        }
    }

    fn to_rational() {
        let mut calc_input = super::CalcInput::new();

        calc_input.try_add_digit(1, 10, false, "999", 64, 32);
        calc_input.try_add_digit(2, 10, false, "999", 64, 32);
        calc_input.try_add_digit(3, 10, false, "999", 64, 32);
        assert_eq!(
            "123",
            calc_input.to_string(10),
            "Verify input before conversion to rational."
        );

        let rat = calc_input.to_rational(10, crate::engine::CalculatorPrecision::Standard);
        //assert_eq!(1, rat.P().Mantissa().size(), "Verify digit count of rational.");
        //assert_eq!(123, rat.P().Mantissa().front(), "Verify first digit of mantissa.");
    }
}
