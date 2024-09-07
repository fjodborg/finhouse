use super::prelude::*;

impl Default for Entry {
    fn default() -> Self {
        Self {
            name: "Ikke navngivet".to_owned(),
            house_price: 0,
            initial_payment: 0,
            interest: Percentage(4.0),
            payment_duration: 30,
            monthly_payment: 10000,
            investment: 0,
        }
    }
}
