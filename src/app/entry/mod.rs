mod entry_impl;
mod widget_impl;
// TODO: Percentage doesn't belong here.
mod utility;

use egui::Widget;

// Prelude to make it easier to split into multiple files but having access like it was one file.
mod prelude {
    pub use super::Entry;
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Entry {
    // #[serde(skip)]
    pub name: String,
    house_price: u64,
    initial_payment: u64,
    payment_duration: u32,
    investment: u64,
    monthly_payment: u64,
}

pub trait ParameterWidget {
    fn house_price_widget(&mut self) -> impl Widget;
    fn initial_payment_widget(&mut self) -> impl Widget;
    fn name_widget(&mut self) -> impl Widget;
    fn monthly_payment_widget(&mut self) -> impl Widget;
    fn payment_duration_widget(&mut self) -> impl Widget;
    fn investments_widget(&mut self) -> impl Widget;
}
