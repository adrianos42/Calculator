use super::command::*;
use super::expression_command::*;
use super::AngleType;
use super::CalculatorEngine;
use super::{calc_display::CalcDisplay, history_display::HistoryDisplay};
use num::Rational;
use std::rc::Rc;
use std::cell::RefCell;

// Maximum depth you can get by precedence. It is just an array's size limit.
pub const MAXPRECDEPTH: usize = 25;

pub struct HistoryCollector {
    history_display: Option<Rc<RefCell<dyn HistoryDisplay>>>,
    calc_display: Option<Rc<RefCell<dyn CalcDisplay>>>,
    cur_line_hist_start: Option<usize>, // Index of the beginning of the current equation.
    last_op_start_index: Option<usize>, // Index of the beginning of the last operand added to the history.
    last_bin_op_start_index: Option<usize>, // Index of the beginning of the last binary operator added to the history
    operand_indices: [Option<usize>; MAXPRECDEPTH], // Stack of index of opnd's beginning for each '('.
    cur_operand_index: Option<usize>,               // Stack index for the above stack
    last_opnd_brace: bool, // If the last opnd in history is already braced so we can avoid putting another one for unary operator.
    decimal_symbol: char,
    tokens: Option<Vec<(String, Option<usize>)>>,
    commands: Option<Vec<ExpCommand>>,
}

impl HistoryCollector {
    pub fn new(
        calc_display: Option<Rc<RefCell<dyn CalcDisplay>>>,
        history_display: Option<Rc<RefCell<dyn HistoryDisplay>>>,
        decimal_symbol: char,
    ) -> Self {
        HistoryCollector {
            calc_display: calc_display,
            commands: None,
            cur_line_hist_start: None,
            cur_operand_index: None,
            decimal_symbol: decimal_symbol,
            history_display: history_display,
            last_bin_op_start_index: None,
            last_op_start_index: None,
            last_opnd_brace: false,
            operand_indices: [None; MAXPRECDEPTH],
            tokens: None,
        }
    }

    pub fn add_opnd_to_history(
        &mut self,
        num_str: &str,
        rational: Rational,
        repetition: Option<bool>,
    ) {
        assert!(num_str.len() >= 1);

        let repetition = repetition.unwrap_or(false);

        let is_negative = num_str.chars().nth(0).unwrap() == '-';
        let mut commands = Vec::new();
        let mut is_sci_fmt = false;
        let mut is_decimal = false;

        let index = if is_negative { 1 } else { 0 };

        for n in num_str[index..num_str.len()].chars() {
            match n {
                'e' => commands.push(Command::Exp),
                '-' => commands.push(Command::Sign),
                '+' => {}
                c if c == self.decimal_symbol => {
                    commands.push(Command::Pnt);
                    if !is_sci_fmt {
                        is_decimal = true;
                    }
                }
                _ => {
                    let mut num = n as isize - '0' as isize;
                    num += Command::N0 as isize;
                    let command = Command::digit_from_integer(num).unwrap();
                    commands.push(command);
                }
            }
        }

        let mut operand_command = OpndCommand::new(commands, is_negative, is_decimal, is_sci_fmt);
        operand_command.set_rational(rational);
        let command_end = self.add_command(ExpCommand::Opnd(operand_command));
        self.last_op_start_index =
            Some(self.ich_add_string_to_equation_string(num_str, Some(command_end)));

        if repetition {
            self.set_expression_display();
        }

        self.last_opnd_brace = false;
        self.last_bin_op_start_index = None;
    }

    pub fn remove_last_opnd_from_history(&mut self) {
        self.truncate_equation_string_from_ich(self.last_op_start_index.unwrap());
        self.set_expression_display();
        self.last_op_start_index = None;
        // TODO
        // This will not restore the last_bin_op_start_index as it is not possible
        // to remove that also later.
    }

    pub fn add_binary_command_to_history(
        &mut self,
        command: Command,
        is_integer_mode: bool,
        no_repetition: Option<bool>,
    ) {
        // TODO
        let command_end = self.add_command(ExpCommand::Binary(BinaryCommand::new(command)));
        self.last_bin_op_start_index = Some(self.ich_add_string_to_equation_string(" ", None));

        self.ich_add_string_to_equation_string(
            command.to_binary_string(is_integer_mode).as_str(),
            Some(command_end),
        );

        if no_repetition.unwrap_or(true) {
            self.set_expression_display();
        }

        self.last_op_start_index = None;
    }

    // This is expected to be called when a binary op in the last say 1+2+ is changing to another one say 1+2* (+ changed to *)
    // It needs to know by this change a Precedence inversion happened. i.e. previous op was lower or equal to its previous op, but the new
    // one isn't. (Eg. 1*2* to 1*2^). It can add explicit brackets to ensure the precedence is inverted. (Eg. (1*2) ^)
    pub fn change_last_bin_op(
        &mut self,
        command: Command,
        prev_inv_to_higher: bool,
        is_integer_mode: bool,
    ) {
        self.truncate_equation_string_from_ich(self.last_bin_op_start_index.unwrap());

        if prev_inv_to_higher {
            self.enclose_prec_inversion_brackets();
        }

        self.add_binary_command_to_history(command, is_integer_mode, None);
    }

    pub fn add_unary_command_to_history(
        &mut self,
        command: Command,
        inv: bool,
        angle_type: AngleType,
    ) {
        let command_end;

        if command == Command::Percent {
            command_end = self.add_command(ExpCommand::Unary(UnaryCommand::new(command, None)));
            self.ich_add_string_to_equation_string(
                CalculatorEngine::command_to_string(command),
                Some(command_end),
            );
        } else {
            let exp_command: ExpCommand;
            if command == Command::Sign {
                exp_command = ExpCommand::Unary(UnaryCommand::new(command, None));
            } else {
                let angle_command = match angle_type {
                    AngleType::Degress => Command::Deg,
                    AngleType::Radians => Command::Rad,
                    AngleType::Gradians => Command::Grad,
                };

                let mut command = command;
                exp_command = if inv {
                    match command {
                        Command::Sin => {
                            command = Command::Asin;
                            ExpCommand::Unary(UnaryCommand::new(angle_command, Some(command)))
                        }
                        Command::Cos => {
                            command = Command::Acos;
                            ExpCommand::Unary(UnaryCommand::new(angle_command, Some(command)))
                        }
                        Command::Tan => {
                            command = Command::Atan;
                            ExpCommand::Unary(UnaryCommand::new(angle_command, Some(command)))
                        }
                        Command::Sinh => {
                            command = Command::Asinh;
                            ExpCommand::Unary(UnaryCommand::new(command, None))
                        }
                        Command::Cosh => {
                            command = Command::Acosh;
                            ExpCommand::Unary(UnaryCommand::new(command, None))
                        }
                        Command::Tanh => {
                            command = Command::Atanh;
                            ExpCommand::Unary(UnaryCommand::new(command, None))
                        }
                        Command::Sec => {
                            command = Command::Asec;
                            ExpCommand::Unary(UnaryCommand::new(angle_command, Some(command)))
                        }
                        Command::Csc => {
                            command = Command::Acsc;
                            ExpCommand::Unary(UnaryCommand::new(angle_command, Some(command)))
                        }
                        Command::Cot => {
                            command = Command::Acot;
                            ExpCommand::Unary(UnaryCommand::new(angle_command, Some(command)))
                        }
                        Command::Sech => {
                            command = Command::Asech;
                            ExpCommand::Unary(UnaryCommand::new(command, None))
                        }
                        Command::Csch => {
                            command = Command::Acsch;
                            ExpCommand::Unary(UnaryCommand::new(command, None))
                        }
                        Command::Coth => {
                            command = Command::Acoth;
                            ExpCommand::Unary(UnaryCommand::new(command, None))
                        }
                        Command::Ln => {
                            command = Command::Powe;
                            ExpCommand::Unary(UnaryCommand::new(command, None))
                        }
                        // Should it be inverse if the op itself is not invertible?
                        _ => ExpCommand::Unary(UnaryCommand::new(command, None)),
                    }
                } else {
                    match command {
                        Command::Sin
                        | Command::Cos
                        | Command::Tan
                        | Command::Sec
                        | Command::Csc
                        | Command::Cot => {
                            ExpCommand::Unary(UnaryCommand::new(angle_command, Some(command)))
                        }
                        _ => ExpCommand::Unary(UnaryCommand::new(command, None)),
                    }
                };
            }

            // TODO
            let command_end = self.add_command(exp_command);

            let mut operan = command.to_unary_string(inv, angle_type);

            if self.last_opnd_brace {
                operan += CalculatorEngine::command_to_string(Command::OpenP);
            }

            self.insert_string_into_equation_string(
                operan.as_str(),
                Some(command_end),
                self.last_op_start_index.unwrap(),
            );

            if self.last_opnd_brace {
                self.ich_add_string_to_equation_string(
                    CalculatorEngine::command_to_string(Command::CloseP),
                    None,
                );
            }
        }

        self.set_expression_display();
        self.last_opnd_brace = false;
        self.last_bin_op_start_index = None;
    }

    pub fn add_open_brace_to_history(&mut self) {
        // TODO
        //self.add_command(Parentheses { Command::OpenP });

        let ich_opnd_start = self.ich_add_string_to_equation_string(
            CalculatorEngine::command_to_string(Command::OpenP),
            None,
        );

        self.set_expression_display();
        self.last_bin_op_start_index = None;
    }

    pub fn add_close_brace_to_history(&mut self) {
        // TODO
        //self.add_command(Parentheses { Command::CloseP });

        self.ich_add_string_to_equation_string(
            CalculatorEngine::command_to_string(Command::CloseP),
            None,
        );
        self.set_expression_display();
        self.pop_last_opdn_start();

        self.last_bin_op_start_index = None;
        self.last_opnd_brace = true;
    }

    pub fn push_last_opnd_start(&mut self, ich_opnd_start: Option<usize>) {
        let ich = ich_opnd_start.unwrap_or(self.last_op_start_index.unwrap());

        self.cur_operand_index = self.cur_operand_index.map(|value| {
            if value < self.operand_indices.len() {
                self.operand_indices[self.cur_operand_index.unwrap()] = Some(ich);
                return value + 1;
            }
            value
        });
    }

    pub fn pop_last_opdn_start(&mut self) {
        if let Some(cur_operand_index) = self.cur_operand_index {
            if cur_operand_index > 0 {
                self.cur_operand_index = self.cur_operand_index.map(|value| value - 1);
                self.last_op_start_index = Some(self.operand_indices[cur_operand_index].unwrap());
            }
        }
    }

    pub fn enclose_prec_inversion_brackets(&mut self) {
        let ich_start = if self.cur_operand_index.unwrap() > 0 {
            self.operand_indices[self.cur_operand_index.unwrap() - 1].unwrap()
        } else {
            0
        };

        self.insert_string_into_equation_string(
            CalculatorEngine::command_to_string(Command::OpenP),
            None,
            ich_start,
        );
        self.ich_add_string_to_equation_string(
            CalculatorEngine::command_to_string(Command::CloseP),
            None,
        );
    }

    pub fn opnd_added_to_history(&self) -> bool {
        false
    }

    pub fn complete_history_line(&mut self, num_str: &str) {
        if let Some(history_display) = self.history_display.as_mut() {
            let added_item_index =
                history_display.borrow_mut().add_to_history(self.tokens.take().unwrap(), self.commands.take().unwrap(), num_str);
            if let Some(calc_display) = self.calc_display {
                calc_display.borrow().on_history_item_added(added_item_index);
            }
        }

        self.cur_line_hist_start = None;
        self.load();
    }

    pub fn complete_equation(&mut self, num_str: &str) {
        // Add only '=' token and not add EQU command, because
        // EQU command breaks loading from history (it duplicate history entries).
        self.ich_add_string_to_equation_string(
            CalculatorEngine::command_to_string(Command::Equ),
            None,
        );

        self.set_expression_display();
        self.complete_history_line(num_str);
    }

    pub fn clear_history_line(&mut self, err_str: &str) {
        if err_str.is_empty() {
            if let Some(calc_display) = self.calc_display {
                calc_display.borrow_mut().set_expression_display(&[], &[]);
            }

            self.cur_line_hist_start = None;
            self.load();
        }
    }

    pub fn add_command(&mut self, command: ExpCommand) -> usize {
        let mut commands = self.commands.get_or_insert_with(|| Vec::new());

        commands.push(command);
        commands.len() - 1
    }

    pub fn update_history_expression(
        &mut self,
        radix: u64,
        precision: crate::engine::CalculatorPrecision,
    ) {
        if let Some(tokens) = self.tokens.as_ref() {
            for token in tokens.iter() {
                if let Some(command_position) = token.1 {
                    let exp_command = self.commands.as_mut().unwrap()[command_position];

                    if let ExpCommand::Opnd(operand) = exp_command {
                        token.0 = operand.get_string(radix, precision);
                        operand.set_commands(
                            self.get_operation_commands_from_string(token.0.as_str()),
                        );
                    }
                }
            }
        }
    }

    pub fn set_decimal_symbol(&mut self, decimal_symbol: char) {
        self.decimal_symbol = decimal_symbol;
    }
}

impl HistoryCollector {
    fn load(&mut self) {
        self.last_op_start_index = None;
        self.last_bin_op_start_index = None;
        self.cur_operand_index = None;
        self.last_opnd_brace = false;

        if let Some(tokens) = self.tokens.as_mut() {
            tokens.clear();
        }

        if let Some(commands) = self.commands.as_mut() {
            commands.clear();
        }
    }

    // Inserts a given string into the global `equation` at the given index ich taking care of reallocation.
    fn ich_add_string_to_equation_string(
        &mut self,
        string: &str,
        command_index: Option<usize>,
    ) -> usize {
        let mut tokens = self.tokens.get_or_insert_with(|| Vec::new());

        tokens.push((String::from(string), command_index));
        tokens.len() - 1
    }

    // Chops off the current equation string from the given index.
    fn truncate_equation_string_from_ich(&mut self, ich: usize) {
        let tokens = self.tokens.as_ref().unwrap();

        // Truncate commands.
        let min_index = None;
        let ntokens = tokens.len();

        for current_pair in &tokens[ich..ntokens] {
            let cur_token_id = current_pair.1;
            if cur_token_id.is_some() {
                if min_index.is_some() || cur_token_id < min_index {
                    min_index = cur_token_id;
                    // TODO
                    // truncate(self.commands, min_idex)
                }
            }
        }

        // TODO
        //truncate(self.commands);
    }

    fn set_expression_display(&mut self) {
        if let Some(calc_display) = self.calc_display {
            calc_display.borrow_mut().set_expression_display(
                self.tokens.unwrap().as_slice(),
                self.commands.unwrap().as_slice(),
            );
        }
    }

    fn insert_string_into_equation_string(
        &mut self,
        string: &str,
        command_index: Option<usize>,
        ich: usize,
    ) {
        self.tokens
            .as_mut()
            .unwrap()
            .insert(ich, (String::from(string), command_index));
    }

    fn get_operation_commands_from_string(&self, num_str: &str) -> Vec<Command> {
        let mut commands = Vec::new();

        let negative = num_str.chars().nth(0).unwrap() == '-';
        let index = if negative { 1 } else { 0 };

        for n in num_str[index..num_str.len()].chars() {
            match n {
                'e' => commands.push(Command::Exp),
                '-' => commands.push(Command::Sign),
                '+' => {}
                c if c == self.decimal_symbol => {
                    commands.push(Command::Pnt);
                }
                _ => {
                    let mut num = n as isize - '0' as isize;
                    num += Command::N0 as isize;
                    let command = Command::digit_from_integer(num).unwrap();
                    commands.push(command);
                }
            }
        }

        commands
    }
}

// fn truncate<T>(v: &mut [T], index: usize) {
//     if index >= v.len() {
//         panic!("Bounds.");
//     }

//     v.
// }
