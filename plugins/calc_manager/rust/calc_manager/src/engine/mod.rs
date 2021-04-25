use crate::engine_strings::EngineStringsId;
use command::Command;
use num::*;
use std::cell::RefCell;
use std::rc::Rc;

use {
    calc_input::CalcInput, history::HistoryCollector, history_display::HistoryDisplay,
    radix_type::RadixType,
};

mod calc_input;
mod constants;
mod history;
mod scicomm;
mod scidisp;
mod scifunc;
mod scioper;
mod sciset;

pub mod calc_display;
pub mod command;
pub mod expression_command;
pub mod history_display;
pub mod radix_type;

const DEFAULT_MAX_DIGITS: usize = 32;
const DEFAULT_PRECISION: CalculatorPrecision = CalculatorPrecision::Scientific;
const DEFAULT_RADIX: u64 = 10;

pub const DEFAULT_DEC_SEPARATOR: char = '.';
pub const DEFAULT_GRP_SEPARATOR: char = ',';
const DEFAULT_GRP_STR: &str = "3;0";
const DEFAULT_NUMBER_STR: &str = "0";

pub const MAX_PREC_DEPTH: usize = 25;
const NUM_WIDTH_LENGTH: usize = 4;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum NumWidth {
    Qword,
    Dword,
    Word,
    Byte,
}

impl NumWidth {
    pub fn bit_width(self) -> usize {
        match self {
            NumWidth::Qword => 64,
            NumWidth::Dword => 32,
            NumWidth::Word => 16,
            NumWidth::Byte => 8,
        }
    }

    fn get_index(self) -> usize {
        match self {
            Qword => 0,
            Dword => 1,
            Word => 2,
            Byte => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum AngleType {
    Degress,
    Radians,
    Gradians,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum NumberFormat {
    Float,
    Scientific,
    Engineering,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CalculatorPrecision {
    Standard = 16,
    Scientific = 32,
    Programmer = 64,
}

// struct CalcInput {
//     dec_symbol: String,
// }

// impl CalcInput {
//     fn new(dec_symbol: &str) -> CalcInput {
//         CalcInput {
//             dec_symbol: String::from(dec_symbol),
//         }
//     }
// }

struct LastDisp {
    value: Rational,
    precision: Option<CalculatorPrecision>,
    radix: u64,
    nFE: Option<NumberFormat>,
    num_width: Option<NumWidth>,
    int_math: bool,
    record: bool,
    use_sep: bool,
}

impl LastDisp {
    fn new() -> Self {
LastDisp {
        value: Rational::zero(),
        precision: None,
        radix: 0,
        nFE: None,
        num_width: None,
        int_math: false,
        record: false,
        use_sep: false,
    }
    }
}

pub struct CalculatorEngine {
    precedence: bool,
    interger_mode: bool,
    calc_display: Option<Rc<RefCell<dyn calc_display::CalcDisplay>>>,
    //resource_provider: Box<dyn ResourceProvider>,
    command: Command, // Operation id.
    previous_command: Command,
    change_commad: bool,          // Flag for changing operation.
    record: bool,                 // Global mode: recording or displaying.
    set_calc_state: bool,         // Flag for setting the engine result state.
    input: calc_input::CalcInput, // Global calc input object for decimal strings.
    nFE: NumberFormat,            // Scientific notation conversion flag.
    memory_value: Box<Rational>,  // Current memory value.
    max_trigonometric_num: Rational,
    hold_value: Rational, // For holding the second operand in repetitive calculations.
    current_value: Rational, // Currently displayed number used everywhere.
    last_value: Rational, // Number before operation (left operand).
    paren_vals: [Rational; MAX_PREC_DEPTH], // Holding array for parenthesis values.
    precedence_vals: [Rational; MAX_PREC_DEPTH], // Holding array for precedence values.
    error: bool,          // Error flag.
    inv: bool,            // Inverse on/off flag.
    no_previous_equal: bool, // Flag for previous equals.
    radix: u64,
    precision: CalculatorPrecision,
    int_digits_sav: usize,
    dec_grouping: Vec<u64>, // Holds the decimal digit grouping number.
    number_string: String,
    temp_command: Command,               // Holding place for the last command.
    open_parentheses_count: usize,       // Number of open parentheses.
    commands: [Command; MAX_PREC_DEPTH], // Holding array for parenthesis operations.
    precedence_commands: [Command; MAX_PREC_DEPTH], // Holding array for precedence operations.
    precedence_command_count: usize,     // Current number of precedence ops in holding.
    last_command: Command,               // Last command entered.
    angle_type: AngleType,               // Current Angle type when in dec mode.
    num_width: NumWidth,                 // Number of bits in currently selected word size.
    bit_width: usize,
    carry_bit: u64,
    history_collector: history::HistoryCollector,
    chop_numbers: [Rational; NUM_WIDTH_LENGTH],
    max_decimal_value_strings: Vec<String>,
    decimal_separator: char,
    group_separator: char,
    last_display: LastDisp,
    //    random_generator_engine: i32,
    //  distr: i32,
}

impl CalculatorEngine {
    pub fn new(
        precedence: bool,
        integer_mode: bool,
        calc_display: Option<Rc<RefCell<dyn calc_display::CalcDisplay>>>,
        history_display: Option<Rc<RefCell<dyn HistoryDisplay>>>,
    ) -> Self {
        let h_calc_display = calc_display.as_ref().unwrap().clone();

        let history_collector =
            HistoryCollector::new(Some(h_calc_display), history_display, DEFAULT_DEC_SEPARATOR);

        let mut calc_engine = Self {
            angle_type: AngleType::Degress,
            calc_display: calc_display,
            carry_bit: 0,
            chop_numbers: [Rational::zero(); NUM_WIDTH_LENGTH],
            current_value: Rational::zero(),
            dec_grouping: Vec::new(),
            decimal_separator: ' ',
            bit_width: NumWidth::bit_width(NumWidth::Qword),
            error: false,
            group_separator: DEFAULT_GRP_SEPARATOR,
            history_collector: history_collector,
            hold_value: Rational::zero(),
            input: CalcInput::new_with_symbol(DEFAULT_DEC_SEPARATOR),
            int_digits_sav: DEFAULT_MAX_DIGITS,
            interger_mode: precedence,
            inv: false,
            last_command: Command::Nop,
            last_value: Rational::zero(),
            max_decimal_value_strings: Vec::new(),
            max_trigonometric_num: Rational::zero(), // TODO
            memory_value: Box::new(Rational::zero()),
            temp_command: Command::Nop,
            nFE: NumberFormat::Float,
            no_previous_equal: true,
            num_width: NumWidth::Qword,
            number_string: String::from(DEFAULT_NUMBER_STR),
            command: Command::Nop,
            open_parentheses_count: 0,
            paren_vals: [Rational::zero(); MAX_PREC_DEPTH],
            precedence_vals: [Rational::zero(); MAX_PREC_DEPTH],
            precedence: integer_mode,
            precision: DEFAULT_PRECISION,
            radix: DEFAULT_RADIX,
            record: false,
            //resource_provider: Box::new(),
            set_calc_state: false,
            change_commad: false,
            previous_command: Command::Nop,
            commands: [Command::Nop; MAX_PREC_DEPTH],
            precedence_command_count: 0,
            precedence_commands: [Command::Nop; MAX_PREC_DEPTH],
            last_display: LastDisp::new(),
        };

        calc_engine.init_chop_numbers();

        calc_engine
            .set_radix_type_and_num_width(Some(RadixType::Decimal), Some(calc_engine.num_width));
        calc_engine.settings_changed();
        calc_engine.display_num();

        calc_engine
    }

    pub fn get_string(id: EngineStringsId) -> String {
        String::new()
    }

    pub fn command_to_string(command: command::Command) -> &'static str {
        EngineStringsId::get_string(Self::engine_id_from_command(command))
    }

    fn engine_id_from_command(command: command::Command) -> EngineStringsId {
        // TODO
        EngineStringsId::LogBaseY
    }

    fn init_chop_numbers(&mut self) {
        assert!(self.chop_numbers.len() >= 4);
        self.chop_numbers[0] = Rational::from_u64(std::u64::MAX).unwrap();
        self.chop_numbers[1] = Rational::from_u32(std::u32::MAX).unwrap();
        self.chop_numbers[1] = Rational::from_u16(std::u16::MAX).unwrap();
        self.chop_numbers[1] = Rational::from_u8(std::u8::MAX).unwrap();

        // Initialize the max dec number you can support for each of the supported bit lengths.
        // This is basically max num in that width / 2 in integer.
        //assert!(self.chop_numbers.len() == self.max_decimal_value_strings.len());
        for (p, chop) in self.chop_numbers.iter().enumerate() {
            let max_val: Rational = chop / 2;

            // TODO
            self.max_decimal_value_strings.push(max_val.to_string());
        }
    }

    pub fn get_chop_numbers(&self) -> Rational {
        self.chop_numbers[self.num_width.get_index()]
    }

    pub fn get_max_decimal_value_string(&self) -> &str {
        self.max_decimal_value_strings[self.num_width.get_index()].as_ref()
    }

    pub fn set_persisted_mem_object(&mut self, mem_object: Rational) {
        self.memory_value = Box::new(mem_object);
    }

    pub fn get_persisted_mem_object(&self) -> Option<Box<Rational>> {
        Some(self.memory_value.clone())
    }

    pub fn get_decimal_separator(&self) -> char {
        self.decimal_separator
    }

    pub fn settings_changed(&mut self) {
        let last_dec = self.decimal_separator;
        // TODO let dec_str: &str = self.resource_provider.get_cengine_string("sDecimal");
        let dec_str = "";

        self.decimal_separator = if dec_str.is_empty() {
            DEFAULT_DEC_SEPARATOR
        } else {
            dec_str.chars().nth(0).unwrap()
        };
        // Until it can be removed, contine to set decimal here
        // TODO
        // set_decimal_separator();

        let last_sep = self.group_separator;
        //TODO let sep_str: &str = self.resource_provider.get_cengine_string("sThousand");
        let sep_str = "";

        self.group_separator = if sep_str.is_empty() {
            DEFAULT_GRP_SEPARATOR
        } else {
            sep_str.chars().nth(0).unwrap()
        };

        //let last_dec_grouping = self.dec_grouping;
        //TODO let mut grp_str: String = self.resource_provider.get_cengine_string("sGrouping");
        let mut grp_str = "";

        if grp_str.is_empty() {
            grp_str = DEFAULT_GRP_STR;
        }

        //self.dec_grouping = self.digit_grouping_string_to_grouping_vector(grp_str); // TODO

        let mut num_changed = true;

        //num_changed = self.dec_grouping != last_dec_grouping || self.group_separator != last_sep;

        if self.decimal_separator != last_dec {
            // Re-initialize member variables' decimal point.
            self.input.set_decimal_symbol(self.decimal_separator);
            self.history_collector
                .set_decimal_symbol(self.decimal_separator);

            // Put the new decimal symbol into the table used to draw the decimal key
            // TODO
            //engine_strings[SIDS_DECIMAL_SEPARATOR] = self.decimal_separator;

            num_changed = true;
        }

        if num_changed {
            self.display_num();
        }
    }

    pub fn in_error_state(&self) -> bool {
        self.error
    }

    pub fn is_input_empry(&self) -> bool {
        self.input.is_empty() && (self.number_string.is_empty() || self.number_string == "0")
    }

    pub fn in_recordind_state(&self) -> bool {
        self.record
    }

    pub fn change_precision(&mut self, value: CalculatorPrecision) {
        self.precision = value;
        // changeconstants
    }
}
