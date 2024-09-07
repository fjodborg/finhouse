use super::prelude::*;

impl Default for Entry {
    fn default() -> Self {
        Self {
            name: "Ikke navngivet".to_owned(),
            house_price: 0,
            initial_payment: 0,
            payment_duration: 30,
            value_increase: Percentage(2.0),

            interest: Percentage(4.0),
            monthly_payment: 10000,
            interest_deduction: Percentage(0.0),

            investment: 0,
            investment_gain: Percentage(10.0),
            investment_tax: Percentage(42.0),
        }
    }
}
