use crate::models::transaction::{Transaction, TransactionType};
use iced::{widget::{button, column, container, row, text, text_input}, Element, Length};
use crate::ui::components::transaction_form::TransactionForm;
use crate::ui::components::nav::Nav;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TransactionMessage {
    AddTransaction,
    EditTransaction(Uuid),
    DeleteTransaction(Uuid),
    FormUpdated(TransactionFormMessage),
    FilterChanged(String),
    SortByChanged(SortBy),
}

#[derive(Debug, Clone)]
pub enum TransactionFormMessage {
    DescriptionChanged(String),
    AmountChanged(String),
    TypeChanged(TransactionType),
    DateChanged(String),
    CategoryChanged(String),
    Submit,
    Cancel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Date,
    Amount,
    Category,
}

pub struct TransactionView {
    transactions: Vec<Transaction>,
    transaction_form: TransactionForm,
    filter: String,
    sort_by: SortBy,
    editing: Option<Uuid>,
}

impl TransactionView {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            transaction_form: TransactionForm::new(),
            filter: String::new(),
            sort_by: SortBy::Date,
            editing: None,
        }
    }

    pub fn update(&mut self, message: TransactionMessage) {
        match message {
            TransactionMessage::AddTransaction => {
                self.transaction_form = TransactionForm::new();
                self.editing = None;
            }
            TransactionMessage::EditTransaction(id) => {
                if let Some(transaction) = self.transactions.iter().find(|t| t.id == id) {
                    self.transaction_form = TransactionForm::from(transaction.clone());
                    self.editing = Some(id);
                }
            }
            TransactionMessage::DeleteTransaction(id) => {
                self.transactions.retain(|t| t.id != id);
            }
            TransactionMessage::FormUpdated(msg) => {
                self.transaction_form.update(msg);
            }
            TransactionMessage::FilterChanged(filter) => {
                self.filter = filter;
            }
            TransactionMessage::SortByChanged(sort_by) => {
                self.sort_by = sort_by;
            }
        }
    }

    pub fn view(&self) -> Element<TransactionMessage> {
        let nav = Nav::new().view();
        
        let add_button = button("Add Transaction")
            .on_press(TransactionMessage::AddTransaction)
            .padding(10);
        
        let filter_input = text_input("Filter transactions...", &self.filter)
            .on_input(TransactionMessage::FilterChanged)
            .padding(10);
        
        let sort_controls = row![
            text("Sort by:").size(14),
            button("Date").on_press(TransactionMessage::SortByChanged(SortBy::Date)),
            button("Amount").on_press(TransactionMessage::SortByChanged(SortBy::Amount)),
            button("Category").on_press(TransactionMessage::SortByChanged(SortBy::Category)),
        ]
        .spacing(10);
        
        let transactions_list = self.transactions
            .iter()
            .filter(|t| t.description.to_lowercase().contains(&self.filter.to_lowercase()))
            .fold(column![].spacing(5), |column, transaction| {
                column.push(
                    container(
                        row![
                            text(&transaction.description).width(Length::FillPortion(3)),
                            text(format!("${:.2}", transaction.amount)).width(Length::FillPortion(1)),
                            text(transaction.category.as_ref().unwrap_or(&"None".to_string())).width(Length::FillPortion(2)),
                            text(transaction.date.format("%Y-%m-%d").to_string()).width(Length::FillPortion(2)),
                            button("Edit")
                                .on_press(TransactionMessage::EditTransaction(transaction.id))
                                .width(Length::Shrink),
                            button("Delete")
                                .on_press(TransactionMessage::DeleteTransaction(transaction.id))
                                .width(Length::Shrink)
                        ]
                        .spacing(10)
                        .padding(10)
                    )
                    .style(container::Style::default())
                    .padding(5)
                )
            });
        
        let form_view = if self.editing.is_some() {
            self.transaction_form.view().map(TransactionMessage::FormUpdated)
        } else {
            container(text("")).into()
        };
        
        column![
            nav,
            row![
                add_button,
                filter_input,
                sort_controls
            ].padding(10).spacing(20),
            transactions_list,
            form_view
        ]
        .padding(20)
        .spacing(20)
        .into()
    }
}
