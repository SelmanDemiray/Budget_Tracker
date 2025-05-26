use crate::models::{Budget};
use iced::{Element, widget::{button, column, container, text}};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum BudgetMessage {
    NameChanged(String),
    AmountChanged(String),
    Save,
    Delete(Uuid),
    Edit(Budget),
    Cancel,
}

pub struct BudgetView {
    budgets: Vec<Budget>,
    editing_budget: Option<Budget>,
    name_input: String,
    amount_input: String,
}

impl BudgetView {
    pub fn new() -> Self {
        Self {
            budgets: Vec::new(),
            editing_budget: None,
            name_input: String::new(),
            amount_input: String::new(),
        }
    }
    
    pub fn update(&mut self, message: BudgetMessage) {
        match message {
            BudgetMessage::NameChanged(name) => {
                self.name_input = name;
            },
            BudgetMessage::AmountChanged(amount) => {
                self.amount_input = amount;
            },
            BudgetMessage::Save => {
                // Budget save logic here
            },
            BudgetMessage::Delete(_id) => {
                // Budget delete logic here
            },
            BudgetMessage::Edit(budget) => {
                self.editing_budget = Some(budget.clone());
                self.name_input = budget.name;
                self.amount_input = budget.amount.to_string();
            },
            BudgetMessage::Cancel => {
                self.editing_budget = None;
                self.name_input = String::new();
                self.amount_input = String::new();
            },
        }
    }
    
    pub fn view(&self) -> Element<BudgetMessage> {
        let content = column!()
            .spacing(20)
            .push(text("Budgets").size(30))
            .push(
                button("Add Budget")
                    .on_press(BudgetMessage::Cancel) // This resets the form
            );
        
        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .into()
    }
}
