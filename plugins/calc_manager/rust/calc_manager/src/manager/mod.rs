use super::engine::{
    calc_display::CalcDisplay, command::Command, expression_command::ExpCommand,
    radix_type::RadixType, AngleType, CalculatorEngine, CalculatorPrecision,
};
use history::{CalculatorHistory, HistoryItem};
use num::{Rational, Zero};
use std::cell::RefCell;
use std::rc::Rc;

mod history;
pub mod number_formatting;

const MAX_MEMORY_SIZE: usize = 100;
const MAX_HISTORY_ITEMS: usize = 20;

#[derive(Eq, PartialEq)]
enum CalculatorMode {
    Standard,
    Scientific,
}

pub struct CalculatorManager {
    calc_display: Rc<RefCell<dyn CalcDisplay>>,
    current_calculator_engine: Option<Rc<RefCell<CalculatorEngine>>>,
    scientific_calculator_engine: Option<Rc<RefCell<CalculatorEngine>>>,
    stantard_calculator_engine: Option<Rc<RefCell<CalculatorEngine>>>,
    programmer_calculator_engine: Option<Rc<RefCell<CalculatorEngine>>>,
    in_history_item_load_mode: bool,

    memorized_numbers: Vec<Rational>,
    persisted_primary_value: Rational,
    is_exponential_format: bool,
    current_degree_mode: AngleType,

    std_history: Rc<RefCell<CalculatorHistory>>,
    sci_history: Rc<RefCell<CalculatorHistory>>,
    history: Option<RefCell<Rc<CalculatorHistory>>>,
}

// impl CalcDisplay for CalculatorManager {
//     fn set_primary_display(&mut self, text: &str, is_error: bool) {
//         if !self.in_history_item_load_mode {
//             self.calc_display
//                 .borrow_mut()
//                 .set_primary_display(text, is_error);
//         }
//     }

//     fn set_is_error(&mut self, value: bool) {
//         self.calc_display.borrow_mut().set_is_error(value);
//     }

//     fn set_expression_display(
//         &mut self,
//         tokens: &[(String, Option<usize>)],
//         commands: &[ExpCommand],
//     ) {
//         if !self.in_history_item_load_mode {
//             self.calc_display
//                 .borrow_mut()
//                 .set_expression_display(tokens, commands);
//         }
//     }

//     fn set_parenthesis_number(&mut self, count: usize) {
//         self.calc_display.borrow_mut().set_parenthesis_number(count);
//     }

//     fn on_no_right_paren_added(&self) {
//         self.calc_display.borrow().on_no_right_paren_added()
//     }

//     fn max_digits_reached(&self) {
//         self.calc_display.borrow().max_digits_reached()
//     }

//     fn binary_operation_received(&self) {
//         self.calc_display.borrow().max_digits_reached()
//     }

//     fn on_history_item_added(&self, added_item_index: usize) {
//         self.calc_display
//             .borrow()
//             .on_history_item_added(added_item_index);
//     }

//     fn set_memorized_numbers(&mut self, memorized_numbers: Vec<String>) {
//         self.calc_display
//             .borrow_mut()
//             .set_memorized_numbers(memorized_numbers)
//     }

//     fn memory_item_changed(&self, index: usize) {
//         self.memory_item_changed(index);
//     }

//     fn input_changed(&self) {
//         self.calc_display.borrow().input_changed()
//     }
// }

impl CalculatorManager {
    pub fn new(calc_display: Rc<RefCell<dyn CalcDisplay>>) -> Self {
        Self {
            calc_display,
            current_calculator_engine: None,
            current_degree_mode: AngleType::Degress,
            history: None,
            in_history_item_load_mode: false,
            is_exponential_format: false,
            persisted_primary_value: Rational::zero(),
            programmer_calculator_engine: None,
            scientific_calculator_engine: None,
            stantard_calculator_engine: None,
            sci_history: Rc::new(RefCell::new(CalculatorHistory::new(MAX_HISTORY_ITEMS))),
            std_history: Rc::new(RefCell::new(CalculatorHistory::new(MAX_HISTORY_ITEMS))),
            memorized_numbers: Vec::new(),
        }
    }

    pub fn reset(&mut self, clear_memory: Option<bool>) {
        self.set_standard_mode();

        if let Some(scientific_calculator_engine) = self.scientific_calculator_engine.as_mut() {
            scientific_calculator_engine
                .borrow_mut()
                .process_command(Command::Deg);
            scientific_calculator_engine
                .borrow_mut()
                .process_command(Command::Clear);

            if self.is_exponential_format {
                self.is_exponential_format = false;
                scientific_calculator_engine
                    .borrow_mut()
                    .process_command(Command::Fe);
            }
        }

        if let Some(programmer_calculator_engine) = self.programmer_calculator_engine.as_mut() {
            programmer_calculator_engine
                .borrow_mut()
                .process_command(Command::Clear);
        }

        if clear_memory.unwrap_or(true) {
            self.memorized_number_clear_all();
        }
    }

    pub fn send_command(&mut self, command: Command) {
        match command {
            Command::ModeBasic => self.set_standard_mode(),
            Command::ModeScientific => self.set_standard_mode(),
            Command::ModeProgrammer => self.set_programmer_mode(),
            Command::Deg | Command::Rad | Command::Grad => {
                self.current_degree_mode = command.get_angle_type().unwrap();
            }
            _ => {
                let mut current_calculator_engine = self
                    .current_calculator_engine
                    .as_mut()
                    .unwrap()
                    .borrow_mut();

                match command {
                    Command::Asin => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Sin);
                    }
                    Command::Acos => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Cos);
                    }
                    Command::Atan => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Tan);
                    }
                    Command::Powe => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Ln);
                    }
                    Command::Asinh => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Sinh);
                    }
                    Command::Acosh => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Cosh);
                    }
                    Command::Atanh => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Tanh);
                    }
                    Command::Asec => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Sec);
                    }
                    Command::Acsc => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Csc);
                    }
                    Command::Acot => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Acot);
                    }
                    Command::Asech => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Sech);
                    }
                    Command::Acsch => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Csch);
                    }
                    Command::Acoth => {
                        current_calculator_engine.process_command(Command::Inv);
                        current_calculator_engine.process_command(Command::Coth);
                    }
                    Command::Fe => {
                        self.is_exponential_format = !self.is_exponential_format;
                        current_calculator_engine.process_command(command);
                    }
                    _ => current_calculator_engine.process_command(command),
                }
            }
        }

        self.calc_display.borrow_mut().input_changed();
    }

    pub fn memorize_number(&mut self) {
        let mut current_calculator_engine = self
            .current_calculator_engine
            .as_mut()
            .unwrap()
            .borrow_mut();

        if !current_calculator_engine.in_error_state() {
            current_calculator_engine.process_command(Command::Store);

            let memory_object = current_calculator_engine.get_persisted_mem_object();

            if let Some(mobj) = memory_object {
                self.memorized_numbers.push(*mobj);
            }

            if self.memorized_numbers.len() > MAX_MEMORY_SIZE {
                self.memorized_numbers
                    .resize_with(MAX_MEMORY_SIZE, || Rational::zero());
            }

            self.set_memorized_numbers_string();
        }
    }

    pub fn memorized_number_load(&mut self, index: usize) {
        let current_calculator_engine = self
            .current_calculator_engine
            .as_mut()
            .unwrap()
            .borrow_mut();

        if !current_calculator_engine.in_error_state() {
            self.memorized_number_select(index);
            current_calculator_engine.process_command(Command::Recall);
            self.calc_display.borrow().input_changed();
        }
    }

    pub fn memorized_number_add(&mut self, index: usize) {
        let current_calculator_engine = self
            .current_calculator_engine
            .as_mut()
            .unwrap()
            .borrow_mut();

        if !current_calculator_engine.in_error_state() {
            if self.memorized_numbers.is_empty() {
                self.memorize_number();
            } else {
                self.memorized_number_select(index);
                current_calculator_engine.process_command(Command::MPlus);

                self.memorized_number_changed(index);
                self.set_memorized_numbers_string();
            }

            self.calc_display.borrow_mut().memory_item_changed(index);
        }
    }

    pub fn memorized_number_subtract(&mut self, index: usize) {
        let current_calculator_engine = self
            .current_calculator_engine
            .as_mut()
            .unwrap()
            .borrow_mut();

        if !current_calculator_engine.in_error_state() {
            if self.memorized_numbers.is_empty() {
                self.memorize_number();
                self.memorized_number_subtract(0);
                self.memorized_number_subtract(0);
            } else {
                self.memorized_number_select(index);
                current_calculator_engine.process_command(Command::MMinus);

                self.memorized_number_changed(index);
                self.set_memorized_numbers_string();
            }

            self.calc_display.borrow_mut().memory_item_changed(index);
        }
    }

    pub fn memorized_number_clear(&mut self, index: usize) {
        if index < self.memorized_numbers.len() {
            self.memorized_numbers.remove(index);
        }
    }

    pub fn memorized_number_clear_all(&mut self) {
        self.memorized_numbers.clear();

        let current_calculator_engine = self
            .current_calculator_engine
            .as_mut()
            .unwrap()
            .borrow_mut();

        current_calculator_engine.process_command(Command::Clear);
        self.set_memorized_numbers_string();
    }

    pub fn is_engine_recording(&self) -> bool {
        let current_calculator_engine = self.current_calculator_engine.as_ref().unwrap().borrow();

        current_calculator_engine.in_recordind_state()
    }

    pub fn is_input_empty(&self) -> bool {
        let current_calculator_engine = self.current_calculator_engine.as_ref().unwrap().borrow();

        current_calculator_engine.is_input_empry()
    }

    pub fn set_radix(&mut self, radix_type: RadixType) {
        let current_calculator_engine = self
            .current_calculator_engine
            .as_mut()
            .unwrap()
            .borrow_mut();

        match radix_type {
            RadixType::Hexadecimal => current_calculator_engine.process_command(Command::Hex),
            RadixType::Decimal => current_calculator_engine.process_command(Command::Dec),
            RadixType::Octal => current_calculator_engine.process_command(Command::Oct),
            RadixType::Binary => current_calculator_engine.process_command(Command::Bin),
        }
        self.set_memorized_numbers_string();
    }

    pub fn set_memorized_numbers_string(&mut self) {
        let current_calculator_engine = self
            .current_calculator_engine
            .as_mut()
            .unwrap()
            .borrow_mut();
        let mut result = Vec::new();

        for memory_item in self.memorized_numbers.iter() {
            let radix = current_calculator_engine.get_current_radix();
            let value = current_calculator_engine.get_string_for_display(*memory_item, radix);

            if !value.is_empty() {
                result
                    .push(current_calculator_engine.group_digits_per_radix(value.as_str(), radix));
            }
        }

        self.calc_display.borrow_mut().set_memorized_numbers(result);
    }

    pub fn get_result_for_radix(
        &self,
        radix: u64,
        precision: u64,
        group_digits_per_radix: bool,
    ) -> String {
        if let Some(current_calculator_engine) = self.current_calculator_engine {
            current_calculator_engine
                .borrow()
                .get_current_result_for_radix(radix, precision, group_digits_per_radix)
        } else {
            "".to_owned()
        }
    }

    pub fn set_precision(&mut self, precision: CalculatorPrecision) {
        let current_calculator_engine = self
            .current_calculator_engine
            .as_mut()
            .unwrap()
            .borrow_mut();

        current_calculator_engine.change_precision(precision);
    }

    pub fn update_max_int_digits(&mut self) {
        let current_calculator_engine = self
            .current_calculator_engine
            .as_mut()
            .unwrap()
            .borrow_mut();

        current_calculator_engine.update_max_int_digits();
    }

    pub fn decimal_separator(&self) -> char {
        if let Some(current_calculator_engine) = self.current_calculator_engine.as_ref() {
            current_calculator_engine.borrow().get_decimal_separator()
        } else {
            crate::engine::DEFAULT_DEC_SEPARATOR
        }
    }

    pub fn get_history_items(&self) -> &[HistoryItem] {
        self.history.as_ref().unwrap().borrow().get_history()
    }

    pub fn get_history_items_with_mode(&self, mode: CalculatorMode) -> Vec<HistoryItem> {
        if mode == CalculatorMode::Standard {
            self.std_history.borrow().get_history().to_vec()
        } else {
            self.sci_history.borrow().get_history().to_vec()
        }
    }

    pub fn remove_history_item(&mut self, index: usize) {
        self.history.unwrap().borrow().remove_item(index);
    }

    pub fn clear_history(&mut self) {
        self.history.unwrap().borrow().clear_history();
    }

    pub fn max_history_size(&self) -> usize {
        self.history.unwrap().borrow().max_history_size()
    }
    pub fn get_current_degree_mode(&self) -> AngleType {
        self.current_degree_mode
    }

    pub fn set_in_history_item_load_mode(&mut self, is_history_item_load_mode: bool) {
        self.in_history_item_load_mode = is_history_item_load_mode;
    }
}

impl CalculatorManager {
    fn set_standard_mode(&mut self) {
        let standard_calculator_engine = self.stantard_calculator_engine.get_or_insert_with(|| {
            Rc::new(RefCell::new(CalculatorEngine::new(
                false,
                false,
                Some(self.calc_display.clone()),
                Some(self.std_history.clone()),
            )))
        });

        self.current_calculator_engine = Some(standard_calculator_engine.clone());
        standard_calculator_engine
            .borrow_mut()
            .process_command(Command::Dec);
        standard_calculator_engine
            .borrow_mut()
            .process_command(Command::Clear);
        standard_calculator_engine
            .borrow_mut()
            .change_precision(CalculatorPrecision::Standard);
        self.update_max_int_digits();
       // self.history = Some(self.std_history.clone());
    }

    fn set_scientific_mode(&mut self) {
        let scientific_mode_engine = self.scientific_calculator_engine.get_or_insert_with(|| {
            Rc::new(RefCell::new(CalculatorEngine::new(
                true,
                false,
                Some(self.calc_display.clone()),
                Some(self.sci_history.clone()),
            )))
        });

        self.current_calculator_engine = Some(scientific_mode_engine.clone());
        scientific_mode_engine
            .borrow_mut()
            .process_command(Command::Dec);
        scientific_mode_engine
            .borrow_mut()
            .process_command(Command::Clear);
        scientific_mode_engine
            .borrow_mut()
            .change_precision(CalculatorPrecision::Scientific);
        //self.history = Some(self.sci_history.clone());
    }

    fn set_programmer_mode(&mut self) {
        let programmer_calculator_engine =
            self.programmer_calculator_engine.get_or_insert_with(|| {
                Rc::new(RefCell::new(CalculatorEngine::new(
                    true,
                    true,
                    Some(self.calc_display.clone()),
                    None,
                )))
            });

        self.current_calculator_engine = Some(programmer_calculator_engine.clone());
        programmer_calculator_engine
            .borrow_mut()
            .process_command(Command::Dec);
        programmer_calculator_engine
            .borrow_mut()
            .process_command(Command::Clear);
        programmer_calculator_engine
            .borrow_mut()
            .change_precision(CalculatorPrecision::Programmer);
        self.history = None;
    }

    fn memorized_number_select(&mut self, index: usize) {
        let mut current_calculator_engine = self.current_calculator_engine.unwrap().borrow_mut();

        if !current_calculator_engine.in_error_state() {
            let value = self.memorized_numbers[index];
            current_calculator_engine.set_persisted_mem_object(value);
        }
    }

    fn memorized_number_changed(&mut self, index: usize) {
        let current_calculator_engine = self.current_calculator_engine.unwrap().borrow();

        if !current_calculator_engine.in_error_state() {
            if let Some(value) = current_calculator_engine.get_persisted_mem_object() {
                self.memorized_numbers[index] = *value;
            }
        }
    }

    fn load_persistent_primary_value(&mut self) {
        let mut current_calculator_engine = self
            .current_calculator_engine
            .as_mut()
            .unwrap()
            .borrow_mut();

        current_calculator_engine.set_persisted_mem_object(self.persisted_primary_value);
        current_calculator_engine.process_command(Command::Recall);
        self.calc_display.borrow_mut().input_changed();
    }
}
