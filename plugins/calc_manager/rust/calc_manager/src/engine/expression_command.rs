use super::command::Command;
use super::command::CommandType;
use super::CalculatorPrecision;
use num::*;

#[derive(Clone)]
pub enum ExpCommand {
    Parentheses(ParenthesesCommand),
    Unary(UnaryCommand),
    Binary(BinaryCommand),
    Opnd(OpndCommand),
}

#[derive(Copy, Clone)]
pub struct ParenthesesCommand(Command);

impl ParenthesesCommand {
    pub fn new(command: Command) -> Self {
        Self(command)
    }
}

#[derive(Clone)]
pub struct UnaryCommand(Command, Option<Command>);

impl UnaryCommand {
    pub fn new(command0: Command, command1: Option<Command>) -> Self {
        Self(command0, command1)
    }

    pub fn set_command(&mut self, command0: Command) {
        self.0 = command0;
        self.1 = None;
    }

    pub fn set_commands(&mut self, command0: Command, command1: Command) {
        self.0 = command0;
        self.1 = Some(command1);
    }

    pub fn get_commands(&self) -> (Command, Option<Command>) {
        (self.0, self.1)
    }
}

#[derive(Clone)]
pub struct BinaryCommand(Command);

impl BinaryCommand {
    pub fn new(command: Command) -> Self {
        Self(command)
    }

    pub fn set_command(&mut self, value: Command) {
        self.0 = value;
    }

    pub fn get_command(&self) -> Command {
        self.0
    }
}

#[derive(Clone)]
pub struct OpndCommand {
    commands: Vec<Command>,
    is_negative: bool,
    is_sci_fmt: bool,
    is_decimal: bool,
    token: String,
    value: Option<Rational>,
}

impl OpndCommand {
    pub fn new(
        commands: Vec<Command>,
        is_negative: bool,
        is_decimal: bool,
        is_sci_fmt: bool,
    ) -> Self {
        Self {
            commands,
            is_decimal,
            is_negative,
            is_sci_fmt,
            token: "".to_owned(),
            value: None,
        }
    }

    pub fn set_rational(&mut self, value: Rational) {
        self.value = Some(value);
    }

    pub fn get_commands(&self) -> &[Command] {
        self.commands.as_slice()
    }

    pub fn set_commands(&mut self, commands: Vec<Command>) {
        self.commands = commands;
    }

    pub fn append_command(&mut self, command: Command) {
        if self.is_sci_fmt {
            self.clear_all_and_append_command(command);
        } else {
            self.commands.push(command);
        }

        if command == Command::Pnt {
            self.is_decimal = true;
        }
    }

    pub fn toggle_sign(&mut self) {
        for command in self.commands.iter() {
            if *command != Command::N0 {
                self.is_negative = !self.is_negative;
                break;
            }
        }
    }

    pub fn remove_from_end(&mut self) {
        if self.is_sci_fmt {
            self.clear_all_and_append_command(Command::N0);
        } else {
            let commands_count = self.commands.len();

            if commands_count == 1 {
                self.clear_all_and_append_command(Command::N0);
            } else {
                let command = self.commands[commands_count - 1];

                if command == Command::Pnt {
                    self.is_decimal = false;
                }

                self.commands.pop();
            }
        }
    }

    pub fn get_is_negative(&self) -> bool {
        self.is_negative
    }

    pub fn get_is_sci_fmt(&self) -> bool {
        self.is_sci_fmt
    }

    pub fn get_is_decimal_present(&self) -> bool {
        self.is_decimal
    }

    pub fn get_token(&mut self, decimal_symbol: char) -> &str {
        let commands_count = self.commands.len();
        self.token.clear();

        for (index, command) in self.commands.iter().enumerate() {
            match command {
                Command::Pnt => self.token.push(decimal_symbol),
                Command::Exp => {
                    self.token.push('0');
                    if let Some(next_command) = self.commands.get(index) {
                        if *next_command != Command::Sign {
                            self.token.push('+');
                        }
                    }
                }
                Command::Sign => self.token.push('-'),
                _ => {
                    let num = command.get_integer().to_string();
                    self.token += num.as_str();
                }
            }
        }

        for (index, ch) in self.token.chars().enumerate() {
            if ch != '0' {
                if ch == decimal_symbol {
                    self.token.replace_range(..index - 1, "");
                } else {
                    self.token.replace_range(..index, "");
                }

                if self.is_negative {
                    self.token.insert(0, '-');
                }

                return self.token.as_str();
            }
        }

        self.token = "0".to_owned();
        
        return self.token.as_str();
    }

    pub fn get_string(&self, radix: u64, precision: CalculatorPrecision) -> String {
        if let Some(value) = self.value {
            //TODO self.value.to_string(radix, Numbe)
        }

        String::new()
    }

    fn clear_all_and_append_command(&mut self, command: Command) {
        self.commands.clear();
        self.commands.push(command);
        self.is_sci_fmt = false;
        self.is_decimal = false;
        self.is_sci_fmt = false;
    }
}
