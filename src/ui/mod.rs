pub mod budget;
pub mod transaction;
pub mod dashboard;
pub mod components;

use crate::models::{Budget, Category, Transaction};
use crate::ui::dashboard::DashboardMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Budget(budget::BudgetMessage),
    Transaction(transaction::TransactionMessage),
    ChangePage(Page),
    LoadTransactions,
    TransactionsLoaded(Vec<Transaction>),
    LoadCategories,
    CategoriesLoaded(Vec<Category>),
    LoadBudgets,
    BudgetsLoaded(Vec<Budget>),
    Dashboard(DashboardMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    Transactions,
    Budgets,
    Reports,
}

pub struct UiState {
    pub page: Page,
    pub transactions: Vec<Transaction>,
    pub categories: Vec<Category>,
    pub budgets: Vec<Budget>,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            page: Page::Dashboard,
            transactions: Vec::new(),
            categories: Vec::new(),
            budgets: Vec::new(),
        }
    }
}
