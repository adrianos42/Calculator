use crate::engine::{expression_command::ExpCommand, history_display::HistoryDisplay};
//use std::rc::Rc;

#[derive(Clone)]
pub struct HistoryItem {
    tokens: Option<Vec<(String, Option<usize>)>>,
    commands: Option<Vec<ExpCommand>>,
    expression: String,
    result: String,
}

pub struct CalculatorHistory {
    history_items: Vec<HistoryItem>,
    max_history_size: usize,
}

impl HistoryDisplay for CalculatorHistory {
    fn add_to_history(
        &mut self,
        tokens: Vec<(String, Option<usize>)>,
        commands: Vec<ExpCommand>,
        result: &str,
    ) -> usize {
        let expression = get_generated_expression(&tokens);

        let history_item = HistoryItem {
            commands: Some(commands),
            tokens: Some(tokens),
            result: String::from(result),
            expression: expression,
        };

        self.add_item(history_item)
    }
}

impl CalculatorHistory {
    pub fn new(max_size: usize) -> Self {
        let history = Self {
            history_items: Vec::new(),
            max_history_size: max_size,
        };

        history
    }

    pub fn add_item(&mut self, history_item: HistoryItem) -> usize {
        if self.history_items.len() >= self.max_history_size {
            self.history_items.remove(0);
        }

        self.history_items.push(history_item);
        self.history_items.len() - 1
    }

    pub fn get_history(&self) -> &[HistoryItem] {
        self.history_items.as_slice()
    }

    pub fn get_history_item(&self, index: usize) -> Option<&HistoryItem> {
        self.history_items.get(index)
    }

    pub fn clear_history(&mut self) {
        self.history_items.clear();
    }

    pub fn remove_item(&mut self, index: usize) -> bool {
        if index < self.history_items.len() {
            self.history_items.remove(index);
            true
        } else {
            false
        }
    }

    pub fn max_history_size(&self) -> usize {
        self.max_history_size
    }
}

fn get_generated_expression(tokens: &[(String, Option<usize>)]) -> String {
    let mut expression = String::new();
    let mut is_first = true;

    for token in tokens {
        if is_first {
            is_first = false;
        } else {
            expression.push(' ');
        }

        expression += token.0.as_str();
    }

    expression
}