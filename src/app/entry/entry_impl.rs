use super::prelude::*;

impl Entry {
    fn calculate_new_loan(&self, loan: f64) -> f64 {
        let interest: f64 = self.interest.into();
        let interest_deduction: f64 = self.interest_deduction.into();

        let d_loan = loan * interest;
        let tax_deduction = d_loan * interest_deduction;

        loan + d_loan - tax_deduction
    }

    pub fn data_points(&self, years: u32) -> egui_plot::Line {
        let range = 0..=years;
        let series = range
            .scan(self.house_price as f64, |remaining_loan, i| {
                let loan = self.calculate_new_loan(*remaining_loan);
                *remaining_loan = loan;
                Some([i as f64, loan])
            })
            .collect::<egui_plot::PlotPoints>();
        egui_plot::Line::new(series)
    }
}

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
