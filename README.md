# Budget Tracker

A modern, cross-platform budget tracking application built with Rust and Iced GUI framework. Track your income, expenses, budgets, and visualize your financial data with interactive charts.

## Features

- **Transaction Management**: Add, view, and delete income and expense transactions
- **Category System**: Organize transactions with customizable categories
- **Budget Tracking**: Create and monitor budgets with progress visualization
- **Dashboard**: Overview of your financial status with summary statistics
- **Interactive Charts**: 
  - Expense breakdown by category (pie chart)
  - Income vs expenses trend (bar chart)
  - Budget progress visualization
- **SQLite Database**: Local data storage with persistent state
- **Cross-Platform**: Runs on Windows, macOS, and Linux

## Screenshots

*(Add screenshots of your application here)*

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd BudgetTracker
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

### Dependencies

This project uses the following major dependencies:

- **[Iced](https://github.com/iced-rs/iced)** - Cross-platform GUI framework
- **[Chrono](https://github.com/chronotope/chrono)** - Date and time handling
- **[SQLite](https://www.sqlite.org/)** - Local database storage
- **[Plotters](https://github.com/plotters-rs/plotters)** - Data visualization and charting
- **[UUID](https://github.com/uuid-rs/uuid)** - Unique identifier generation
- **[Serde](https://github.com/serde-rs/serde)** - Serialization framework

## Usage

### Getting Started

1. Launch the application
2. The dashboard will display your financial overview
3. Navigate between different sections using the navigation menu

### Adding Transactions

1. Go to the Transactions page
2. Click "Add Transaction"
3. Fill in the required fields:
   - Description
   - Amount
   - Type (Income/Expense)
   - Category
   - Date
4. Click "Save" to add the transaction

### Creating Budgets

1. Navigate to the Budgets page
2. Click "Create Budget"
3. Set up your budget:
   - Budget name
   - Category
   - Amount limit
   - Time period (start and end dates)
4. Monitor progress on the dashboard

### Categories

The application comes with pre-defined categories:
- Housing
- Food
- Transportation
- Entertainment
- Healthcare
- Utilities
- Shopping
- Personal
- Income
- Other

## Project Structure

```
d:\BudgetTracker\
├── src/
│   ├── app.rs              # Main application logic
│   ├── main.rs             # Application entry point
│   ├── models/             # Data models
│   │   ├── mod.rs
│   │   ├── budget.rs       # Budget model
│   │   ├── category.rs     # Category model
│   │   └── transaction.rs  # Transaction model
│   ├── ui/                 # User interface components
│   │   ├── mod.rs
│   │   ├── dashboard.rs    # Dashboard view
│   │   ├── budget.rs       # Budget management UI
│   │   ├── transaction.rs  # Transaction management UI
│   │   └── components/     # Reusable UI components
│   │       ├── mod.rs
│   │       ├── charts.rs   # Chart components
│   │       ├── datepicker.rs
│   │       ├── nav.rs
│   │       ├── transaction_form.rs
│   │       └── budget_form.rs
│   └── db/                 # Database operations
│       ├── mod.rs
│       ├── operations.rs   # CRUD operations
│       └── schema.rs       # Database schema
├── Cargo.toml              # Project dependencies
├── Cargo.lock              # Locked dependency versions
├── .gitignore              # Git ignore rules
└── README.md               # This file
```

## Database

The application uses SQLite for local data storage. The database file is automatically created at:

- **Windows**: `%APPDATA%/budgettracker/BudgetTracker/data/budget.db`
- **macOS**: `~/Library/Application Support/com.budgettracker.BudgetTracker/budget.db`
- **Linux**: `~/.local/share/budgettracker/BudgetTracker/budget.db`

### Database Schema

The application uses three main tables:

- **transactions**: Stores all income and expense records
- **categories**: Defines transaction categories with colors and icons
- **budgets**: Stores budget definitions and limits

## Development

### Running in Development Mode

```bash
cargo run
```

### Building for Release

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Export data to CSV/PDF
- [ ] Import transactions from bank statements
- [ ] Recurring transactions
- [ ] Multiple account support
- [ ] Dark theme
- [ ] Mobile responsive design
- [ ] Cloud synchronization
- [ ] Advanced reporting features
- [ ] Investment tracking
- [ ] Goal setting and tracking

## Support

If you encounter any issues or have questions, please open an issue on the GitHub repository.

## Acknowledgments

- Thanks to the Iced community for the excellent GUI framework
- SQLite for reliable local data storage
- Plotters for beautiful chart rendering
- All contributors who help improve this project
