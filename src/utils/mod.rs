use chrono::{NaiveDate, Local, Datelike};

pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

pub fn format_date(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

// Utility functions for date handling
pub fn today() -> NaiveDate {
    let local_date = Local::now().date_naive();
    NaiveDate::from_ymd_opt(local_date.year(), local_date.month(), local_date.day())
        .expect("Failed to create NaiveDate")
}

pub fn format_month_year(date: &NaiveDate) -> String {
    date.format("%b %Y").to_string()
}

pub fn get_month_start() -> NaiveDate {
    let now = Local::now().naive_local().date();
    NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap_or(now)
}

pub fn get_month_end() -> NaiveDate {
    let now = Local::now().naive_local().date();
    
    // Get the first day of the next month and subtract 1 day
    let (year, month) = if now.month() == 12 {
        (now.year() + 1, 1)
    } else {
        (now.year(), now.month() + 1)
    };
    
    let first_of_next_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(now);
    first_of_next_month.pred_opt().unwrap_or(now)
}

pub fn calculate_percentage(value: f64, total: f64) -> f64 {
    if total == 0.0 {
        0.0
    } else {
        (value / total) * 100.0
    }
}
