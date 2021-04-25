use crate::engine_strings::EngineStringsId;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use super::constants::*;

use super::*;
use super::{command::*, AngleType, CalculatorEngine};
use num::*;

lazy_static! {
    static ref TWO_PI: Rational = Rational::zero();
    static ref PI: Rational = Rational::zero();
}


#[derive(Clone)]
struct FunctionNameElement {
    degree: Option<EngineStringsId>,
    inverse_degree: Option<EngineStringsId>,
    rad: Option<EngineStringsId>,
    inverse_rad: Option<EngineStringsId>,
    grad: Option<EngineStringsId>,
    inverse_grad: Option<EngineStringsId>,
    programmer_mode: Option<EngineStringsId>,
}

impl FunctionNameElement {
    fn has_angle(&self) -> bool {
        self.rad.is_some()
            || self.inverse_rad.is_some()
            || self.grad.is_some()
            || self.inverse_grad.is_some()
    }
}

lazy_static! {
    static ref OPERATOR_STRING_TABLE: HashMap<Command, FunctionNameElement> = {
        [
            (
                Command::Chop,
                FunctionNameElement {
                    inverse_degree: Some(EngineStringsId::Frac),
                    degree: None,
                    grad: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Sin,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Sind),
                    inverse_degree: Some(EngineStringsId::Asind),
                    rad: Some(EngineStringsId::Sinr),
                    inverse_rad: Some(EngineStringsId::Asinr),
                    grad: Some(EngineStringsId::Sing),
                    inverse_grad: Some(EngineStringsId::Asing),
                    programmer_mode: None,
                },
            ),
            (
                Command::Cos,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Cosd),
                    inverse_degree: Some(EngineStringsId::Acosd),
                    rad: Some(EngineStringsId::Cosr),
                    inverse_rad: Some(EngineStringsId::Acosr),
                    grad: Some(EngineStringsId::Cosg),
                    inverse_grad: Some(EngineStringsId::Acosg),
                    programmer_mode: None,
                },
            ),
            (
                Command::Tan,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Tand),
                    inverse_degree: Some(EngineStringsId::Atand),
                    rad: Some(EngineStringsId::Tanr),
                    inverse_rad: Some(EngineStringsId::Atanr),
                    grad: Some(EngineStringsId::Tang),
                    inverse_grad: Some(EngineStringsId::Atang),
                    programmer_mode: None,
                },
            ),
            (
                Command::Sinh,
                FunctionNameElement {
                    inverse_degree: Some(EngineStringsId::Asinh),
                    degree: None,
                    grad: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Cosh,
                FunctionNameElement {
                    inverse_degree: Some(EngineStringsId::Acosh),
                    degree: None,
                    grad: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Tanh,
                FunctionNameElement {
                    inverse_degree: Some(EngineStringsId::Atanh),
                    degree: None,
                    grad: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Sec,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Secd),
                    inverse_degree: Some(EngineStringsId::Asecd),
                    rad: Some(EngineStringsId::Secr),
                    inverse_rad: Some(EngineStringsId::Asecr),
                    grad: Some(EngineStringsId::Secg),
                    inverse_grad: Some(EngineStringsId::Asecg),
                    programmer_mode: None,
                },
            ),
            (
                Command::Csc,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Cscd),
                    inverse_degree: Some(EngineStringsId::Acscd),
                    rad: Some(EngineStringsId::Cscr),
                    inverse_rad: Some(EngineStringsId::Acscr),
                    grad: Some(EngineStringsId::Cscg),
                    inverse_grad: Some(EngineStringsId::Acscg),
                    programmer_mode: None,
                },
            ),
            (
                Command::Cot,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Cotd),
                    inverse_degree: Some(EngineStringsId::Acotd),
                    rad: Some(EngineStringsId::Cotr),
                    inverse_rad: Some(EngineStringsId::Acotr),
                    grad: Some(EngineStringsId::Cotg),
                    inverse_grad: Some(EngineStringsId::Acotg),
                    programmer_mode: None,
                },
            ),
            (
                Command::Sech,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Sech),
                    inverse_degree: Some(EngineStringsId::Asech),
                    grad: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Csch,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Csch),
                    inverse_degree: Some(EngineStringsId::Acsch),
                    grad: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Coth,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Coth),
                    inverse_degree: Some(EngineStringsId::Acoth),
                    grad: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Ln,
                FunctionNameElement {
                    inverse_degree: Some(EngineStringsId::Powe),
                    degree: None,
                    grad: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Sqr,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Sqr),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Cub,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Cube),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Fac,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Fact),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Rec,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Reciproc),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Dms,
                FunctionNameElement {
                    inverse_degree: Some(EngineStringsId::Degrees),
                    grad: None,
                    degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Sign,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Negate),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Degrees,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Degrees),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Pow2,
                FunctionNameElement {
                    degree: Some(EngineStringsId::TwoPowX),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::LogBaseY,
                FunctionNameElement {
                    degree: Some(EngineStringsId::LogBaseY),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Abs,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Abs),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Ceil,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Ceil),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Floor,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Floor),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Nand,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Nand),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Nor,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Nor),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Rshfl,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Rsh),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Rorc,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Ror),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Rolc,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Rol),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::CubeRoot,
                FunctionNameElement {
                    degree: Some(EngineStringsId::CubeRoot),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    programmer_mode: None,
                    rad: None,
                },
            ),
            (
                Command::Mod,
                FunctionNameElement {
                    degree: Some(EngineStringsId::Mod),
                    programmer_mode: Some(EngineStringsId::ProgrammerMod),
                    grad: None,
                    inverse_degree: None,
                    inverse_grad: None,
                    inverse_rad: None,
                    rad: None,
                },
            ),
        ]
        .iter()
        .cloned()
        .collect()
    };
}

impl Command {
    fn get_precedence(&self) -> u8 {
        match *self {
            Command::Nop | Command::Or | Command::Xor => 0,
            Command::And | Command::Nand | Command::Nor => 1,
            Command::Add | Command::Sub => 2,
            Command::Rshf
            | Command::Lshf
            | Command::Rshfl
            | Command::Mod
            | Command::Div
            | Command::Mul => 3,
            Command::Pwr | Command::Root | Command::LogBaseY => 4,
            _ => 0,
        }
    }
}

impl CalculatorEngine {
    pub fn handle_error_command(&mut self, command: Command) {}

    pub fn handle_max_digits_reached(&mut self) {
        if let Some(calc_display) = self.calc_display.as_mut() {
            calc_display.borrow_mut().max_digits_reached();
        }
    }

    pub fn clear_temporary_values(&mut self) {
        self.inv = false;
        self.input.clear();
        self.record = true;
        self.check_add_last_binary_command_to_history(None);
        self.display_num();
        self.error = false;
    }

    pub fn clear_display(&mut self) {
        if let Some(calc_display) = self.calc_display.as_mut() {
            calc_display.borrow_mut().set_expression_display(&[], &[]);
        }
    }

    pub fn process_command(&mut self, command: Command) {
        let mut command = command;

        if command == Command::SetResult {
            command = Command::Recall;
            self.set_calc_state = true;
        }

        self.process_command_worker(command);
    }

    pub fn process_command_worker(&mut self, command: Command) {
        let mut command = command;

        if !command.is_gui_setting() {
            self.last_command = self.temp_command;
            self.temp_command = command;
        }

        // Clear expression shown after = sign.
        if self.no_previous_equal {
            self.clear_display();
        }

        if self.error {
            match command {
                Command::Clear => {}
                Command::Centr => command = Command::Clear,
                _ => self.handle_error_command(command),
            }
        }

        if self.record {
            if command.is_binary()
                || command.is_unary()
                || command.is_in_range(Command::Fe, Command::MMinus)
                || command.is_in_range(Command::Hex, Command::Bin)
                || command.is_in_range(Command::Qword, Command::Byte)
                || command.is_in_range(Command::Deg, Command::Grad)
                || command.is_in_range(Command::BinPos0, Command::BinPos63)
                || command == Command::Inv
                || (command == Command::Sign && self.radix != 10)
                || command == Command::Rand
                || command == Command::Euler
            {
                self.record = false;
                self.current_value = self.input.to_rational(self.radix, self.precision);
                self.display_num();
            }
        } else if command.is_digit() || command == Command::Pnt {
            self.record = true;
            self.input.clear();
            self.check_add_last_binary_command_to_history(None);
        }

        // Interpret digit keys.
        if command.is_digit() {
            let value = command.get_integer();

            if value >= self.radix {
                self.handle_error_command(command);
                return;
            }

            if !self.input.try_add_digit(
                value,
                self.radix,
                self.interger_mode,
                self.get_max_decimal_value_string(),
                self.bit_width,
                self.int_digits_sav,
            ) {
                self.handle_error_command(command);
                self.handle_max_digits_reached();
                return;
            }

            self.display_num();

            return;
        }

        if command.is_binary() {
            // Change the operation if last input was operation.
            if self.last_command.is_binary() {
                let mut precedence_inversion_to_higher = false;

                if self.precedence && self.previous_command != Command::Nop {
                    let prev = self.previous_command.get_precedence();
                    let x = self.last_command.get_precedence();
                    let i = self.command.get_precedence();

                    // Condition for precedence inversion.
                    if x <= prev && i > prev {
                        precedence_inversion_to_higher = true;
                        self.previous_command = Command::Nop;
                    }
                }

                self.history_collector.change_last_bin_op(
                    self.command,
                    precedence_inversion_to_higher,
                    self.interger_mode,
                );
                self.display_announce_binary_operator();
                return;
            }

            if self.history_collector.opnd_added_to_history() {
                // If the previous command was ) or unop, then it's already in history as an opnd from (...)
                self.history_collector.add_opnd_to_history(
                    self.number_string.as_str(),
                    self.current_value,
                    None,
                );
            }

            if self.change_commad {
                self.do_precedence_check(command);
            }

            self.display_announce_binary_operator();
            self.last_value = self.current_value;
            self.command = command;
            self.history_collector
                .add_binary_command_to_history(command, self.interger_mode, None);
            self.change_commad = true;
            self.no_previous_equal = true;
            return;
        }

        if command.is_unary() || command == Command::Degrees {
            if self.last_command.is_binary() {
                self.current_value = self.last_value;
            }

            if command != Command::Percent {
                if self.history_collector.opnd_added_to_history() {
                    self.history_collector.add_opnd_to_history(
                        self.number_string.as_str(),
                        self.current_value,
                        None,
                    );
                }

                self.history_collector
                    .add_unary_command_to_history(command, self.inv, self.angle_type);
            }

            match command {
                Command::Sin
                | Command::Cos
                | Command::Tan
                | Command::Sinh
                | Command::Cosh
                | Command::Tanh
                | Command::Sec
                | Command::Csc
                | Command::Cot
                | Command::Sech
                | Command::Csch
                | Command::Coth => {
                    if self.is_current_too_big_for_trig() {
                        self.current_value = Rational::zero();
                        self.display_error(1);
                        return;
                    }
                }
                _ => {}
            };

            self.current_value = self.sci_calc_functions(self.current_value, command);

            if self.error {
                return;
            }

            self.display_num();

            if command == Command::Percent {
                self.check_add_last_binary_command_to_history(None);
                self.history_collector.add_opnd_to_history(
                    self.number_string.as_str(),
                    self.current_value,
                    Some(true),
                );
            }

            match command {
                Command::Chop
                | Command::Sin
                | Command::Cos
                | Command::Tan
                | Command::Ln
                | Command::Dms
                | Command::Degrees
                | Command::Sinh
                | Command::Cosh
                | Command::Tanh
                | Command::Sec
                | Command::Csc
                | Command::Cot
                | Command::Sech
                | Command::Csch
                | Command::Coth
                    if self.inv =>
                {
                    self.inv = false
                }
                _ => {}
            };

            return;
        }

        if command.is_in_range(BIN_EDIT_START, BIN_EDIT_END) {
            if self.last_command.is_binary() {
                self.current_value = self.last_value;
            }

            self.check_add_last_binary_command_to_history(None);

            if self.try_toggle_bit(&self.current_value, command.binary_index()) {
                self.display_num();
            }

            return;
        }

        match command {
            Command::Clear => {
                if self.change_commad {
                    self.check_add_last_binary_command_to_history(Some(false));
                }

                self.last_value = Rational::zero();

                self.change_commad = false;
                self.open_parentheses_count = 0;
                self.precedence_command_count = 0;
                self.temp_command = Command::Nop;
                self.last_command = Command::Nop;
                self.command = Command::Nop;
                self.previous_command = Command::Nop;
                self.no_previous_equal = true;
                self.carry_bit = 0;

                // Clear the parentheses status box indicator; this will not cleared for CENTR.
                if let Some(calc_display) = self.calc_display {
                    calc_display.borrow_mut().set_parenthesis_number(0);
                    self.clear_display();
                }

                self.history_collector.clear_history_line("");
                self.clear_temporary_values();
            }
            Command::Back => {
                // Divide number by the current radix and truncate.
                // Only allow backspace if we're recording.
                if self.record {
                    self.input.backspace();
                    self.display_num();
                } else {
                    self.handle_error_command(command);
                }
            }
            Command::Equ => {
                while self.open_parentheses_count > 0 {
                    if self.error {
                        return;
                    }

                    self.temp_command = self.last_command;
                    self.process_command(Command::CloseP);
                    self.last_command = self.temp_command;
                    self.temp_command = command;
                }

                if !self.no_previous_equal {
                    self.last_value = self.current_value;
                }

                if self.last_command.is_binary() {
                    self.current_value = self.last_value;
                }

                if !self.history_collector.opnd_added_to_history() {
                    self.history_collector.add_opnd_to_history(
                        self.number_string.as_str(),
                        self.current_value,
                        None,
                    );
                }

                // Evaluate the precedence stack.
                self.resolve_highest_precedence_operation();

                while self.precedence && self.precedence_command_count > 0 {
                    self.precedence_command_count -= 1;
                    self.command = self.precedence_commands[self.precedence_command_count];

                    // Precedence inversion check
                    let i = self.previous_command.get_precedence();
                    let x = self.command.get_precedence();

                    if i <= x {
                        self.history_collector.enclose_prec_inversion_brackets();
                    }

                    self.history_collector.pop_last_opdn_start();

                    self.no_previous_equal = true;

                    self.resolve_highest_precedence_operation();
                }

                if !self.error {
                    let grouped_string =
                        self.group_digits_per_radix(self.number_string.as_str(), self.radix);
                    self.history_collector
                        .complete_equation(grouped_string.as_str());
                }

                self.change_commad = false;
                self.previous_command = Command::Nop;
            }
            Command::OpenP | Command::CloseP => {
                if self.open_parentheses_count >= MAX_PREC_DEPTH && command == Command::OpenP
                    || self.open_parentheses_count == 0 && command != Command::OpenP
                    || self.precedence_command_count >= MAX_PREC_DEPTH
                        && self.precedence_commands[self.precedence_command_count - 1]
                            != Command::Nop
                {
                    if self.open_parentheses_count == 0 && command != Command::OpenP {
                        match self.calc_display.as_mut() {
                            Some(calc_display) => calc_display.as_ref().borrow().on_no_right_paren_added(),
                            None => {}
                        }
                    }

                    self.handle_error_command(command);
                    return;
                }

                if command == Command::OpenP {
                    self.check_add_last_binary_command_to_history(None);
                    self.history_collector.add_open_brace_to_history();

                    // Open level of parenthesis, save number and operation.
                    self.paren_vals[self.open_parentheses_count] = self.last_value;

                    self.commands[self.open_parentheses_count] = if self.change_commad {
                        self.command
                    } else {
                        Command::Nop
                    };

                    self.open_parentheses_count += 1;

                    // Save a special marker on the precedence array.
                    if self.precedence_command_count < self.precedence_commands.len() {
                        self.precedence_commands[self.precedence_command_count] = Command::Nop;
                        self.precedence_command_count += 1;
                    }

                    self.last_value = Rational::zero();

                    if self.last_command.is_binary() {
                        // We want 1 + ( to start as 1 + (0. Any number you type replaces 0.
                        // But if it is 1 + 3 (, it is treated as 1 + (3
                        self.current_value = Rational::zero();
                    }

                    self.temp_command = Command::Nop;
                    self.command = Command::Nop;
                    self.change_commad = false; // A `(` is like starting a new sub equation.
                } else {
                    if self.last_command.is_binary() {
                        self.current_value = self.last_value;
                    }

                    if !self.history_collector.opnd_added_to_history() {
                        self.history_collector.add_opnd_to_history(
                            self.number_string.as_str(),
                            self.current_value,
                            None,
                        );
                    }

                    self.current_value =
                        self.do_operation(self.command, self.current_value, self.last_value);
                    self.previous_command = self.command;

                    // Process the precedence stack until the command in null.
                    self.precedence_command_count -= 1;
                    self.command = self.precedence_commands[self.precedence_command_count];

                    while self.command != Command::Nop {
                        // Precedence inversion check
                        let i = self.previous_command.get_precedence();
                        let x = self.command.get_precedence();

                        if i < x {
                            self.history_collector.enclose_prec_inversion_brackets();
                        }

                        self.history_collector.pop_last_opdn_start();

                        self.last_value = self.precedence_vals[self.precedence_command_count];

                        self.current_value =
                            self.do_operation(self.command, self.current_value, self.last_value);
                        self.previous_command = self.command;

                        self.precedence_command_count -= 1;
                        self.command = self.precedence_commands[self.precedence_command_count];
                    }

                    self.history_collector.add_close_brace_to_history();

                    self.open_parentheses_count -= 1;
                    self.last_value = self.paren_vals[self.open_parentheses_count];
                    self.command = self.commands[self.open_parentheses_count];

                    self.change_commad = self.command != Command::Nop;
                }

                match self.calc_display.as_mut() {
                    Some(calc_display) => {
                        calc_display.borrow_mut().set_parenthesis_number(if self.open_parentheses_count >= 0 {
                            self.open_parentheses_count
                        } else {
                            0
                        })
                    }
                    None => {}
                }

                if !self.error {
                    self.display_num();
                }
            }

            Command::Hex | Command::Dec | Command::Oct | Command::Bin => {
                self.set_radix_type_and_num_width(Some(command.get_radix_type().unwrap()), None);
                self.history_collector
                    .update_history_expression(self.radix, self.precision);
            }
            Command::Qword | Command::Dword | Command::Word | Command::Byte => {
                if self.record {
                    self.current_value = self.input.to_rational(self.radix, self.precision);
                    self.record = false;
                }

                self.set_radix_type_and_num_width(None, Some(command.get_num_width().unwrap()));
            }
            Command::Dec | Command::Rad | Command::Grad => {
                self.angle_type = command.get_angle_type().unwrap();
            }
            Command::Sign => {
                if self.record {
                    if self
                        .input
                        .try_toggle_sign(self.interger_mode, self.get_max_decimal_value_string())
                    {
                        self.display_num();
                    } else {
                        self.handle_error_command(command);
                    }
                }

                if self.last_command.is_binary() {
                    self.current_value = self.last_value;
                }

                if !self.history_collector.opnd_added_to_history() {
                    self.history_collector
                        .add_opnd_to_history(self.number_string.as_str(), self.current_value, None);
                }

                self.current_value = -self.current_value;

                self.display_num();
                self.history_collector.add_unary_command_to_history(
                    Command::Sign,
                    self.inv,
                    self.angle_type,
                );
            }

            Command::Recall => {
                if self.set_calc_state {
                    // Not a memory recall, set the result.
                    self.set_calc_state = false;
                } else {
                    // Recall immediate memory value.
                    self.current_value = *self.memory_value;
                }

                self.check_add_last_binary_command_to_history(None);
                self.display_num();
            }
            Command::MPlus => {
                let result = *self.memory_value + self.current_value;
                //TODO self.memory_value = result.truncate_num_for_int_math();
            }
            Command::MMinus => {
                let result = *self.memory_value - self.current_value;
                //TODO self.memory_value = result.truncate_num_for_int_math();
            }
            Command::Store => {} //TODO self.memory_value = result.truncate_num_for_int_math();
            Command::MClear => self.memory_value = Box::new(Rational::zero()),
            Command::Pi => {
                if !self.interger_mode {
                    self.check_add_last_binary_command_to_history(None);
                    self.current_value = if self.inv { Rational::zero() } else { Rational::zero() }; // TODO

                    self.display_num();
                    self.inv = false;
                    return;
                }

                self.handle_error_command(command);
            }
            Command::Rand => {
                if !self.interger_mode {
                    self.check_add_last_binary_command_to_history(None);

                    // TODO rational operations.

                    self.inv = false;
                    return;
                }

                self.handle_error_command(command);
            }
            Command::Euler => {
                if !self.interger_mode {
                    self.check_add_last_binary_command_to_history(None);
                    // TODO self.current_value = Rational {RATIONAL_EXPONENTIAL };

                    self.display_num();
                    self.inv = false;
                    return;
                }

                self.handle_error_command(command);
            }
            Command::Fe => {
                // Toggle exponential notation display.
                //TODO self.nFE = '
                self.display_num();
            }
            Command::Exp => {
                if self.record && !self.interger_mode && self.input.try_begin_exponent() {
                    self.display_num();
                    return;
                }

                self.handle_error_command(command);
            }
            Command::Pnt => {
                if self.record && !self.interger_mode && self.input.try_add_decimal_pt() {
                    self.display_num();
                } else {
                    self.handle_error_command(command);
                }
            }
            Command::Inv => self.inv = !self.inv,
            _ => {}
        }
    }

    fn do_precedence_check(&mut self, command: Command) {
        let mut x = command.get_precedence();
        let mut i = self.command.get_precedence();

        if (x > i) && self.precedence {
            if self.precedence_command_count < MAX_PREC_DEPTH {
                let size = self.precedence_command_count;
                self.precedence_vals[size] = self.last_value;

                self.precedence_commands[size] = self.command;
                self.history_collector.push_last_opnd_start(None);
            } else {
                self.precedence_command_count = MAX_PREC_DEPTH - 1;
                self.handle_error_command(command);
            }
            self.precedence_command_count += 1;
        } else {
            self.current_value =
                self.do_operation(self.command, self.current_value, self.last_value);
            self.previous_command = self.command;

            if !self.error {
                self.display_num();
            }

            if (self.precedence_command_count != 0)
                && self.precedence_commands[self.precedence_command_count - 1] != Command::Nop
            {
                self.precedence_command_count -= 1;
                self.command = self.precedence_commands[self.precedence_command_count];

                x = command.get_precedence();

                if i <= x {
                    self.history_collector.enclose_prec_inversion_brackets();
                }

                self.history_collector.pop_last_opdn_start();
                return self.do_precedence_check(command);
            }
        }
    }

    pub fn resolve_highest_precedence_operation(&mut self) {
        if self.command != Command::Nop {
            // If this is the first equ in a string, set `self.hold_valueue = self.current_value`.
            // Otherwise let `self.current_value = self.hold_valueue`. This keep `self.current_value`
            // constant through all EQUs in a row.
            if self.no_previous_equal {
                self.hold_value = self.current_value;
            } else {
                self.current_value = self.hold_value;
                self.display_num(); // To update the `self.num_string`.
                self.history_collector.add_binary_command_to_history(
                    self.command,
                    self.interger_mode,
                    Some(false),
                );
                self.history_collector.add_opnd_to_history(
                    self.number_string.as_str(),
                    self.current_value,
                    None,
                );
            }

            // Do the current or last operation.
            self.current_value =
                self.do_operation(self.command, self.current_value, self.last_value);
            self.previous_command = self.command;
            self.last_value = self.current_value;

            // Check for errors. If this wasn't done, `display_num` would immeditely overwrite any error message.
            if !self.error {
                self.display_num();
            }

            // No longer the first EQU.
            self.no_previous_equal = false;
        } else if !self.error {
            self.display_num();
        }
    }

    pub fn check_add_last_binary_command_to_history(&mut self, add_to_history: Option<bool>) {
        if self.change_commad {
            if self.history_collector.opnd_added_to_history() {
                self.history_collector.remove_last_opnd_from_history();
            }
        } else if self.history_collector.opnd_added_to_history() && !self.error {
            match self.last_command {
                // Corner case, where opnd is already in history but still a new opnd starting (1 + 4 sqrt 5). This is yet another
                // special casing of previous case under if (m_bChangeOp), but this time we can do better than just removing it
                // Let us make a current value =. So in case of 4 SQRT (or a equation under braces) and then a new equation is started, we can just form
                // a useful equation of sqrt(4) = 2 and continue a new equation from now on. But no point in doing this for things like
                // MR, SUM etc. All you will get is 5 = 5 kind of no useful equation.
                n @ Command::Sign | n @ Command::CloseP
                    if self.open_parentheses_count == 0 && n.is_unary() =>
                {
                    if add_to_history.unwrap_or(true) {
                        self.history_collector.complete_history_line(
                            self.group_digits_per_radix(self.number_string.as_str(), self.radix).as_str(),
                        );
                    }
                }
                _ => self.history_collector.remove_last_opnd_from_history(),
            }
        }
    }

    pub fn set_primary_display(&mut self, text: &str, is_error: bool) {
        if let Some(calc_display) = self.calc_display.as_mut() {
            calc_display.borrow_mut().set_primary_display(text, is_error);
            calc_display.borrow_mut().set_is_error(is_error);
        }
    }

    pub fn display_announce_binary_operator(&mut self) {
        if let Some(calc_display) = self.calc_display.as_mut() {
            calc_display.borrow_mut().binary_operation_received();
        }
    }

    pub fn is_current_too_big_for_trig(&self) -> bool {
        self.current_value >= self.max_trigonometric_num
    }

    pub fn get_current_radix(&self) -> u64 {
        self.radix
    }

    pub fn get_current_result_for_radix(
        &self,
        radix: u64,
        precision: u64,
        group_digits_per_radix: bool,
    ) -> String {
        let rational = if self.record {
            self.input.to_rational(self.radix, self.precision)
        } else {
            self.current_value
        };

        // TODO change_constants(self.radix, self.precision);

        let num_string = self.get_string_for_display(rational, radix);

        if !self.number_string.is_empty() {
            // TODO change_constants(self.radix, self.precision);
        }

        if group_digits_per_radix {
            self.group_digits_per_radix(self.number_string.as_str(), radix)
        } else {
            num_string
        }
    }

    pub fn get_string_for_display(&self, rational: Rational, radix: u64) -> String {
        //let result;

        String::new()

        // if !self.interger_mode {
        //     result = rational.to_string(radix, self.nFE, self.precision);
        // } else {
        //     // Programmer mode.
        //     // Find most siginificant bit to determine if number is negative.
        //     let temp_rational = self.truncate_num_for_int_math(rational);

        //     // TODO w64bits;
        // }
    }

    pub fn generate_random_number(&self) -> f64 {
        0.0
    }
}

impl Command {
    pub fn to_unary_string(self, inv: bool, angle_type: AngleType) -> String {
        let ids = OPERATOR_STRING_TABLE
            .get(&self)
            .map(|value| match angle_type {
                AngleType::Degress if !value.has_angle() => {
                    if inv {
                        value.inverse_degree
                    } else {
                        value.degree
                    }
                }
                AngleType::Radians => {
                    if inv {
                        value.inverse_rad
                    } else {
                        value.rad
                    }
                }
                AngleType::Gradians => {
                    if inv {
                        value.inverse_grad
                    } else {
                        value.inverse_grad
                    }
                }
            });

        if let Some(id) = ids {
            return String::from(id.expect("Invalid angle type").get_string());
        }

        return String::from(self.get_string_id().unwrap().get_string());
    }

    pub fn to_binary_string(self, is_integer_mode: bool) -> String {
        let ids = OPERATOR_STRING_TABLE.get(&self).map(|value| {
            if is_integer_mode && value.programmer_mode.is_some() {
                value.programmer_mode
            } else {
                value.degree
            }
        });

        if let Some(id) = ids {
            return String::from(id.expect("Invalid type").get_string());
        }

        return String::from(self.get_string_id().unwrap().get_string());
    }
}
