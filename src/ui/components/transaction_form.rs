use iced::{widget::{button, column, row, text, text_input, radio}, Element, Length, Alignment};
use crate::models::transaction::{Transaction, TransactionType};
use crate::ui::transaction::TransactionFormMessage;
use chrono::{Local};

pub struct TransactionForm {
    description: String,
    amount: String,
    transaction_type: TransactionType,
    date: String,
    category: String,
}

impl TransactionForm {
    pub fn new() -> Self {
        Self {
            description: String::new(),
            amount: String::new(),
            transaction_type: TransactionType::Expense,
            date: Local::now().format("%Y-%m-%d").to_string(),
            category: String::new(),
        }
    }

    pub fn update(&mut self, message: TransactionFormMessage) {
        match message {
            TransactionFormMessage::DescriptionChanged(description) => {
                self.description = description;
            }
            TransactionFormMessage::AmountChanged(amount) => {
                self.amount = amount;
            }
            TransactionFormMessage::TypeChanged(transaction_type) => {
                self.transaction_type = transaction_type;
            }
            TransactionFormMessage::DateChanged(date) => {
                self.date = date;
            }
            TransactionFormMessage::CategoryChanged(category) => {
                self.category = category;
            }
            TransactionFormMessage::Submit => {
                // Handle submission - this would be connected to the DB in a real implementation
                self.description = String::new();
                self.amount = String::new();
                self.transaction_type = TransactionType::Expense;
                self.date = Local::now().format("%Y-%m-%d").to_string();
                self.category = String::new();
            }
            TransactionFormMessage::Cancel => {
                // Reset the form
                self.description = String::new();
                self.amount = String::new();
                self.transaction_type = TransactionType::Expense;
                self.date = Local::now().format("%Y-%m-%d").to_string();
                self.category = String::new();
            }
        }
    }

    pub fn view(&self) -> Element<TransactionFormMessage> {
        let title = text("Transaction Form")
            .size(24)
            .width(Length::Fill)
            .horizontal_alignment(Alignment::Center);

        let description_input = column![
            text("Description:").size(16),
            text_input("Enter description", &self.description)
                .on_input(TransactionFormMessage::DescriptionChanged)
                .padding(10)
        ]
        .spacing(5);

        let amount_input = column![
            text("Amount:").size(16),
            text_input("Enter amount", &self.amount)
                .on_input(TransactionFormMessage::AmountChanged)
                .padding(10)
        ]
        .spacing(5);

        let type_selection = column![
            text("Type:").size(16),
            row![
                radio("Expense", TransactionType::Expense, Some(self.transaction_type), TransactionFormMessage::TypeChanged),
                radio("Income", TransactionType::Income, Some(self.transaction_type), TransactionFormMessage::TypeChanged)
            ]
            .spacing(10)
        ]
        .spacing(5);

        let date_input = column![
            text("Date:").size(16),
            text_input("YYYY-MM-DD", &self.date)
                .on_input(TransactionFormMessage::DateChanged)
                .padding(10)
        ]
        .spacing(5);

        let category_input = column![
            text("Category:").size(16),
            text_input("Enter category", &self.category)
                .on_input(TransactionFormMessage::CategoryChanged)
                .padding(10)
        ]
        .spacing(5);

        let submit_button = button(text("Save"))
            .on_press(TransactionFormMessage::Submit);

        let cancel_button = button(text("Cancel"))
            .on_press(TransactionFormMessage::Cancel);

        let delete_button = button(text("Delete"))
            .on_press(TransactionFormMessage::Cancel);

        let buttons = row![
            submit_button,
            cancel_button,
            delete_button
        ]
        .spacing(10);

        column![
            title,
            description_input,
            amount_input,
            type_selection,
            date_input,
            category_input,
            buttons
        ]
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
    }
}

impl From<Transaction> for TransactionForm {
    fn from(transaction: Transaction) -> Self {
        Self {
            description: transaction.description,
            amount: transaction.amount.to_string(),
            transaction_type: transaction.transaction_type,
            date: transaction.date.format("%Y-%m-%d").to_string(),
            category: transaction.category.unwrap_or_default(),
        }
    }
}
