use chrono::{DateTime, Local};
use iced::widget::{button, column, container, row, text, text_input, scrollable};
use iced::{Element, Length, Task};
use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Message {
    AddTransaction,
    UpdateDescription(String),
    UpdateAmount(String),
    UpdateCategory(String),
    TransactionAdded(Result<(), String>),
    LoadTransactions,
    TransactionsLoaded(Result<Vec<Transaction>, String>),
    DeleteTransaction(Uuid),
    TransactionDeleted(Result<(), String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub description: String,
    pub amount: f64,
    pub category: String,
    pub date: DateTime<Local>,
}

#[derive(Debug)]
pub struct BudgetTracker {
    transactions: Vec<Transaction>,
    new_description: String,
    new_amount: String,
    new_category: String,
    database_path: PathBuf,
}

impl BudgetTracker {
    pub fn new() -> (Self, Task<Message>) {
        let database_path = get_database_path();
        
        let app = BudgetTracker {
            transactions: Vec::new(),
            new_description: String::new(),
            new_amount: String::new(),
            new_category: String::new(),
            database_path,
        };

        // Initialize database
        if let Err(e) = app.init_database() {
            eprintln!("Failed to initialize database: {}", e);
        }

        (app, Task::perform(async {}, |_| Message::LoadTransactions))
    }

    pub fn title(&self) -> String {
        String::from("Budget Tracker")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UpdateDescription(description) => {
                self.new_description = description;
                Task::none()
            }
            Message::UpdateAmount(amount) => {
                self.new_amount = amount;
                Task::none()
            }
            Message::UpdateCategory(category) => {
                self.new_category = category;
                Task::none()
            }
            Message::AddTransaction => {
                if let Ok(amount) = self.new_amount.parse::<f64>() {
                    let transaction = Transaction {
                        id: Uuid::new_v4(),
                        description: self.new_description.clone(),
                        amount,
                        category: self.new_category.clone(),
                        date: Local::now(),
                    };

                    let db_path = self.database_path.clone();
                    Task::perform(
                        async move {
                            save_transaction(&db_path, &transaction).await
                        },
                        Message::TransactionAdded,
                    )
                } else {
                    Task::none()
                }
            }
            Message::TransactionAdded(result) => {
                match result {
                    Ok(()) => {
                        self.new_description.clear();
                        self.new_amount.clear();
                        self.new_category.clear();
                        Task::perform(async {}, |_| Message::LoadTransactions)
                    }
                    Err(e) => {
                        eprintln!("Failed to add transaction: {}", e);
                        Task::none()
                    }
                }
            }
            Message::LoadTransactions => {
                let db_path = self.database_path.clone();
                Task::perform(
                    async move {
                        load_transactions(&db_path).await
                    },
                    Message::TransactionsLoaded,
                )
            }
            Message::TransactionsLoaded(result) => {
                match result {
                    Ok(transactions) => {
                        self.transactions = transactions;
                    }
                    Err(e) => {
                        eprintln!("Failed to load transactions: {}", e);
                    }
                }
                Task::none()
            }
            Message::DeleteTransaction(id) => {
                let db_path = self.database_path.clone();
                Task::perform(
                    async move {
                        delete_transaction(&db_path, id).await
                    },
                    Message::TransactionDeleted,
                )
            }
            Message::TransactionDeleted(result) => {
                match result {
                    Ok(()) => {
                        Task::perform(async {}, |_| Message::LoadTransactions)
                    }
                    Err(e) => {
                        eprintln!("Failed to delete transaction: {}", e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let total_balance: f64 = self.transactions.iter().map(|t| t.amount).sum();

        let input_section = column![
            text("Add New Transaction").size(20),
            text_input("Description", &self.new_description)
                .on_input(Message::UpdateDescription)
                .padding(10),
            text_input("Amount", &self.new_amount)
                .on_input(Message::UpdateAmount)
                .padding(10),
            text_input("Category", &self.new_category)
                .on_input(Message::UpdateCategory)
                .padding(10),
            button("Add Transaction")
                .on_press(Message::AddTransaction)
                .padding(10),
        ]
        .spacing(10)
        .padding(20);

        let balance_section = column![
            text(format!("Total Balance: ${:.2}", total_balance))
                .size(24)
        ]
        .padding(20);

        let transactions_list = scrollable(
            column(
                self.transactions
                    .iter()
                    .map(|transaction| {
                        row![
                            text(&transaction.description).width(Length::Fill),
                            text(format!("${:.2}", transaction.amount)).width(Length::Shrink),
                            text(&transaction.category).width(Length::Shrink),
                            text(transaction.date.format("%Y-%m-%d").to_string()).width(Length::Shrink),
                            button("Delete")
                                .on_press(Message::DeleteTransaction(transaction.id))
                                .padding(5),
                        ]
                        .spacing(10)
                        .padding(10)
                        .into()
                    })
                    .collect::<Vec<_>>()
            )
            .spacing(5)
        )
        .height(Length::Fill);

        container(
            column![
                input_section,
                balance_section,
                text("Transactions").size(20),
                transactions_list
            ]
            .spacing(10)
        )
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn init_database(&self) -> SqlResult<()> {
        let conn = Connection::open(&self.database_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                description TEXT NOT NULL,
                amount REAL NOT NULL,
                category TEXT NOT NULL,
                date TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }
}

fn get_database_path() -> PathBuf {
    if let Some(data_dir) = directories::ProjectDirs::from("com", "budgettracker", "BudgetTracker") {
        let data_path = data_dir.data_dir();
        std::fs::create_dir_all(data_path).unwrap_or_else(|_| {
            eprintln!("Failed to create data directory");
        });
        data_path.join("budget.db")
    } else {
        PathBuf::from("budget.db")
    }
}

async fn save_transaction(db_path: &PathBuf, transaction: &Transaction) -> Result<(), String> {
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    conn.execute(
        "INSERT INTO transactions (id, description, amount, category, date) VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            transaction.id.to_string(),
            transaction.description.clone(),
            transaction.amount.to_string(),
            transaction.category.clone(),
            transaction.date.to_rfc3339(),
        ],
    )
    .map_err(|e| format!("Failed to insert transaction: {}", e))?;
    
    Ok(())
}

async fn load_transactions(db_path: &PathBuf) -> Result<Vec<Transaction>, String> {
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    let mut stmt = conn
        .prepare("SELECT id, description, amount, category, date FROM transactions ORDER BY date DESC")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let transaction_iter = stmt
        .query_map([], |row| {
            let id_str: String = row.get(0)?;
            let date_str: String = row.get(4)?;
            
            Ok(Transaction {
                id: Uuid::parse_str(&id_str).map_err(|_| rusqlite::Error::InvalidColumnType(0, "id".to_string(), rusqlite::types::Type::Text))?,
                description: row.get(1)?,
                amount: row.get(2)?,
                category: row.get(3)?,
                date: DateTime::parse_from_rfc3339(&date_str)
                    .map_err(|_| rusqlite::Error::InvalidColumnType(4, "date".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&Local),
            })
        })
        .map_err(|e| format!("Failed to query transactions: {}", e))?;
    
    let mut transactions = Vec::new();
    for transaction in transaction_iter {
        transactions.push(transaction.map_err(|e| format!("Failed to parse transaction: {}", e))?);
    }
    
    Ok(transactions)
}

async fn delete_transaction(db_path: &PathBuf, id: Uuid) -> Result<(), String> {
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    conn.execute(
        "DELETE FROM transactions WHERE id = ?1",
        [id.to_string()],
    )
    .map_err(|e| format!("Failed to delete transaction: {}", e))?;
    
    Ok(())
}