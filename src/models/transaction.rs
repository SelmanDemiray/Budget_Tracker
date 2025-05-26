use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub description: String,
    pub amount: f64,
    pub date: NaiveDate,
    pub transaction_type: TransactionType,
    pub category_id: Option<Uuid>,
}

impl Transaction {
    pub fn new(description: String, amount: f64, date: NaiveDate, 
               transaction_type: TransactionType, category_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            amount,
            date,
            transaction_type,
            category_id,
        }
    }
}
