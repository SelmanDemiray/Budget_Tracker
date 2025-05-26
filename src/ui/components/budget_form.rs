use chrono::NaiveDate;
use crate::models::{Budget, Category};
use iced::widget::{button, container};
use crate::ui::budget::BudgetMessage;
use iced::{Element, widget::{column, row, text, text_input, pick_list}, Length, Alignment};
use iced_aw::date_picker;
use crate::app::{Message, TransactionType};

#[derive(Debug, Clone, Default)]
pub struct BudgetFormState {
    pub name: String,
    pub amount: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub category_id: Option<String>,
    pub is_editing: bool,
    pub editing_id: Option<String>,
    pub show_start_date_picker: bool,
    pub show_end_date_picker: bool,
    pub error: Option<String>,
    pub description: String,
    pub transaction_type: TransactionType,
}

pub fn budget_form_view<'a>(
    budget: &'a Budget,
    categories: &'a [Category],
    state: &BudgetFormState,
    is_editing: bool,
) -> Element<'a, BudgetMessage> {
    let name_input = text_input::TextInput::new(
        "Budget Name",
        &state.name_input,
        BudgetMessage::NameChanged,
    )
    .padding(10);
    
    let amount_input = text_input::TextInput::new(
        "Amount",
        &state.amount_input,
        BudgetMessage::AmountChanged,
    )
    .padding(10);
    
    // Category picker
    let category_options: Vec<(String, String)> = categories
        .iter()
        .map(|c| (c.id.clone(), c.name.clone()))
        .collect();
        
    let category_picker = pick_list::PickList::new(
        &category_options[..],
        budget.category_id.clone(),
        |id| BudgetMessage::CategorySelected(id),
    );
    
    // Start date picker
    let start_date_button = button::Button::new(
        text::Text::new(format!("Start Date: {}", budget.start_date.format("%Y-%m-%d")))
    )
    .on_press(BudgetMessage::OpenStartDatePicker(true))
    .padding(10);
    
    let start_datepicker = if state.show_start_date_picker {
        date_picker::DatePicker::new(budget.start_date)
            .show(true)
            .on_cancel(|_| BudgetMessage::OpenStartDatePicker(false))
            .on_submit(BudgetMessage::StartDateSelected)
    } else {
        date_picker::DatePicker::new(budget.start_date)
            .show(false)
            .on_cancel(|_| BudgetMessage::OpenStartDatePicker(false))
            .on_submit(BudgetMessage::StartDateSelected)
    };
    
    // End date picker
    let end_date_button = button::Button::new(
        text::Text::new(format!("End Date: {}", budget.end_date.format("%Y-%m-%d")))
    )
    .on_press(BudgetMessage::OpenEndDatePicker(true))
    .padding(10);
    
    let end_datepicker = if state.show_end_date_picker {
        date_picker::DatePicker::new(budget.end_date)
            .show(true)
            .on_cancel(|_| BudgetMessage::OpenEndDatePicker(false))
            .on_submit(BudgetMessage::EndDateSelected)
    } else {
        date_picker::DatePicker::new(budget.end_date)
            .show(false)
            .on_cancel(|_| BudgetMessage::OpenEndDatePicker(false))
            .on_submit(BudgetMessage::EndDateSelected)
    };
    
    // Buttons
    let submit_button = button::Button::new(text::Text::new("Save"))
        .on_press(BudgetMessage::Submit)
        .padding(10)
        .style(Button::Primary);
        
    let cancel_button = button::Button::new(text::Text::new("Cancel"))
        .on_press(BudgetMessage::Cancel)
        .padding(10)
        .style(Button::Secondary);
        
    let buttons_row = row::Row::new()
        .push(submit_button)
        .push(cancel_button)
        .spacing(10)
        .align_items(Alignment::Center);
        
    // Error message
    let error_text = if let Some(error) = &state.error {
        text::Text::new(error)
            .size(14)
            .style(iced::Color::from_rgb(0.8, 0.0, 0.0))
    } else {
        text::Text::new("")
    };
    
    let title = text::Text::new("Add New Transaction")
        .size(24);
    
    let transaction_types = vec![
        TransactionType::Income,
        TransactionType::Expense,
    ];
    
    let transaction_type_dropdown = pick_list::PickList::new(
        &transaction_types[..],
        Some(state.transaction_type),
        Message::TransactionTypeSelected
    )
    .placeholder("Type");
    
    let description_input = text_input::TextInput::new(
        "Description",
        &state.description,
        Message::DescriptionChanged
    )
    .padding(10);
    
    let category_input = text_input::TextInput::new(
        "Category",
        &state.category,
        Message::CategoryChanged
    )
    .padding(10);
    
    let add_button = button::Button::new(
        text::Text::new("Add Transaction")
    )
    .on_press(Message::AddTransaction)
    .padding(10);
    
    let form_row1 = row::Row::new()
        .spacing(10)
        .push(transaction_type_dropdown)
        .push(description_input);
    
    let form_row2 = row::Row::new()
        .spacing(10)
        .push(category_input)
        .push(amount_input);
    
    container::Container::new(
        Column::new()
            .push(Text::new(if is_editing { "Edit Budget" } else { "New Budget" }).size(20))
            .push(name_input)
            .push(amount_input)
            .push(
                Row::new()
                    .push(Text::new("Category:").width(Length::Fixed(100.0)))
                    .push(category_picker)
                    .align_items(Alignment::Center)
            )
            .push(
                Row::new()
                    .push(Text::new("Start Date:").width(Length::Fixed(100.0)))
                    .push(start_date_button)
                    .align_items(Alignment::Center)
            )
            .push(start_datepicker)
            .push(
                Row::new()
                    .push(Text::new("End Date:").width(Length::Fixed(100.0)))
                    .push(end_date_button)
                    .align_items(Alignment::Center)
            )
            .push(end_datepicker)
            .push(error_text)
            .push(buttons_row)
            .spacing(10)
            .padding(20)
            .width(Length::Fill)
            .style(Container::Box)
    ).into()
}
