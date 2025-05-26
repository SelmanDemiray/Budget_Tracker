use chrono::NaiveDate;
use iced::Element;
use iced_aw::date_picker;

pub fn create_date_picker<'a, Message, F, G>(
    is_open: bool,
    selected_date: NaiveDate,
    on_date_selected: F,
    on_close: G,
) -> Element<'a, Message>  // Added lifetime parameter 'a
where
    F: Fn(NaiveDate) -> Message + 'static,
    G: Fn() -> Message + 'static,
{
    date_picker::DatePicker::new(selected_date)
        .show(is_open)
        .on_cancel(on_close)
        .on_submit(on_date_selected)
        .into()
}
