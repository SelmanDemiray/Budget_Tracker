use iced::{widget::{button, row, text}, Element, Length};
use crate::ui::Page;

pub struct Nav {
    active_page: Page,
}

impl Nav {
    pub fn new() -> Self {
        Self {
            active_page: Page::Dashboard,
        }
    }

    pub fn view(&self) -> Element<Page> {
        let dashboard_button = button(text("Dashboard"))
            .on_press(Page::Dashboard)
            .padding(10);

        let transactions_button = button(text("Transactions"))
            .on_press(Page::Transactions)
            .padding(10);

        let budgets_button = button(text("Budgets"))
            .on_press(Page::Budgets)
            .padding(10);

        let reports_button = button(text("Reports"))
            .on_press(Page::Reports)
            .padding(10);

        row![
            dashboard_button,
            transactions_button,
            budgets_button,
            reports_button
        ]
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
    }
}
