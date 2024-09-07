mod entry_impl;
// TODO: Percentage doesn't belong here.
mod utility;

// Prelude to make it easier to split into multiple files but having access like it was one file.
mod prelude {
    pub use super::Entry;
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Entry {
    // #[serde(skip)]
    pub name: String,
    pub house_price: u64,
    pub initial_payment: u64,
    pub payment_duration: u32,
    pub investment: u64,
    pub monthly_payment: u64,
}
