mod app;

fn main() -> iced::Result {
    iced::application(app::BudgetTracker::title, app::BudgetTracker::update, app::BudgetTracker::view)
        .run_with(app::BudgetTracker::new)
}
