use crate::models::{Budget, Category, Transaction, TransactionType};
use crate::ui::UiState;
use chrono::{Datelike, NaiveDate};

use iced::{Alignment, Element, Length};
use iced::widget::{column, container, row, scrollable, text, rule};
use crate::ui::components::charts;
use iced::{Element, Length, Padding};
use iced::widget::{button};

use crate::app::{BudgetTracker, Message, TransactionType};

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    Refresh,
    Error(String),
    ViewTransactions,
    ViewBudgets,
}

pub struct Dashboard<'a> {
    transactions: &'a [Transaction],
    categories: &'a [Category],
    budgets: &'a [Budget],
}

impl<'a> Dashboard<'a> {
    pub fn new(
        transactions: &'a [Transaction],
        categories: &'a [Category],
        budgets: &'a [Budget],
    ) -> Self {
        Self { 
            transactions,
            categories,
            budgets,
        }
    }
    
    pub fn view(&self) -> Element<DashboardMessage> {
        let title = Text::new("Dashboard")
            .size(28)
            .style(Text::Default);
            
        let summary = self.summary_view();
        
        let charts_column = Column::new()
            .push(charts::create_expense_by_category_chart(
                self.transactions, 
                self.categories
            ).map(|_| DashboardMessage::ViewTransactions))
            .push(charts::create_income_vs_expense_chart(
                self.transactions
            ).map(|_| DashboardMessage::ViewTransactions))
            .push(charts::create_budget_progress_chart(
                self.budgets,
                self.transactions,
                self.categories
            ).map(|_| DashboardMessage::ViewBudgets))
            .spacing(20)
            .width(Length::Fill)
            .align_items(Alignment::Center);
            
        let content = Column::new()
            .push(title)
            .push(summary)
            .push(charts_column)
            .spacing(20)
            .padding(20)
            .width(Length::Fill);
            
        Scrollable::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    
    fn summary_view(&self) -> Element<DashboardMessage> {
        // Calculate total income, expenses and balance for the current month
        let now = chrono::Local::now().naive_local().date();
        let start_of_month = NaiveDate::from_ymd_opt(
            now.year(),
            now.month(),
            1
        ).unwrap();
        
        let (total_income, total_expenses) = self.transactions.iter()
            .filter(|t| t.date >= start_of_month && t.date <= now)
            .fold((0.0, 0.0), |(inc, exp), t| {
                match t.transaction_type {
                    TransactionType::Income => (inc + t.amount, exp),
                    TransactionType::Expense => (inc, exp + t.amount),
                }
            });
            
        let balance = total_income - total_expenses;
        
        let income_text = Text::new(format!("Income: ${:.2}", total_income))
            .size(20)
            .style(Text::Color(iced::Color::from_rgb(0.0, 0.5, 0.0)));
            
        let expenses_text = Text::new(format!("Expenses: ${:.2}", total_expenses))
            .size(20)
            .style(Text::Color(iced::Color::from_rgb(0.8, 0.0, 0.0)));
            
        let balance_text = Text::new(format!("Balance: ${:.2}", balance))
            .size(20)
            .style(if balance >= 0.0 {
                Text::Color(iced::Color::from_rgb(0.0, 0.5, 0.0))
            } else {
                Text::Color(iced::Color::from_rgb(0.8, 0.0, 0.0))
            });
            
        let month_name = chrono::Month::from_u32(now.month())
            .unwrap()
            .name();
            
        let month_text = Text::new(format!("Summary for {}", month_name))
            .size(22);
            
        let summary_row = Row::new()
            .push(Container::new(income_text).width(Length::Fill))
            .push(Container::new(expenses_text).width(Length::Fill))
            .push(Container::new(balance_text).width(Length::Fill))
            .spacing(20);
            
        Column::new()
            .push(month_text)
            .push(summary_row)
            .spacing(10)
            .padding(20)
            .width(Length::Fill)
            .style(Container::Box)
            .into()
    }
}

pub fn view(state: &UiState) -> Element<DashboardMessage> {
    let title = Text::new("Dashboard")
        .size(30)
        .width(Length::Fill);

    let summary = summary_section(state);
    
    let recent_transactions = recent_transactions_section(state);
    
    let budgets_overview = budget_overview_section(state);

    Column::new()
        .push(title)
        .push(Rule::horizontal(10))
        .push(summary)
        .push(Rule::horizontal(10))
        .push(
            Row::new()
                .push(recent_transactions)
                .push(budgets_overview)
                .spacing(20)
        )
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

fn summary_section(state: &UiState) -> Element<DashboardMessage> {
    // Calculate total income
    let total_income: f64 = state.transactions.iter()
        .filter(|t| t.transaction_type == TransactionType::Income)
        .map(|t| t.amount)
        .sum();
    
    // Calculate total expenses
    let total_expenses: f64 = state.transactions.iter()
        .filter(|t| t.transaction_type == TransactionType::Expense)
        .map(|t| t.amount)
        .sum();
    
    // Calculate balance
    let balance = total_income - total_expenses;
    
    let income_text = Text::new(format!("Total Income: ${:.2}", total_income))
        .size(18);
    
    let expenses_text = Text::new(format!("Total Expenses: ${:.2}", total_expenses))
        .size(18);
    
    let balance_text = Text::new(format!("Balance: ${:.2}", balance))
        .size(20);
    
    Container::new(
        Column::new()
            .push(Text::new("Financial Summary").size(22))
            .push(
                Row::new()
                    .push(income_text)
                    .push(expenses_text)
                    .spacing(20)
                    .width(Length::Fill)
            )
            .push(balance_text)
            .spacing(15)
            .padding(15)
            .width(Length::Fill)
    )
    .width(Length::Fill)
    .style(Container::Box)
    .into()
}

fn recent_transactions_section(state: &UiState) -> Element<DashboardMessage> {
    let title = Text::new("Recent Transactions").size(22);
    
    let transactions_list = state.transactions.iter()
        .take(5) // Show only the 5 most recent transactions
        .fold(
            Column::new().spacing(10),
            |column, transaction| {
                let date = transaction.date.format("%Y-%m-%d").to_string();
                let transaction_type = match transaction.transaction_type {
                    TransactionType::Income => "Income",
                    TransactionType::Expense => "Expense",
                };
                
                let category_name = state.categories.iter()
                    .find(|c| c.id == transaction.category_id)
                    .map(|c| c.name.as_str())
                    .unwrap_or("No Category");
                
                column.push(
                    Container::new(
                        Row::new()
                            .push(Text::new(date).width(Length::Fill))
                            .push(Text::new(format!("${:.2}", transaction.amount)).width(Length::Fill))
                            .push(Text::new(transaction_type).width(Length::Fill))
                            .push(Text::new(category_name).width(Length::Fill))
                    )
                    .padding(10)
                    .style(Container::Box)
                )
            }
        );
    
    Container::new(
        Column::new()
            .push(title)
            .push(transactions_list)
            .spacing(15)
            .padding(15)
            .width(Length::Fill)
    )
    .width(Length::Fill)
    .style(Container::Box)
    .into()
}

fn budget_overview_section(state: &UiState) -> Element<DashboardMessage> {
    let title = Text::new("Budget Overview").size(22);
    
    let budgets_list = state.budgets.iter()
        .take(5) // Show only 5 budgets
        .fold(
            Column::new().spacing(10),
            |column, budget| {
                let category_name = state.categories.iter()
                    .find(|c| c.id == budget.category_id)
                    .map(|c| c.name.as_str())
                    .unwrap_or("No Category");
                
                // Calculate spending for this budget
                let spending: f64 = state.transactions.iter()
                    .filter(|t| t.transaction_type == TransactionType::Expense)
                    .filter(|t| t.category_id == budget.category_id)
                    .filter(|t| t.date >= budget.start_date && t.date <= budget.end_date)
                    .map(|t| t.amount)
                    .sum();
                
                // Calculate percentage of budget used
                let percentage = if budget.amount > 0.0 {
                    (spending / budget.amount) * 100.0
                } else {
                    0.0
                };
                
                column.push(
                    Container::new(
                        Column::new()
                            .push(
                                Row::new()
                                    .push(Text::new(format!("{} ({})", budget.name, category_name)))
                                    .push(Text::new(format!("${:.2} / ${:.2} ({:.1}%)", 
                                        spending, budget.amount, percentage)))
                                    .spacing(10)
                            )
                            // Here you might add a progress bar
                            .spacing(5)
                    )
                    .padding(10)
                    .style(Container::Box)
                )
            }
        );
    
    Container::new(
        Column::new()
            .push(title)
            .push(budgets_list)
            .spacing(15)
            .padding(15)
            .width(Length::Fill)
    )
    .width(Length::Fill)
    .style(Container::Box)
    .into()
}

pub fn view<'a>(budget_tracker: &BudgetTracker) -> Element<'a, Message> {
    let title = Text::new("Budget Dashboard")
        .size(30);

    let summary_title = Text::new("Financial Summary")
        .size(24);
        
    let total_income = budget_tracker.calculate_total_income();
    let total_expenses = budget_tracker.calculate_total_expenses();
    let balance = total_income - total_expenses;
    
    let summary_row = Row::new()
        .spacing(20)
        .push(
            Container::new(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Income").size(18))
                    .push(Text::new(format!("${:.2}", total_income)).size(24))
            )
            .padding(Padding::new(10))
            .width(Length::FillPortion(1))
        )
        .push(
            Container::new(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Expenses").size(18))
                    .push(Text::new(format!("${:.2}", total_expenses)).size(24))
            )
            .padding(Padding::new(10))
            .width(Length::FillPortion(1))
        )
        .push(
            Container::new(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Balance").size(18))
                    .push(Text::new(format!("${:.2}", balance))
                        .size(24)
                        .style(if balance >= 0.0 { 
                            iced::Color::from_rgb(0.0, 0.6, 0.0) 
                        } else { 
                            iced::Color::from_rgb(0.8, 0.0, 0.0) 
                        }))
            )
            .padding(Padding::new(10))
            .width(Length::FillPortion(1))
        );

    // Chart placeholder - in a real app, this would use plotters-iced
    let chart_placeholder = Container::new(
        Text::new("Expense Breakdown Chart (Placeholder)")
    )
    .width(Length::Fill)
    .height(Length::Units(200))
    .center_x()
    .center_y();

    let recent_transactions_title = Text::new("Recent Transactions")
        .size(24);
    
    // Build recent transactions list
    let mut recent_transactions = Column::new()
        .spacing(10);
    
    // Add headers
    let header_row = Row::new()
        .spacing(20)
        .push(Text::new("Date").width(Length::FillPortion(2)))
        .push(Text::new("Description").width(Length::FillPortion(3)))
        .push(Text::new("Category").width(Length::FillPortion(2)))
        .push(Text::new("Amount").width(Length::FillPortion(1)));
    
    recent_transactions = recent_transactions.push(header_row);
    
    // Add up to 5 recent transactions
    for transaction in budget_tracker.transactions.iter().take(5) {
        let amount_text = format!("${:.2}", transaction.amount);
        
        let row = Row::new()
            .spacing(20)
            .push(Text::new(&transaction.date.to_string()).width(Length::FillPortion(2)))
            .push(Text::new(&transaction.description).width(Length::FillPortion(3)))
            .push(Text::new(&transaction.category).width(Length::FillPortion(2)))
            .push(
                Text::new(&amount_text)
                    .width(Length::FillPortion(1))
                    .style(match transaction.transaction_type {
                        TransactionType::Income => iced::Color::from_rgb(0.0, 0.6, 0.0),
                        TransactionType::Expense => iced::Color::from_rgb(0.8, 0.0, 0.0),
                    })
            );
            
        recent_transactions = recent_transactions.push(row);
    }
    
    // View all transactions button
    let view_all_button = Button::new(
        Text::new("View All Transactions")
    )
    .on_press(Message::ViewAllTransactions)
    .padding(10);
    
    Column::new()
        .spacing(20)
        .push(title)
        .push(summary_title)
        .push(summary_row)
        .push(chart_placeholder)
        .push(recent_transactions_title)
        .push(recent_transactions)
        .push(view_all_button)
        .padding(20)
        .into()
}
