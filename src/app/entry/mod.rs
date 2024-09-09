mod entry_impl;
mod widget_impl;
// TODO: Percentage doesn't belong here.
mod utility;

use egui::Widget;

pub use super::entry::utility::{MultiLines, Percentage};
// Prelude to make it easier to split into multiple files but having access like it was one file.
mod prelude {
    pub use super::Entry;
    pub use super::Percentage;
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Entry {
    // #[serde(skip)]
    pub name: String,
    house_price: i64,
    initial_payment: i64,
    income: i64,
    interest: Percentage,
    interest_deduction: Percentage,
    value_increase: Percentage,

    payment_duration: u32,

    investment: u64,
    investment_gain: Percentage,
    investment_tax: Percentage,

    pub monthly_expenses: Vec<MultiLines>,
}

pub trait ParameterWidget {
    fn name_widget(&mut self) -> impl Widget;
    fn house_price_widget(&mut self) -> impl Widget;
    fn initial_payment_widget(&mut self) -> impl Widget;
    fn available_amount_widget(&mut self) -> impl Widget;
    fn payment_duration_widget(&mut self) -> impl Widget;
    fn income_widget(&mut self) -> impl Widget;
    fn value_increase_widget(&mut self) -> impl Widget;

    fn interest_widget(&mut self) -> impl Widget;
    fn interest_deduction_widget(&mut self) -> impl Widget;
    fn monthly_payment_widget(&mut self, after_tax: bool) -> impl Widget;

    fn investments_widget(&mut self) -> impl Widget;
    fn investments_gain_widget(&mut self) -> impl Widget;
    fn investments_tax_widget(&mut self) -> impl Widget;

    fn monthly_expenses_widget(&mut self) -> Vec<(impl Widget, impl Widget)>;
}
