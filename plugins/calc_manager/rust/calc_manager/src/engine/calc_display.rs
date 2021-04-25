use super::expression_command::ExpCommand;

pub trait CalcDisplay {
    fn set_primary_display(&mut self, text: &str, is_error: bool);
    fn set_is_error(&mut self, value: bool) {}
    fn set_expression_display(&mut self, tokens: &[(String, Option<usize>)], commands: &[ExpCommand]);
    fn set_parenthesis_number(&mut self, count: usize);
    fn set_memorized_numbers(&mut self, memorized_numbers: Vec<String>);
    
    fn on_no_right_paren_added(&self);
    fn max_digits_reached(&self);
    fn binary_operation_received(&self);
    fn on_history_item_added(&self, added_item_index: usize);
    fn memory_item_changed(&self, index: usize);
    fn input_changed(&self);
}