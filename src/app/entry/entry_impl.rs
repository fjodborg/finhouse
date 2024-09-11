use std::cell::RefCell;
use std::rc::Rc;

use super::prelude::*;
use super::Loan;

impl Entry {
    pub fn new(plot_duration: Rc<RefCell<u32>>) -> Self {
        let years = 30.0;
        let interest = Percentage(4.66);
        let interest_deduction = Percentage(20.6);
        let house_price = 0.0;
        let initial_payment = 0.0;
        Self {
            plot_duration,

            // House.
            name: "Ikke navngivet".to_owned(),
            loan: Loan::new(
                years,
                house_price,
                initial_payment,
                interest.into(),
                interest_deduction.into(),
            ),
            income: 35000,
            value_increase: Percentage(2.0).into(),

            // Interest.

            // Investment.
            investment: 0,
            investment_gain: Percentage(5.0),
            investment_tax: Percentage(42.0),

            // Expenses.
            monthly_expenses: vec![],
        }
    }

    // TODO: Find a better place for this function.
    fn calculate_new_loan(&self, loan: f64, yearly_payment: f64) -> f64 {
        let interest = self.loan.interest;
        let d_loan = loan * interest;

        loan + d_loan - yearly_payment
    }

    pub fn calculate_available_amount(&self) -> f64 {
        let summed_monthly_expenses = self
            .monthly_expenses
            .iter()
            .fold(0.0, |acc, x| acc + x.value as f64);

        let monthly_payment_after_deduction =
            self.loan.get_monthly_payment() * (1.0 - self.loan.interest_deduction);

        let total_monthly_payment = summed_monthly_expenses + monthly_payment_after_deduction;
        let available_amount = self.income as f64 - total_monthly_payment;
        available_amount
    }

    fn calculate_stock_gains(&self, years: f64) -> f64 {
        let gains: f64 = self.investment_gain.into();
        let investment = self.investment as f64;
        investment * (1.0 + gains).powf(years)
    }

    pub fn data_points(&self, years: u32, scale: f64) -> egui_plot::Line {
        let range = 0..=years;

        let loan = self.loan.get_loan();
        let yearly_payment = 12.0 * self.loan.get_monthly_payment();

        // TODO: Needs to handle monthly payment after house is paid off somehow.
        // Maybe the solution is to just not check if loan is < 0.0, since a persons "net-worth" still increases.

        // Calculate the loan payment.
        let series: Vec<_> = range
            .scan(loan as f64, |remaining_loan, i| {
                // TODO: Find a more rustic way to do this.
                if i != 0 && *remaining_loan > 1.0 {
                    *remaining_loan = self.calculate_new_loan(*remaining_loan, yearly_payment);
                }

                Some([i as f64, *remaining_loan])
            })
            .map(|[x, y]| [x, y / scale])
            .collect();

        // Apply the stock gains.
        let series = series
            .into_iter()
            .map(|[i, loan]| {
                let value = 0.0;
                [i, loan + value]
            })
            .collect();

        let final_series: egui_plot::PlotPoints = series;
        egui_plot::Line::new(final_series)
    }
}
