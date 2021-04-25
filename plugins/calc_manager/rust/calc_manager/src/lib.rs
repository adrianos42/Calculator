//use calc_manager_types::{idl_impl::*, idl_types::*};
use std::{collections::HashMap, thread::sleep, time::Duration};
use std::sync::{Arc, RwLock};

pub use idl_types;
use idl_types::idl_internal::*;

//mod engine;
//mod engine_strings;
//mod manager;

pub struct Programmer {
    stream_instances: Arc<RwLock<i64>>,
    stream_subscriber: Option<Box<dyn StreamInstance + Send>>,
}

impl idl_types::idl_impl::ProgrammerInstance for Programmer {
    fn commands(
        &mut self,
        value: Box<dyn StreamInstance + Send>,
        stream_instance: Box<dyn StreamInstance + Send>,
    ) {
        let context = self.stream_instances.clone();
        value.wake_client();
        self.stream_subscriber = Some(value);

        std::thread::spawn(move || {
            let mut prev = 0;
            loop {
                sleep(Duration::from_millis(600));
                *context.write().unwrap() = prev;
                stream_instance.wake_client();
                if prev > 5 {
                    return;
                }
                prev += 1;
            }
        });
    }

    fn commands_stream(
        &mut self,
        stream_instance: Box<dyn StreamInstance + Send>,
        stream: StreamReceiver,
    ) -> StreamSender<i64> {
        match stream {
            StreamReceiver::Request => {
                let value = *self.stream_instances.read().unwrap();
                if value <= 5 {
                    StreamSender::Value(value)
                } else {
                    StreamSender::Done
                }
            }
            StreamReceiver::Close => StreamSender::Ok,
            StreamReceiver::Pause => StreamSender::Ok,
            StreamReceiver::Resume => StreamSender::Ok,
            _ => panic!(),
        }
    }

    fn commands_stream_sender(
        &mut self,
        stream_instance: Box<dyn StreamInstance + Send>,
        stream: StreamSender<i64>,
    ) -> StreamReceiver {
        match stream {
            StreamSender::Request => StreamReceiver::Start,
            StreamSender::Value(value) => {
                println!("from client {}", value + 100);
                StreamReceiver::Ok
            }
            StreamSender::Done => {
                println!("done");
                StreamReceiver::Ok
            }
            _ => panic!(),
        }
    }

    fn shaa(&mut self, name: (String, i64)) -> (i64, String)  {
        (name.1 + 2, format!("you're welcome, {}", name.0))
    }
}

impl Programmer {
    pub fn new() -> Self {
        Self {
            stream_instances: Default::default(),
            stream_subscriber: None,
        }
    }
}

// #[cfg(test)]
// mod calculator_manager_tests {
//     use crate::engine::{
//         calc_display::CalcDisplay, command::Command, expression_command::ExpCommand,
//     };
//     use crate::manager::number_formatting::*;
//     use crate::manager::*;

//     use std::rc::Rc;

//     #[derive(Default)]
//     struct CalculatorManagerDisplayTester {
//         primary_display: String,
//         expression: String,
//         parenthesis_display: usize,
//         is_error: bool,
//         memorized_numbers: Vec<String>,
//         max_digits_called_count: usize,
//         binary_operator_received_call_count: usize,
//     }

//     impl CalculatorManagerDisplayTester {
//         fn new() -> Self {
//             let mut result = Self::default();
//             result.reset();
//             result
//         }

//         fn reset(&mut self) {
//             self.is_error = false;
//             self.max_digits_called_count = 0;
//             self.binary_operator_received_call_count = 0;
//         }

//         fn get_primary_display(&self) -> &str {
//             self.primary_display.as_str()
//         }

//         fn get_expression(&self) -> &str {
//             self.expression.as_str()
//         }

//         fn get_memorized_numbers(&self) -> &[String] {
//             self.memorized_numbers.as_slice()
//         }

//         fn get_is_error(&self) -> bool {
//             self.is_error
//         }

//         fn get_max_digits_called_count(&self) -> usize {
//             self.max_digits_called_count
//         }

//         fn get_binary_operator_received_call_count(&self) -> usize {
//             self.binary_operator_received_call_count
//         }
//     }

//     impl CalcDisplay for CalculatorManagerDisplayTester {
//         fn set_primary_display(&mut self, text: &str, is_error: bool) {
//             self.primary_display = String::from(text);
//             self.is_error = is_error;
//         }

//         fn set_is_error(&mut self, value: bool) {
//             self.is_error = value;
//         }

//         fn set_expression_display(
//             &mut self,
//             tokens: &[(String, Option<usize>)],
//             commands: &[ExpCommand],
//         ) {
//             self.expression.clear();

//             for value in tokens {
//                 self.expression += value.0.as_str()
//             }
//         }

//         fn set_parenthesis_number(&mut self, count: usize) {
//             self.parenthesis_display = count;
//         }

//         fn on_no_right_paren_added(&self) {
//             // This method is used to create a narrator announcement when
//             // a close parenthesis cannot be added because there are no open parentheses.
//         }

//         fn max_digits_reached(&self) {
//             self.max_digits_reached();
//         }

//         fn binary_operation_received(&self) {
//             self.binary_operator_received_call_count += 1;
//         }

//         fn on_history_item_added(&self, added_item_index: usize) {}

//         fn set_memorized_numbers(&mut self, memorized_numbers: Vec<String>) {
//             self.memorized_numbers = memorized_numbers;
//         }

//         fn memory_item_changed(&self, index: usize) {}

//         fn input_changed(&self) {}
//     }

//     fn command_list_from_string_input(input: &str) -> Vec<Command> {
//         std::todo!()
//     }

//     fn text_max_digits_reached_scenario(input: &str) {}

//     struct TestDriver {
//         display_tester: Rc<CalculatorManagerDisplayTester>,
//         calculator_manager: CalculatorManager,
//     }

//     impl TestDriver {
//         fn new() -> Self {
//             let display_tester = Rc::new(CalculatorManagerDisplayTester::new());
//             let calculator_manager = CalculatorManager::new(display_tester.clone());

//             Self {
//                 calculator_manager,
//                 display_tester,
//             }
//         }

//         fn test_commands(
//             &mut self,
//             expected_primary: &str,
//             expected_expression: &str,
//             test_commands: &[Command],
//         ) {
//             for command in test_commands {
//                 if *command == Command::Nop {
//                     break;
//                 }

//                 self.calculator_manager.send_command(*command);
//             }

//             assert_eq!(expected_primary, self.display_tester.get_primary_display());

//             if expected_expression != "N/A" {
//                 assert_eq!(expected_expression, self.display_tester.get_expression());
//             }
//         }

//         fn test_commands_cleanup(
//             &mut self,
//             expected_primary: &str,
//             expected_expression: &str,
//             test_commands: &[Command],
//         ) {
//             self.calculator_manager.reset(None);

//             self.test_commands(expected_primary, expected_expression, test_commands);
//         }

//         fn test_commands_with_scientific(
//             &mut self,
//             expected_primary: &str,
//             expected_expression: &str,
//             test_commands: &[Command],
//         ) {
//             self.calculator_manager
//                 .send_command(Command::ModeScientific);

//             self.test_commands(expected_primary, expected_expression, test_commands);
//         }

//         fn test_commands_cleanup_with_scientific(
//             &mut self,
//             expected_primary: &str,
//             expected_expression: &str,
//             test_commands: &[Command],
//         ) {
//             self.calculator_manager.reset(None);

//             self.calculator_manager
//                 .send_command(Command::ModeScientific);

//             self.test_commands(expected_primary, expected_expression, test_commands);
//         }
//     }

//     #[test]
//     fn standard() {
//         let mut test_driver = TestDriver::new();

//         let commands1 = vec![
//             Command::N1,
//             Command::N2,
//             Command::N3,
//             Command::Pnt,
//             Command::N4,
//             Command::N5,
//             Command::N6,
//             Command::Nop,
//         ];
//         test_driver.test_commands("123.456", "", &commands1);

//         let commands2 = vec![Command::Add, Command::Nop];
//         test_driver.test_commands("0", "0 + ", &commands2);

//         let commands3 = vec![Command::Sqrt, Command::Nop];
//         test_driver.test_commands("0", "√(0)", &commands3);

//         let commands4 = vec![
//             Command::N2,
//             Command::Add,
//             Command::N3,
//             Command::Equ,
//             Command::N4,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands("7", "4 + 3=", &commands4);

//         let commands5 = vec![Command::N4, Command::Equ, Command::Nop];
//         test_driver.test_commands("4", "4=", &commands5);

//         let commands6 = vec![
//             Command::N2,
//             Command::N5,
//             Command::N6,
//             Command::Sqrt,
//             Command::Sqrt,
//             Command::Sqrt,
//             Command::Nop,
//         ];
//         test_driver.test_commands("2", "√(√(√(256)))", &commands6);

//         let commands7 = vec![
//             Command::N3,
//             Command::Sub,
//             Command::N6,
//             Command::Equ,
//             Command::Mul,
//             Command::N3,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands("-9", "-3 × 3=", &commands7);

//         let commands8 = vec![
//             Command::N9,
//             Command::Mul,
//             Command::N6,
//             Command::Sub,
//             Command::Centr,
//             Command::N8,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands("46", "9 × 6 - 8=", &commands8);

//         let commands9 = vec![
//             Command::N6,
//             Command::Mul,
//             Command::N6,
//             Command::Percent,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands("0.36", "6 × 0.06=", &commands9);

//         let commands10 = vec![
//             Command::N5,
//             Command::N0,
//             Command::Add,
//             Command::N2,
//             Command::N0,
//             Command::Percent,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands("60", "50 + 10=", &commands10);

//         let commands11 = vec![Command::N4, Command::Add, Command::Equ, Command::Nop];
//         test_driver.test_commands("8", "4 + 4=", &commands11);

//         let commands12 = vec![
//             Command::N5,
//             Command::Add,
//             Command::Mul,
//             Command::N3,
//             Command::Nop,
//         ];
//         test_driver.test_commands("3", "6 × 0.06= ", &commands12);

//         let commands13 = vec![
//             Command::N1,
//             Command::Exp,
//             Command::Sign,
//             Command::N9,
//             Command::N9,
//             Command::N9,
//             Command::N9,
//             Command::Div,
//             Command::N1,
//             Command::N0,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands("Overflow", "1.e-9999 ÷ ", &commands13);

//         let commands14 = vec![
//             Command::N5,
//             Command::N0,
//             Command::Add,
//             Command::N2,
//             Command::N0,
//             Command::Percent,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands("60", "50 + 10=", &commands14);

//         let commands15 = vec![
//             Command::N0,
//             Command::Div,
//             Command::N0,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands("Result is undefined", "0 ÷ ", &commands15);

//         let commands16 = vec![
//             Command::N1,
//             Command::Div,
//             Command::N0,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands("Cannot divide by zero", "1 ÷ ", &commands16);

//         let commands17 = vec![
//             Command::N1,
//             Command::N2,
//             Command::Add,
//             Command::N5,
//             Command::Centr,
//             Command::N2,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands("14", "12 + 2 + ", &commands17);

//         let commands18 = vec![
//             Command::N1,
//             Command::N0,
//             Command::N0,
//             Command::Sign,
//             Command::Rec,
//             Command::Nop,
//         ];
//         test_driver.test_commands("-0.01", "1/(-100)", &commands18);

//         let commands19 = vec![
//             Command::N1,
//             Command::N2,
//             Command::N3,
//             Command::Back,
//             Command::Back,
//             Command::Nop,
//         ];
//         test_driver.test_commands("1", "", &commands19);

//         let commands20 = vec![
//             Command::N1,
//             Command::N2,
//             Command::N3,
//             Command::Back,
//             Command::Back,
//             Command::Back,
//             Command::Nop,
//         ];
//         test_driver.test_commands("0", "", &commands20);

//         let commands21 = vec![
//             Command::N4,
//             Command::Sqrt,
//             Command::Sub,
//             Command::N2,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands("0", "√(4) - 2 + ", &commands21);

//         let commands22 = vec![
//             Command::N1,
//             Command::N0,
//             Command::N2,
//             Command::N4,
//             Command::Sqrt,
//             Command::Sub,
//             Command::N3,
//             Command::N2,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands("0", "√(1024) - 32 + ", &commands22);
//     }

//     #[test]
//     fn scientific() {
//         let mut test_driver = TestDriver::new();

//         let commands1 = vec![
//             Command::N1,
//             Command::N2,
//             Command::N3,
//             Command::Pnt,
//             Command::N4,
//             Command::N5,
//             Command::N6,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("123.456", "", &commands1);

//         let commands2 = vec![Command::Add, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("0", "0 + ", &commands2);

//         let commands3 = vec![Command::Sqrt, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("0", "√(0)", &commands3);

//         let commands4 = vec![
//             Command::N1,
//             Command::Add,
//             Command::N0,
//             Command::Mul,
//             Command::N2,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("1", "1 + 0 × 2=", &commands4);

//         let commands5 = vec![Command::N4, Command::Equ, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("4", "4=", &commands5);

//         let commands6 = vec![
//             Command::N2,
//             Command::N5,
//             Command::N6,
//             Command::Sqrt,
//             Command::Sqrt,
//             Command::Sqrt,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("2", "√(√(√(256)))", &commands6);

//         let commands7 = vec![
//             Command::N3,
//             Command::Sub,
//             Command::N6,
//             Command::Equ,
//             Command::Mul,
//             Command::N3,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("-9", "-3 × 3 + ", &commands7);

//         let commands8 = vec![
//             Command::N9,
//             Command::Mul,
//             Command::N6,
//             Command::Sub,
//             Command::Centr,
//             Command::N8,
//             Command::Mul,
//             Command::N2,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("38", "9 × 6 - 8 × 2 + ", &commands8);

//         let commands9 = vec![
//             Command::N6,
//             Command::Mul,
//             Command::N6,
//             Command::Sign,
//             Command::Sqrt,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("Invalid input", "6 × √(-6)", &commands9);

//         let commands10 = vec![
//             Command::N5,
//             Command::N0,
//             Command::Add,
//             Command::N2,
//             Command::N0,
//             Command::Rec,
//             Command::Sub,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("50.05", "50 + 1/(20) - ", &commands10);

//         let commands11 = vec![Command::N4, Command::Add, Command::Equ, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("8", "4 + 4=", &commands11);

//         let commands12 = vec![
//             Command::N5,
//             Command::Add,
//             Command::Mul,
//             Command::N3,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("3", "5 × ", &commands12);

//         let commands13 = vec![
//             Command::N1,
//             Command::Exp,
//             Command::Sign,
//             Command::N9,
//             Command::N9,
//             Command::N9,
//             Command::N9,
//             Command::Div,
//             Command::N1,
//             Command::N0,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("Overflow", "1.e-9999 ÷ ", &commands13);

//         let commands14 = vec![
//             Command::N5,
//             Command::N0,
//             Command::Add,
//             Command::N2,
//             Command::N0,
//             Command::Percent,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("60", "50 + 10=", &commands14);

//         let commands15 = vec![
//             Command::N0,
//             Command::Div,
//             Command::N0,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific(
//             "Result is undefined",
//             "0 ÷ ",
//             &commands15,
//         );

//         let commands16 = vec![
//             Command::N1,
//             Command::Div,
//             Command::N0,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific(
//             "Cannot divide by zero",
//             "1 ÷ ",
//             &commands16,
//         );

//         let commands17 = vec![
//             Command::N1,
//             Command::N2,
//             Command::Add,
//             Command::N5,
//             Command::Centr,
//             Command::N2,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("14", "12 + 2 + ", &commands17);

//         let commands18 = vec![
//             Command::N1,
//             Command::N0,
//             Command::N0,
//             Command::Sign,
//             Command::Rec,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("-0.01", "1/(-100)", &commands18);

//         let commands19 = vec![
//             Command::N1,
//             Command::N2,
//             Command::N3,
//             Command::Back,
//             Command::Back,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("1", "", &commands19);

//         let commands20 = vec![
//             Command::N1,
//             Command::N2,
//             Command::N3,
//             Command::Back,
//             Command::Back,
//             Command::Back,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("0", "", &commands20);

//         let commands21 = vec![
//             Command::N4,
//             Command::Sqrt,
//             Command::Sub,
//             Command::N2,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands("0", "√(4) - 2 + ", &commands21);

//         let commands22 = vec![Command::N0, Command::Sqrt, Command::Nop];
//         test_driver.test_commands("0", "√(0)", &commands22);

//         let commands23 = vec![
//             Command::N1,
//             Command::N0,
//             Command::N2,
//             Command::N4,
//             Command::Sqrt,
//             Command::Sub,
//             Command::N3,
//             Command::N2,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands("0", "√(1024) - 32 + ", &commands23);

//         let commands24 = vec![
//             Command::N2,
//             Command::N5,
//             Command::N7,
//             Command::Sqrt,
//             Command::Sqrt,
//             Command::Sqrt,
//             Command::Nop,
//         ];

//         test_driver.test_commands_cleanup_with_scientific(
//             "2.0009748976330773374220277351385",
//             "√(√(√(257)))",
//             &commands24,
//         );
//     }

//     #[test]
//     fn scientific2() {
//         let mut test_driver = TestDriver::new();

//         let commands1 = vec![Command::N1, Command::N2, Command::Sqr, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("144", "sqr(12)", &commands1);

//         let commands2 = vec![Command::N5, Command::Fac, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("120", "fact(5)", &commands2);

//         let commands3 = vec![
//             Command::N5,
//             Command::Pwr,
//             Command::N2,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("25", "5 ^ 2 + ", &commands3);

//         let commands4 = vec![
//             Command::N8,
//             Command::Root,
//             Command::N3,
//             Command::Mul,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("2", "8 yroot 3 × ", &commands4);

//         let commands5 = vec![Command::N8, Command::Cub, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("512", "cube(8)", &commands5);

//         let commands6 = vec![Command::N8, Command::Cub, Command::CubeRoot, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("8", "cuberoot(cube(8))", &commands6);

//         let commands7 = vec![Command::N1, Command::N0, Command::Log, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("1", "log(10)", &commands7);

//         let commands8 = vec![Command::N5, Command::Pow10, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("100,000", "10^(5)", &commands8);

//         let commands9 = vec![Command::N1, Command::N0, Command::Ln, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "2.3025850929940456840179914546844",
//             "ln(10)",
//             &commands9,
//         );

//         let commands10 = vec![Command::N1, Command::Sin, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "0.01745240643728351281941897851632",
//             "sin₀(1)",
//             &commands10,
//         );

//         let commands11 = vec![Command::N1, Command::Cos, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "0.99984769515639123915701155881391",
//             "cos₀(1)",
//             &commands11,
//         );

//         let commands12 = vec![Command::N1, Command::Tan, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "0.01745506492821758576512889521973",
//             "tan₀(1)",
//             &commands12,
//         );

//         let commands13 = vec![Command::N1, Command::Sin, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("90", "sin₀⁻¹(1)", &commands13);

//         let commands14 = vec![Command::N1, Command::Acos, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("0", "cos₀⁻¹(1)", &commands14);

//         let commands15 = vec![Command::N1, Command::Atan, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("45", "tan₀⁻¹(1)", &commands15);

//         let commands16 = vec![Command::N2, Command::Powe, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "7.389056098930650227230427460575",
//             "e^(2)",
//             &commands16,
//         );

//         let commands17 = vec![
//             Command::N5,
//             Command::Pwr,
//             Command::N0,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands("1", "5 ^ 0 + ", &commands17);

//         let commands18 = vec![
//             Command::N0,
//             Command::Pwr,
//             Command::N0,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands("1", "0 ^ 0 + ", &commands18);

//         let commands19 = vec![
//             Command::N2,
//             Command::N7,
//             Command::Sign,
//             Command::Root,
//             Command::N3,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("-3", "-27 yroot 3 + ", &commands19);
//         let commands20 = vec![
//             Command::N8,
//             Command::Pwr,
//             Command::OpenP,
//             Command::N2,
//             Command::Div,
//             Command::N3,
//             Command::CloseP,
//             Command::Sub,
//             Command::N4,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("0", "8 ^ (2 ÷ 3) - 4 + ", &commands20);

//         let commands21 = vec![
//             Command::N4,
//             Command::Pwr,
//             Command::OpenP,
//             Command::N3,
//             Command::Div,
//             Command::N2,
//             Command::CloseP,
//             Command::Sub,
//             Command::N8,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("0", "4 ^ (3 ÷ 2) - 8 + ", &commands21);

//         let commands22 = vec![
//             Command::N1,
//             Command::N0,
//             Command::Pwr,
//             Command::N1,
//             Command::Pnt,
//             Command::N2,
//             Command::N3,
//             Command::N4,
//             Command::N5,
//             Command::N6,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific(
//             "17.161687912241792074207286679393",
//             "10 ^ 1.23456 + ",
//             &commands22,
//         );

//         let commands23 = vec![Command::N1, Command::Sec, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "1.0001523280439076654284264342126",
//             "sec₀(1)",
//             &commands23,
//         );

//         let commands24 = vec![Command::N1, Command::Csc, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "57.298688498550183476612683735174",
//             "csc₀(1)",
//             &commands24,
//         );

//         let commands25 = vec![Command::N1, Command::Cot, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "57.289961630759424687278147537113",
//             "cot₀(1)",
//             &commands25,
//         );

//         let commands26 = vec![Command::N1, Command::Asec, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("0", "sec₀⁻¹(1)", &commands26);

//         let commands27 = vec![Command::N1, Command::Acsc, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("90", "csc₀⁻¹(1)", &commands27);

//         let commands28 = vec![Command::N1, Command::Acot, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("45", "cot₀⁻¹(1)", &commands28);

//         let commands29 = vec![Command::N1, Command::Sech, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "0.64805427366388539957497735322615",
//             "sech(1)",
//             &commands29,
//         );

//         let commands30 = vec![Command::N1, Command::Csch, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "0.85091812823932154513384276328718",
//             "csch(1)",
//             &commands30,
//         );

//         let commands31 = vec![Command::N1, Command::Coth, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "1.3130352854993313036361612469308",
//             "coth(1)",
//             &commands31,
//         );

//         let commands32 = vec![Command::N1, Command::Asech, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("0", "sech⁻¹(1)", &commands32);

//         let commands33 = vec![Command::N1, Command::Acsch, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "0.88137358701954302523260932497979",
//             "csch⁻¹(1)",
//             &commands33,
//         );

//         let commands34 = vec![Command::N2, Command::Acoth, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific(
//             "0.54930614433405484569762261846126",
//             "coth⁻¹(2)",
//             &commands34,
//         );

//         let commands35 = vec![Command::N8, Command::Pow2, Command::Nop];
//         test_driver.test_commands("256", "2^(8)", &commands35);

//         let commands36 = vec![Command::Rand, Command::Ceil, Command::Nop];
//         test_driver.test_commands("1", "N/A", &commands36);

//         let commands37 = vec![Command::Rand, Command::Floor, Command::Nop];
//         test_driver.test_commands("0", "N/A", &commands37);

//         let commands38 = vec![Command::Rand, Command::Sign, Command::Ceil, Command::Nop];
//         test_driver.test_commands("0", "N/A", &commands38);

//         let commands39 = vec![Command::Rand, Command::Sign, Command::Floor, Command::Nop];
//         test_driver.test_commands("-1", "N/A", &commands39);

//         let commands40 = vec![
//             Command::N3,
//             Command::Pnt,
//             Command::N8,
//             Command::Floor,
//             Command::Nop,
//         ];
//         test_driver.test_commands("3", "floor(3.8)", &commands40);

//         let commands41 = vec![
//             Command::N3,
//             Command::Pnt,
//             Command::N8,
//             Command::Ceil,
//             Command::Nop,
//         ];
//         test_driver.test_commands("4", "ceil(3.8)", &commands41);

//         let commands42 = vec![
//             Command::N5,
//             Command::LogBaseY,
//             Command::N3,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands("1.464973520717927", "5 log base 3 + ", &commands42);
//     }

//     #[test]
//     fn scientific_parenthesis() {
//         let mut test_driver = TestDriver::new();

//         let commands1 = vec![
//             Command::N1,
//             Command::Add,
//             Command::OpenP,
//             Command::Add,
//             Command::N3,
//             Command::CloseP,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("3", "1 + (0 + 3)", &commands1);

//         let commands2 = vec![
//             Command::OpenP,
//             Command::OpenP,
//             Command::N1,
//             Command::N2,
//             Command::CloseP,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("12", "((12)", &commands2);

//         let commands3 = vec![
//             Command::N1,
//             Command::N2,
//             Command::CloseP,
//             Command::CloseP,
//             Command::OpenP,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("12", "(", &commands3);

//         let commands4 = vec![
//             Command::N2,
//             Command::OpenP,
//             Command::N2,
//             Command::CloseP,
//             Command::Add,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("2", "(2) + ", &commands4);

//         let commands5 = vec![
//             Command::N2,
//             Command::OpenP,
//             Command::N2,
//             Command::CloseP,
//             Command::Add,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("4", "(2) + 2=", &commands5);
//     }

//     #[test]
//     fn scientific_error() {
//         let mut test_driver = TestDriver::new();

//         let commands1 = vec![
//             Command::N1,
//             Command::Div,
//             Command::N0,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific(
//             "Cannot divide by zero",
//             "1 ÷ ",
//             &commands1,
//         );
//         assert!(test_driver.display_tester.get_is_error());

//         let commands2 = vec![Command::N2, Command::Sign, Command::Log, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("Invalid input", "log(-2)", &commands2);
//         assert!(test_driver.display_tester.get_is_error());

//         let commands3 = vec![
//             Command::N0,
//             Command::Div,
//             Command::N0,
//             Command::Equ,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific(
//             "Result is undefined",
//             "0 ÷ ",
//             &commands3,
//         );
//         assert!(test_driver.display_tester.get_is_error());

//         // Do the same tests for the basic calculator
//         test_driver.test_commands("Cannot divide by zero", "1 ÷ ", &commands1);
//         assert!(test_driver.display_tester.get_is_error());
//         test_driver.test_commands("Invalid input", "log(-2)", &commands2);
//         assert!(test_driver.display_tester.get_is_error());
//         test_driver.test_commands("Result is undefined", "0 ÷ ", &commands3);
//         assert!(test_driver.display_tester.get_is_error());
//     }

//     #[test]
//     fn scientific_mode_change() {
//         let mut test_driver = TestDriver::new();

//         let commands1 = vec![Command::Rad, Command::Pi, Command::Sin, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("0", "N/A", &commands1);

//         let commands2 = vec![Command::Rad, Command::Pi, Command::Cos, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("-1", "N/A", &commands2);

//         let commands3 = vec![Command::Rad, Command::Pi, Command::Tan, Command::Nop];
//         test_driver.test_commands_cleanup_with_scientific("0", "N/A", &commands3);

//         let commands4 = vec![
//             Command::Grad,
//             Command::N4,
//             Command::N0,
//             Command::N0,
//             Command::Sin,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("0", "N/A", &commands4);

//         let commands5 = vec![
//             Command::Grad,
//             Command::N4,
//             Command::N0,
//             Command::N0,
//             Command::Cos,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("1", "N/A", &commands5);

//         let commands6 = vec![
//             Command::Grad,
//             Command::N4,
//             Command::N0,
//             Command::N0,
//             Command::Tan,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup_with_scientific("0", "N/A", &commands6);
//     }

//     #[test]
//     fn programmer() {
//         let mut test_driver = TestDriver::new();

//         let commands1 = vec![Command::N1, Command::N2, Command::N3, Command::Nop];
//         test_driver.test_commands_cleanup("123", "", &commands1);

//         let commands2 = vec![Command::ModeScientific, Command::Nop];
//         test_driver.test_commands_cleanup("0", "", &commands2);

//         let commands3 = vec![Command::N1, Command::N2, Command::N3, Command::Nop];
//         test_driver.test_commands_cleanup("123", "", &commands3);

//         let commands4 = vec![Command::ModeProgrammer, Command::Nop];
//         test_driver.test_commands_cleanup("0", "", &commands4);

//         let commands5 = vec![Command::N1, Command::N2, Command::N3, Command::Nop];
//         test_driver.test_commands_cleanup("123", "", &commands5);

//         let commands6 = vec![Command::ModeScientific, Command::Nop];
//         test_driver.test_commands_cleanup("0", "", &commands6);

//         let commands7 = vec![Command::N6, Command::N7, Command::Add, Command::Nop];
//         test_driver.test_commands_cleanup("67", "67 + ", &commands7);

//         let commands8 = vec![Command::ModeBasic, Command::Nop];
//         test_driver.test_commands_cleanup("0", "", &commands8);
//     }

//     #[test]
//     fn mode_change() {
//         let mut test_driver = TestDriver::new();

//         let commands1 = vec![
//             Command::ModeProgrammer,
//             Command::N5,
//             Command::N3,
//             Command::Nand,
//             Command::N8,
//             Command::N3,
//             Command::And,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup("-18", "53 NAND 83 AND ", &commands1);

//         let commands2 = vec![
//             Command::ModeProgrammer,
//             Command::N5,
//             Command::N3,
//             Command::Nor,
//             Command::N8,
//             Command::N3,
//             Command::And,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup("-120", "53 NOR 83 AND ", &commands2);

//         let commands3 = vec![
//             Command::ModeProgrammer,
//             Command::N5,
//             Command::Lshf,
//             Command::N1,
//             Command::And,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup("10", "5 Lsh 1 AND ", &commands3);

//         let commands5 = vec![
//             Command::ModeProgrammer,
//             Command::N5,
//             Command::Rshfl,
//             Command::N1,
//             Command::And,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup("2", "5 Rsh 1 AND ", &commands5);

//         let commands6 = vec![
//             Command::ModeProgrammer,
//             Command::BinPos63,
//             Command::Rshf,
//             Command::N5,
//             Command::N6,
//             Command::And,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup("-128", "-9223372036854775808 Rsh 56 AND ", &commands6);

//         let commands7 = vec![
//             Command::ModeProgrammer,
//             Command::N1,
//             Command::Rol,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup("2", "RoL(1)", &commands7);

//         let commands8 = vec![
//             Command::ModeProgrammer,
//             Command::N1,
//             Command::Ror,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup("-9,223,372,036,854,775,808", "RoR(1)", &commands8);

//         let commands9 = vec![
//             Command::ModeProgrammer,
//             Command::N1,
//             Command::Rorc,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup("0", "RoR(1)", &commands9);

//         let commands10 = vec![
//             Command::ModeProgrammer,
//             Command::N1,
//             Command::Rorc,
//             Command::Rorc,
//             Command::Nop,
//         ];
//         test_driver.test_commands_cleanup("-9,223,372,036,854,775,808", "RoR(RoR(1))", &commands10);
//     }

//     // #[test]
//     // fn memory() {
//     //     let scientificCalculatorTest52 = vec![Command::N1, Command::Store, Command::Nop ];
//     //     wstring expectedPrimaryDisplayTestScientific52("1");
//     //     wstring expectedExpressionDisplayTestScientific52("");

//     //     let scientificCalculatorTest53 = vec![Command::N1, Command::Nop ];
//     //     wstring expectedPrimaryDisplayTestScientific53("1");
//     //     wstring expectedExpressionDisplayTestScientific53("");

//     //     CalculatorManagerDisplayTester* pCalculatorDisplay = (CalculatorManagerDisplayTester*)calculator_display_tester.get();
//     //     wstring resultPrimary = "";
//     //     wstring resultExpression = "";

//     //     Cleanup();
//     //     ExecuteCommands(scientificCalculatorTest52);
//     //     resultPrimary = pCalculatorDisplay->GetPrimaryDisplay();
//     //     resultExpression = pCalculatorDisplay->GetExpression();
//     //     assert_eq!(expectedPrimaryDisplayTestScientific52, resultPrimary);

//     //     Cleanup();
//     //     ExecuteCommands(scientificCalculatorTest53);
//     //     calculator_manager->MemorizeNumber();
//     //     calculator_manager->SendCommand(Command::Clear);
//     //     calculator_manager->MemorizedNumberLoad(0);
//     //     resultPrimary = pCalculatorDisplay->GetPrimaryDisplay();
//     //     resultExpression = pCalculatorDisplay->GetExpression();
//     //     assert_eq!(expectedPrimaryDisplayTestScientific52, resultPrimary);

//     //     Cleanup();
//     //     calculator_manager->SendCommand(Command::N1);
//     //     calculator_manager->MemorizeNumber();
//     //     calculator_manager->SendCommand(Command::Clear);
//     //     calculator_manager->SendCommand(Command::N2);
//     //     calculator_manager->MemorizeNumber();
//     //     calculator_manager->SendCommand(Command::Clear);
//     //     calculator_manager->MemorizedNumberLoad(1);
//     //     resultPrimary = pCalculatorDisplay->GetPrimaryDisplay();
//     //     assert_eq!(wstring("1"), resultPrimary);

//     //     calculator_manager->MemorizedNumberLoad(0);
//     //     resultPrimary = pCalculatorDisplay->GetPrimaryDisplay();
//     //     assert_eq!(wstring("2"), resultPrimary);

//     //     Cleanup();
//     //     calculator_manager->SendCommand(Command::N1);
//     //     calculator_manager->SendCommand(Command::Sign);
//     //     calculator_manager->MemorizeNumber();
//     //     calculator_manager->SendCommand(Command::Add);
//     //     calculator_manager->SendCommand(Command::N2);
//     //     calculator_manager->SendCommand(Command::Equ);
//     //     calculator_manager->MemorizeNumber();
//     //     calculator_manager->SendCommand(Command::Mul);
//     //     calculator_manager->SendCommand(Command::N2);
//     //     calculator_manager->MemorizeNumber();

//     //     vector<wstring> memorizedNumbers = pCalculatorDisplay->GetMemorizedNumbers();

//     //     vector<wstring> expectedMemorizedNumbers;
//     //     expectedMemorizedNumbers.push_back("2");
//     //     expectedMemorizedNumbers.push_back("1");
//     //     expectedMemorizedNumbers.push_back("-1");

//     //     bool isEqual = false;
//     //     if (memorizedNumbers.size() < expectedMemorizedNumbers.size())
//     //     {
//     //         isEqual = std::equal(memorizedNumbers.begin(), memorizedNumbers.end(), expectedMemorizedNumbers.begin());
//     //     }
//     //     else
//     //     {
//     //         isEqual = std::equal(expectedMemorizedNumbers.begin(), expectedMemorizedNumbers.end(), memorizedNumbers.begin());
//     //     }
//     //     assert!(isEqual);

//     //     calculator_manager->SendCommand(Command::Clear);
//     //     calculator_manager->SendCommand(Command::N2);
//     //     calculator_manager->MemorizedNumberAdd(0);
//     //     calculator_manager->MemorizedNumberAdd(1);
//     //     calculator_manager->MemorizedNumberAdd(2);

//     //     memorizedNumbers = pCalculatorDisplay->GetMemorizedNumbers();

//     //     expectedMemorizedNumbers.clear();
//     //     expectedMemorizedNumbers.push_back("4");
//     //     expectedMemorizedNumbers.push_back("3");
//     //     expectedMemorizedNumbers.push_back("1");

//     //     if (memorizedNumbers.size() < expectedMemorizedNumbers.size())
//     //     {
//     //         isEqual = std::equal(memorizedNumbers.begin(), memorizedNumbers.end(), expectedMemorizedNumbers.begin());
//     //     }
//     //     else
//     //     {
//     //         isEqual = std::equal(expectedMemorizedNumbers.begin(), expectedMemorizedNumbers.end(), memorizedNumbers.begin());
//     //     }
//     //     assert!(isEqual);

//     //     calculator_manager->SendCommand(Command::Clear);
//     //     calculator_manager->SendCommand(Command::N1);
//     //     calculator_manager->SendCommand(Command::Pnt);
//     //     calculator_manager->SendCommand(Command::N5);

//     //     calculator_manager->MemorizedNumberSubtract(0);
//     //     calculator_manager->MemorizedNumberSubtract(1);
//     //     calculator_manager->MemorizedNumberSubtract(2);

//     //     memorizedNumbers = pCalculatorDisplay->GetMemorizedNumbers();

//     //     expectedMemorizedNumbers.clear();
//     //     expectedMemorizedNumbers.push_back("2.5");
//     //     expectedMemorizedNumbers.push_back("1.5");
//     //     expectedMemorizedNumbers.push_back("-0.5");

//     //     if (memorizedNumbers.size() < expectedMemorizedNumbers.size())
//     //     {
//     //         isEqual = std::equal(memorizedNumbers.begin(), memorizedNumbers.end(), expectedMemorizedNumbers.begin());
//     //     }
//     //     else
//     //     {
//     //         isEqual = std::equal(expectedMemorizedNumbers.begin(), expectedMemorizedNumbers.end(), memorizedNumbers.begin());
//     //     }
//     //     assert!(isEqual);

//     //     // Memorizing 101 numbers, which exceeds the limit.
//     //     Cleanup();
//     //     for (int i = 0; i < 101; i++)
//     //     {
//     //         calculator_manager->SendCommand(Command::N1);
//     //         calculator_manager->MemorizeNumber();
//     //     }

//     //     memorizedNumbers = pCalculatorDisplay->GetMemorizedNumbers();
//     //     assert_eq!((size_t)100, memorizedNumbers.size());

//     //     // Memorizing new number, which should show up at the top of the memory
//     //     calculator_manager->SendCommand(Command::N2);
//     //     calculator_manager->MemorizeNumber();
//     //     memorizedNumbers = pCalculatorDisplay->GetMemorizedNumbers();
//     //     assert_eq!(wstring("2"), memorizedNumbers.at(0));

//     //     // Test for trying to memorize invalid value
//     //     calculator_manager->SendCommand(Command::N2);
//     //     calculator_manager->SendCommand(Command::Sign);
//     //     calculator_manager->SendCommand(Command::Sqrt);
//     //     calculator_manager->MemorizeNumber();
//     // }

//     #[test]
//     fn max_digits_reached() {
//         text_max_digits_reached_scenario("1,234,567,891,011,1213");
//     }

//     #[test]
//     fn max_digits_reached_leading_decimal() {
//         text_max_digits_reached_scenario("0.12345678910111213");
//     }

//     #[test]
//     fn max_digits_reached_trailing_decimal() {
//         text_max_digits_reached_scenario("123,456,789,101,112.13");
//     }

//     #[test]
//     fn formatting_trim_trailing_zeros() {
//         assert_eq!(trim_trailing_zeros("2.1032100000000").as_str(), "2.10321");
//         assert_eq!(trim_trailing_zeros("-122.123200").as_str(), "-122.1232");
//         assert_eq!(trim_trailing_zeros("0.0001200").as_str(), "0.00012");
//         assert_eq!(trim_trailing_zeros("12.000").as_str(), "12");
//         assert_eq!(trim_trailing_zeros("-12.00000").as_str(), "-12");
//         assert_eq!(trim_trailing_zeros("0.000").as_str(), "0");
//         assert_eq!(trim_trailing_zeros("322423").as_str(), "322423");
//     }

//     // #[test]
//     // fn formatting_get_number_digits() {
//     //      wstring number = "2.10321";
//     //     unsigned int digitsCount = GetNumberDigits(number);
//     //     assert_eq!(digitsCount, 6);
//     //     number = "-122.1232";
//     //     digitsCount = GetNumberDigits(number);
//     //     assert_eq!(digitsCount, 7);
//     //     number = "-3432";
//     //     digitsCount = GetNumberDigits(number);
//     //     assert_eq!(digitsCount, 4);
//     //     number = "0";
//     //     digitsCount = GetNumberDigits(number);
//     //     assert_eq!(digitsCount, 1);
//     //     number = "0.0001223";
//     //     digitsCount = GetNumberDigits(number);
//     //     assert_eq!(digitsCount, 8);
//     // }

//     // #[test]
//     // fn formatting_get_number_digits_whole_number_part() {
//     //     unsigned int digitsCount = GetNumberDigitsWholeNumberPart(2.10321);
//     //     assert_eq!(digitsCount, 1);
//     //     digitsCount = GetNumberDigitsWholeNumberPart(-122.1232);
//     //     assert_eq!(digitsCount, 3);
//     //     digitsCount = GetNumberDigitsWholeNumberPart(-3432);
//     //     assert_eq!(digitsCount, 4);
//     //     digitsCount = GetNumberDigitsWholeNumberPart(0);
//     //     assert_eq!(digitsCount, 1);
//     //     digitsCount = GetNumberDigitsWholeNumberPart(324328412837382);
//     //     assert_eq!(digitsCount, 15);
//     //     digitsCount = GetNumberDigitsWholeNumberPart(324328412837382.232213214324234);
//     //     assert_eq!(digitsCount, 15);
//     //     digitsCount = GetNumberDigitsWholeNumberPart(0.032);
//     //     assert_eq!(digitsCount, 1);
//     //     digitsCount = GetNumberDigitsWholeNumberPart(0.00000000000000000001);
//     //     assert_eq!(digitsCount, 1);
//     // }

//     // #[test]
//     // fn formatting_round_significant_digits() {
//     //      wstring result = RoundSignificantDigits(12.342343242, 3);
//     //     assert_eq!(result, "12.342");
//     //     result = RoundSignificantDigits(12.3429999, 3);
//     //     assert_eq!(result, "12.343");
//     //     result = RoundSignificantDigits(12.342500001, 3);
//     //     assert_eq!(result, "12.343");
//     //     result = RoundSignificantDigits(-2312.1244243346454345, 5);
//     //     assert_eq!(result, "-2312.12442");
//     //     result = RoundSignificantDigits(0.3423432423, 5);
//     //     assert_eq!(result, "0.34234");
//     //     result = RoundSignificantDigits(0.3423, 7);
//     //     assert_eq!(result, "0.3423000");
//     // }

//     // #[test]
//     // fn formatting_to_scientific_number() {
//     //      wstring result = ToScientificNumber(3423);
//     //     assert_eq!(result, "3.423000e+03");
//     //     result = ToScientificNumber(-21);
//     //     assert_eq!(result, "-2.100000e+01");
//     //     result = ToScientificNumber(0.0232);
//     //     assert_eq!(result, "2.320000e-02");
//     //     result = ToScientificNumber(-0.00921);
//     //     assert_eq!(result, "-9.210000e-03");
//     //     result = ToScientificNumber(2343243345677);
//     //     assert_eq!(result, "2.343243e+12");
//     //     result = ToScientificNumber(-3432474247332942);
//     //     assert_eq!(result, "-3.432474e+15");
//     //     result = ToScientificNumber(0.000000003432432);
//     //     assert_eq!(result, "3.432432e-09");
//     //     result = ToScientificNumber(-0.000000003432432);
//     //     assert_eq!(result, "-3.432432e-09");
//     // }

//     // #[test]
//     // fn operator_received() {
//     //     CalculatorManagerDisplayTester* pCalculatorDisplay = (CalculatorManagerDisplayTester *)calculator_display_tester.get();

//     //      assert_eq!(0, pCalculatorDisplay->GetBinaryOperatorReceivedCallCount());

//     //      calculator_manager->SetStandardMode();
//     //      ExecuteCommands({
//     //          Command::N1,
//     //          Command::Add
//     //      });

//     //      wstring display = pCalculatorDisplay->GetPrimaryDisplay();
//     //      assert_eq!("1", display);

//     //      // Verify BinaryOperatorReceived
//     //      assert_eq!(1, pCalculatorDisplay->GetBinaryOperatorReceivedCallCount());
//     // }

//     // #[test]
//     // fn operator_received_multiple() {
//     //     CalculatorManagerDisplayTester* pCalculatorDisplay = (CalculatorManagerDisplayTester *)calculator_display_tester.get();

//     //      assert_eq!(0, pCalculatorDisplay->GetBinaryOperatorReceivedCallCount());

//     //      calculator_manager->SetStandardMode();
//     //      ExecuteCommands({
//     //          Command::N1,
//     //          Command::Add,
//     //          Command::Sub,
//     //          Command::Mul
//     //      });

//     //      wstring display = pCalculatorDisplay->GetPrimaryDisplay();
//     //      assert_eq!("1", display);

//     //      // Verify BinaryOperatorReceived
//     //      assert_eq!(3, pCalculatorDisplay->GetBinaryOperatorReceivedCallCount());
//     // }

//     // #[test]
//     // fn operator_received_long_input() {
//     //     CalculatorManagerDisplayTester* pCalculatorDisplay = (CalculatorManagerDisplayTester *)calculator_display_tester.get();

//     //      assert_eq!(0, pCalculatorDisplay->GetBinaryOperatorReceivedCallCount());

//     //      calculator_manager->SetStandardMode();
//     //      ExecuteCommands({
//     //          Command::N1,
//     //          Command::Add,
//     //          Command::N2,
//     //          Command::Mul,
//     //          Command::N1,
//     //          Command::N0,
//     //          Command::Sub,
//     //          Command::N5,
//     //          Command::Div,
//     //          Command::N5,
//     //          Command::Equ
//     //      });

//     //      wstring display = pCalculatorDisplay->GetPrimaryDisplay();
//     //      assert_eq!("5", display);

//     //      // Verify BinaryOperatorReceived
//     //      assert_eq!(4, pCalculatorDisplay->GetBinaryOperatorReceivedCallCount());
//     // }
// }
