use super::prelude::*;

impl Default for Entry {
    fn default() -> Self {
        Self {
            // House.
            name: "Ikke navngivet".to_owned(),
            house_price: 0,
            initial_payment: 0,
            payment_duration: 30,
            value_increase: Percentage(2.0),

            // Interest.
            interest: Percentage(4.0),
            interest_deduction: Percentage(0.0),
            monthly_payment: 10000,

            // Investment.
            investment: 0,
            investment_gain: Percentage(10.0),
            investment_tax: Percentage(42.0),

            // Expenses.
            monthly_expenses: vec![],
        }
    }
}
