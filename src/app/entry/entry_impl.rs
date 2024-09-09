use super::prelude::*;
use super::Loan;

impl Default for Entry {
    fn default() -> Self {
        let years = 30.0;
        let interest = Percentage(4.6);
        let interest_deduction = Percentage(20.6);
        let house_price = 0.0;
        let initial_payment = 0.0;
        Self {
            // House.
            name: "Ikke navngivet".to_owned(),
            loan: Loan::new(
                years,
                house_price,
                initial_payment,
                interest.into(),
                interest_deduction.into(),
            ),
            income: 30000,
            value_increase: Percentage(2.0).into(),

            // Interest.

            // Investment.
            investment: 0,
            investment_gain: Percentage(10.0),
            investment_tax: Percentage(42.0),

            // Expenses.
            monthly_expenses: vec![],
        }
    }
}

impl Entry {
    // TODO: Find a better place for this function.
    fn calculate_new_loan(&self, loan: f64, yearly_payment: f64) -> f64 {
        let interest = self.loan.interest;
        let d_loan = loan * interest;

        loan + d_loan - yearly_payment
    }

    fn calculate_stock_gains(&self, years: f64) -> f64 {
        let gains: f64 = self.investment_gain.into();
        let investment = self.investment as f64;
        investment * (1.0 + gains).powf(years)
    }

    pub fn data_points(&self, years: u32) -> egui_plot::Line {
        let range = 0..=years;

        let loan = self.loan.get_loan();
        let yearly_payment = 12.0 * self.loan.get_monthly_payment();

        // Loan and payment is negative money.
        let loan = -loan;
        let yearly_payment = -yearly_payment;

        // TODO: Needs to handle monthly payment after house is paid off somehow.
        // Maybe the solution is to just not check if loan is < 0.0, since a persons "net-worth" still increases.

        // Calculate the loan payment.
        let series: Vec<_> = range
            .scan(loan as f64, |remaining_loan, i| {
                // TODO: Find a more rustic way to do this.
                if i != 0 && *remaining_loan < -1.0 {
                    *remaining_loan = self.calculate_new_loan(*remaining_loan, yearly_payment);
                }

                Some([i as f64, *remaining_loan])
            })
            .collect();

        // Apply the stock gains.
        let series = series
            .into_iter()
            .map(|[i, loan]| {
                let value = self.calculate_stock_gains(i);
                [i, loan + value]
            })
            .collect();

        let final_series: egui_plot::PlotPoints = series;
        egui_plot::Line::new(final_series)
    }
}
