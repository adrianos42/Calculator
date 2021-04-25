use super::expression_command::ExpCommand;

pub trait HistoryDisplay {
    fn add_to_history(
        &mut self,
        tokens: Vec<(String, Option<usize>)>,
        commands: Vec<ExpCommand>,
        result: &str,
    ) -> usize;
}
