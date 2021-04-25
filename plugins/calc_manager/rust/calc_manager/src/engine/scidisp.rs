use super::{CalculatorEngine, NumWidth};
use lazy_static::lazy_static;
use num::{Rational, Zero};
use regex::Regex;

const MAX_EXPONENT: usize = 4;
const MAX_GROUPING_SIZE: usize = 16;
const DEC_PRE_SEP_STR: &str = "[+-]?(\\d*)[";
const DEC_POST_SEP_STR: &str = "]?(\\d*)(?:e[+-]?(\\d*))?$";

const IDS_ERR_UNK_CH: u64 = 1;
const ERR_INPUT_OVERFLOW: u64 = 0;

impl CalculatorEngine {
    pub fn truncate_num_for_int_math(&self, rational: Rational) {}

    pub fn display_num(&mut self) {
        if self.record
            || self.last_display.value != self.current_value
            || self.last_display.precision.is_some()
                && self.last_display.precision.unwrap() != self.precision
            || self.last_display.radix != self.radix
            || self.last_display.nFE.unwrap() != self.nFE // TODO
            || !self.last_display.use_sep
            || self.last_display.num_width.is_some()
                && self.last_display.num_width.unwrap() != self.num_width
            || self.last_display.int_math != self.interger_mode
            || self.record != self.record
        {
            self.last_display.precision = Some(self.precision);
            self.last_display.radix = self.radix;
            self.last_display.nFE = Some(self.nFE);
            self.last_display.num_width = Some(self.num_width);
            self.last_display.int_math = self.interger_mode;
            self.last_display.record = self.record;
            self.last_display.use_sep = true;

            if self.record {
                self.number_string = self.input.to_string(self.radix);
            } else {
                if self.interger_mode {
                  //  self.current_value = self.truncate_num_for_int_math(self.current_value); TODO
                }

                self.number_string = self.get_string_for_display(self.current_value, self.radix);
            }

            self.last_display.value = self.current_value;

            // if (self.radix == 10)
            //     && self.is_number_invalid(
            //         self.number_string,
            //         MAX_EXPONENT,
            //         self.precision,
            //         self.radix,
            //     )
            // {
            //     self.display_error(0x80000008); // CALC_E_OVERFLOW
            // } else {
            //     self.set_primary_display(
            //         self.group_digits_per_radix(self.number_string, self.radix),
            //         None,
            //     );
            // }
        }
    }

    pub fn is_number_invalid(
        &self,
        number_string: &str,
        max_exp: usize,
        max_matissa: usize,
        radix: u64,
    ) -> bool {
        let error = 0;

        if radix == 10 {
            // start with an optional + or -
            // followed by zero or more digits
            // followed by an optional decimal point
            // followed by zero or more digits
            // followed by an optional exponent
            // in case there's an exponent:
            //      its optionally followed by a + or -
            //      which is followed by zero or more digits
            let mut rx = DEC_PRE_SEP_STR.to_owned();
            rx.push(self.decimal_separator);
            rx += DEC_POST_SEP_STR;

            if let Ok(r) = Regex::new(&rx) {
                if r.is_match(number_string) {
                    for cap in r.captures_iter(number_string) {

                    }
                } else {
                    error = ERR_INPUT_OVERFLOW;   
                }

                let captures = r.captures(number_string).unwrap();
            }


        }

        false
    }

    // DigitGroupingStringToGroupingVector
    //
    // Description:
    //   This will take the digit grouping string found in the regional applet and
    //   represent this string as a vector.
    //
    //   groupingString
    //   0;0      - no grouping
    //   3;0      - group every 3 digits
    //   3        - group 1st 3, then no grouping after
    //   3;0;0    - group 1st 3, then no grouping after
    //   3;2;0    - group 1st 3 and then every 2 digits
    //   4;0      - group every 4 digits
    //   5;3;2;0  - group 5, then 3, then every 2
    //   5;3;2    - group 5, then 3, then 2, then no grouping after
    //
    // Returns: the groupings as a vector
    pub fn digit_grouping_string_to_grouping_vector(grouping_string: &str) -> Vec<u64> {
        Vec::new()
    }

    pub fn group_digits_per_radix(&self, number_string: &str, radix: u64) -> String {
        if number_string.is_empty() {
            String::new()
        } else {
            match radix {
                10 => self.group_digits(&self.group_separator.to_string(), &self.dec_grouping, number_string, Some(number_string.chars().nth(0).unwrap() == '-')),
                8 => self.group_digits(" ", &[3, 0], number_string, None),
                2 | 16 => self.group_digits(" ", &[4, 0], number_string, None),
                _ => number_string.to_owned(),
            }
        }
    }

    // GroupDigits
    //
    // Description:
    //   This routine will take a grouping vector and the display string and
    //   add the separator according to the pattern indicated by the separator.
    //
    //   Grouping
    //   0,0      - no grouping
    //   3,0      - group every 3 digits
    //   3        - group 1st 3, then no grouping after
    //   3,0,0    - group 1st 3, then no grouping after
    //   3,2,0    - group 1st 3 and then every 2 digits
    //   4,0      - group every 4 digits
    //   5,3,2,0  - group 5, then 3, then every 2
    //   5,3,2    - group 5, then 3, then 2, then no grouping after
    pub fn group_digits(
        &self,
        delimiter: &str,
        grouping: &[u64],
        display_string: &str,
        is_num_negative: Option<bool>,
    ) -> String {
        if delimiter.is_empty() || grouping.is_empty() {
            return String::new();
        }

        // Find the position of exponential 'e' in the string.
        let index =  if let Some(value) = display_string.find(self.decimal_separator) {
            value
        } else if let Some(value) = display_string.find('e') {
            value            
        } else {
            0
        };

        let mut result = String::new();
        let mut grouping_size = 0;

        let start = if is_num_negative.unwrap() { 1 } else { 0 }; // TODO

        result
    }
}

// #[cfg(test)]
// mod calculator_display_engine_tests {
//     #[test]
//     fn test_group_digits_per_radix() {
//         let mut calc_engine = super::CalculatorEngine::new(false, false, None, None);

//         // Empty/Error cases
//         assert!(
//             calc_engine.group_digits_per_radix("", 10).empty(),
//             "Verify grouping empty string returns empty string."
//         );
//         assert_eq!(
//             "12345678",
//             calc_engine.group_digits_per_radix("12345678", 9),
//             "Verify grouping on invalid base returns original string"
//         );

//         // Octal
//         assert_eq!(
//             "1 234 567",
//             calc_engine.group_digits_per_radix("1234567", 8),
//             "Verify grouping in octal."
//         );
//         assert_eq!(
//             "123",
//             calc_engine.group_digits_per_radix("123", 8),
//             "Verify minimum grouping in octal."
//         );

//         // Binary/Hexadecimal
//         assert_eq!(
//             "12 3456 7890",
//             calc_engine.group_digits_per_radix("1234567890", 2),
//             "Verify grouping in binary."
//         );
//         assert_eq!(
//             "1234",
//             calc_engine.group_digits_per_radix("1234", 2),
//             "Verify minimum grouping in binary."
//         );
//         assert_eq!(
//             "12 3456 7890",
//             calc_engine.group_digits_per_radix("1234567890", 16),
//             "Verify grouping in hexadecimal."
//         );
//         assert_eq!(
//             "1234",
//             calc_engine.group_digits_per_radix("1234", 16),
//             "Verify minimum grouping in hexadecimal."
//         );

//         // Decimal
//         assert_eq!(
//             "1,234,567,890",
//             calc_engine.group_digits_per_radix("1234567890", 10),
//             "Verify grouping in base10."
//         );
//         assert_eq!(
//             "1,234,567.89",
//             calc_engine.group_digits_per_radix("1234567.89", 10),
//             "Verify grouping in base10 with decimal."
//         );
//         assert_eq!(
//             "1,234,567e89",
//             calc_engine.group_digits_per_radix("1234567e89", 10),
//             "Verify grouping in base10 with exponent."
//         );
//         assert_eq!(
//             "1,234,567.89e5",
//             calc_engine.group_digits_per_radix("1234567.89e5", 10),
//             "Verify grouping in base10 with decimal and exponent."
//         );
//         assert_eq!(
//             "-123,456,789",
//             calc_engine.group_digits_per_radix("-123456789", 10),
//             "Verify grouping in base10 with negative."
//         );
//     }

//     #[test]
//     fn test_is_number_invalid() {
//         let history_display = None;
//         let mut calc_engine = super::CalculatorEngine::new(false, false, None, history_display);

//         // Binary Number Checks
//         let valid_bin_strs = vec!["0", "1", "0011", "1100"];
//         let invalid_bin_strs = vec!["2", "A", "0.1"];

//         for st in valid_bin_strs {
//             assert_eq!(0, calc_engine.is_number_invalid(st, 0, 0, 2));
//         }

//         for st in invalid_bin_strs {
//             assert_eq!(ERR_UNK_CH, calc_engine.is_number_invalid(st, 0, 0, 2));
//         }

//         // Octal Number Checks
//         let valid_oct_strs = vec!["0", "7", "01234567", "76543210"];
//         let invalid_oct_strs = ["8", "A", "0.7"];

//         for st in valid_oct_strs {
//             assert_eq!(0, calc_engine.is_number_invalid(st, 0, 0, 8));
//         }

//         for st in invalid_oct_strs {
//             assert_eq!(ERR_UNK_CH, calc_engine.is_number_invalid(st, 0, 0, 8));
//         }

//         // Hexadecimal Number Checks
//         let valid_hex_strs = ["0", "F", "0123456789ABCDEF", "FEDCBA9876543210"];
//         let invalid_hex_strs = vec!["G", "abcdef", "x", "0.1"];

//         for st in valid_hex_strs {
//             assert_eq!(0, calc_engine.is_number_invalid(st, 0, 0, 16 /* HEx */));
//         }

//         for st in invalid_hex_strs {
//             assert_eq!(ERR_UNK_CH, calc_engine.is_number_invalid(st, 0, 0, 16));
//         }

//         // Decimal Number Checks

//         // Special case errors: long exponent, long mantissa
//         let long_exp = "1e12345";
//         assert_eq!(0, calc_engine.is_number_invalid(long_exp, 5, 100, 10));
//         assert_eq!(
//             ERR_INPUT_OVERFLOW,
//             calc_engine.is_number_invalid(long_exp, 4, 100, 10)
//         );

//         // Mantissa length is sum of:
//         //  - digits before decimal separator, minus leading zeroes
//         //  - digits after decimal separator, including trailing zeroes
//         // Each of these mantissa values should calculate as a length of 5
//         let long_mant_strs = vec![
//             "10000",
//             "10.000",
//             "0000012345",
//             "123.45",
//             "0.00123",
//             "0.12345",
//             "-123.45e678",
//         ];

//         for st in long_mant_strs {
//             assert_eq!(0, calc_engine.is_number_invalid(st, 100, 5, 10));
//         }

//         for st in long_mant_strs {
//             assert_eq!(
//                 ERR_INPUT_OVERFLOW,
//                 calc_engine.is_number_invalid(st, 100, 4, 10)
//             );
//         }

//         // Regex matching (descriptions taken from CalcUtils.cpp)
//         // Use 100 for exp/mantissa length as they are tested above
//         let valid_dec_strs = [
//             // Start with an optional + or -
//             "+1",
//             "-1",
//             "1",
//             // Followed by zero or more digits
//             "-",
//             "",
//             "1234567890",
//             // Followed by an optional decimal point
//             "1.0",
//             "-.",
//             "1.",
//             // Followed by zero or more digits
//             "0.0",
//             "0.123456",
//             // Followed by an optional exponent ('e')
//             "1e",
//             "1.e",
//             "-e",
//             // If there's an exponent, its optionally followed by + or -
//             // and followed by zero or more digits
//             "1e+12345",
//             "1e-12345",
//             "1e123",
//             // All together
//             "-123.456e+789",
//         ];

//         let invalid_dec_strs = ["x123", "123-", "1e1.2", "1-e2"];

//         for st in valid_dec_strs {
//             assert_eq!(0, calc_engine.is_number_invalid(st, 100, 100, 10));
//         }

//         for st in invalid_dec_strs {
//             assert_eq!(
//                 ERR_UNK_CH,
//                 calc_engine.is_number_invalid(st, 100, 100, 10)
//             );
//         }
//     }

//     #[test]
//     fn test_digit_grouping_string_to_grouping_vector() {
//         let mut grouping_vector = [];
//         assert_eq!(
//             &grouping_vector,
//             super::CalculatorEngine::digit_grouping_string_to_grouping_vector(""),
//             "Verify empty grouping"
//         );

//         grouping_vector = [1];
//         assert_eq!(
//             &grouping_vector,
//             super::CalculatorEngine::digit_grouping_string_to_grouping_vector("1"),
//             "Verify simple grouping"
//         );

//         grouping_vector = [3, 0];
//         assert_eq!(
//             &grouping_vector,
//             super::CalculatorEngine::digit_grouping_string_to_grouping_vector("3;0"),
//             "Verify standard grouping"
//         );

//         grouping_vector = [3, 0, 0];
//         assert_eq!(
//             &grouping_vector,
//             super::CalculatorEngine::digit_grouping_string_to_grouping_vector("3;0;0"),
//             "Verify expanded non-repeating grouping"
//         );

//         grouping_vector = [5, 3, 2, 4, 6];
//         assert_eq!(
//             &grouping_vector,
//             super::CalculatorEngine::digit_grouping_string_to_grouping_vector("5;3;2;4;6"),
//             "Verify long grouping"
//         );

//         grouping_vector = [15, 15, 15, 0];
//         assert_eq!(
//             &grouping_vector,
//             super::CalculatorEngine::digit_grouping_string_to_grouping_vector("15;15;15;0"),
//             "Verify large grouping"
//         );

//         grouping_vector = [4, 7, 0];
//         assert_eq!(
//             &grouping_vector,
//             super::CalculatorEngine::digit_grouping_string_to_grouping_vector("4;16;7;25;0"),
//             "Verify we ignore oversize grouping"
//         );

//         grouping_vector = [3, 0];
//         let non_repeating_grouping = "3;0;0";
//         let repeating_grouping = non_repeating_grouping.substr(0, 3);
//         assert_eq!(
//             &grouping_vector,
//             super::CalculatorEngine::digit_grouping_string_to_grouping_vector(repeating_grouping),
//             "Verify we don't go past the end of wstring_view range"
//         );
//     }

//     #[test]
//     fn test_group_digits() {
//         let history_display = None;
//         let mut calc_engine = super::CalculatorEngine::new(false, false, None, history_display);

//         assert_eq!(
//             "1234567",
//             calc_engine.group_digits("", &[3, 0], "1234567", false),
//             "Verify handling of empty delimiter."
//         );

//         assert_eq!(
//             "1234567",
//             calc_engine.group_digits(",", &[], "1234567", false),
//             "Verify handling of empty grouping."
//         );

//         assert_eq!(
//             "1,234,567",
//             calc_engine.group_digits(",", &[3, 0], "1234567", false),
//             "Verify standard digit grouping."
//         );

//         assert_eq!(
//             "1 234 567",
//             calc_engine.group_digits(" ", &[3, 0], "1234567", false),
//             "Verify delimiter change."
//         );

//         assert_eq!(
//             "1|||234|||567",
//             calc_engine.group_digits("|||", &[3, 0], "1234567", false),
//             "Verify long delimiter."
//         );

//         assert_eq!(
//             "12,345e67",
//             calc_engine.group_digits(",", &[3, 0], "12345e67", false),
//             "Verify respect of exponent."
//         );

//         assert_eq!(
//             "12,345.67",
//             calc_engine.group_digits(",", &[3, 0], "12345.67", false),
//             "Verify respect of decimal."
//         );

//         assert_eq!(
//             "1,234.56e7",
//             calc_engine.group_digits(",", &[3, 0], "1234.56e7", false),
//             "Verify respect of exponent and decimal."
//         );

//         assert_eq!(
//             "-1,234,567",
//             calc_engine.group_digits(",", &[3, 0], "-1234567", true),
//             "Verify negative number grouping."
//         );

//         // Test various groupings
//         assert_eq!(
//             "1234567890123456",
//             calc_engine.group_digits(",", &[0, 0], "1234567890123456", false),
//             "Verify no grouping."
//         );

//         assert_eq!(
//             "1234567890123,456",
//             calc_engine.group_digits(",", &[3], "1234567890123456", false),
//             "Verify non-repeating grouping."
//         );

//         assert_eq!(
//             "1234567890123,456",
//             calc_engine.group_digits(",", &[3, 0, 0], "1234567890123456", false),
//             "Verify expanded form non-repeating grouping."
//         );

//         assert_eq!(
//             "12,34,56,78,901,23456",
//             calc_engine.group_digits(",", &[5, 3, 2, 0], "1234567890123456", false),
//             "Verify multigroup with repeating grouping."
//         );

//         assert_eq!(
//             "1234,5678,9012,3456",
//             calc_engine.group_digits(",", &[4, 0], "1234567890123456", false),
//             "Verify repeating non-standard grouping."
//         );

//         assert_eq!(
//             "123456,78,901,23456",
//             calc_engine.group_digits(",", &[5, 3, 2], "1234567890123456", false),
//             "Verify multigroup non-repeating grouping."
//         );
        
//         assert_eq!(
//             "123456,78,901,23456",
//             calc_engine.group_digits(",", &[5, 3, 2, 0, 0], "1234567890123456", false),
//             "Verify expanded form multigroup non-repeating grouping."
//         );
//     }
// }