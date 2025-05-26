use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub id: String,
    pub name: String,
    pub category_id: Option<String>,
    pub amount: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Budget {
    pub fn new(
        name: String,
        category_id: Option<String>,
        amount: f64,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            category_id,
            amount,
            start_date,
            end_date,
        }
    }
}
