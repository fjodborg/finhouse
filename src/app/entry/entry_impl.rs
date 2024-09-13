use std::cell::RefCell;
use std::rc::Rc;

use super::prelude::*;
use super::Loan;

impl Entry {
    pub fn new(plot_duration: Rc<RefCell<u32>>) -> Self {
        let years = 30;
        let interest = Percentage(4.66);
        let interest_deduction = Percentage(33.1);
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

    pub fn calculate_available_amount(&self, after_loan: bool) -> f64 {
        let summed_monthly_expenses = self
            .monthly_expenses
            .iter()
            .fold(0.0, |acc, x| acc + x.value as f64);

        let monthly_payment = match after_loan {
            true => 0.0,
            false => {
                self.loan.get_monthly_payment()
                    - self.calculate_monthly_tax_deduced_interest_amount()
            }
        };

        let total_monthly_payment = summed_monthly_expenses + monthly_payment;
        let available_amount = self.income as f64 - total_monthly_payment;
        available_amount
    }

    pub fn calculate_money_paid(&self) -> f64 {
        let monthly_payment = self.loan.get_monthly_payment();
        let yearly_payment = 12.0 * monthly_payment;
        let money_paid_for_house = yearly_payment * self.loan.duration as f64;

        let loan = self.loan.get_loan();
        let interest_paid = money_paid_for_house - loan;
        let interest_paid_deduced = interest_paid * self.loan.interest_deduction;
        let paid_money_for_home = loan + interest_paid - interest_paid_deduced;

        let summed_monthly_expenses = self
            .monthly_expenses
            .iter()
            .fold(0.0, |acc, x| acc + x.value as f64);
        let summed_yearly_expenses = 12.0 * summed_monthly_expenses;

        let plot_duration = *self.plot_duration.borrow() as f64;
        let summed_expenses = summed_yearly_expenses * plot_duration;

        let money_paid = paid_money_for_home + summed_expenses;
        money_paid
    }

    pub fn calculate_value_and_networth(&self) -> f64 {
        let plot_years = *self.plot_duration.borrow() as f64;
        let total_amount = {
            // Calculate amount for duration of loan payment.
            let yearly_available_amount = 12.0 * self.calculate_available_amount(false);
            let loan_years = self.loan.duration as f64;
            let loan_amount = loan_years * yearly_available_amount;

            // Calculate amount after loan is paid off.
            let yearly_after_loan_amount = 12.0 * self.calculate_available_amount(true);
            let after_loan_amount = (plot_years - loan_years) * yearly_after_loan_amount;

            loan_amount + after_loan_amount
        };

        let investment = self.investment as f64;

        let stock_gain = investment + {
            let interest: f64 = self.investment_gain.into();
            let tax: f64 = self.investment_tax.into();
            let gain = investment * (1.0 + interest).powf(plot_years);
            let delta = gain - investment;
            // TODO: Maybe reconsider this? A match or if seems more readable.
            (delta > 0.0).then(|| delta * tax).unwrap_or(delta)
        };

        let house_price = self.loan.house_price;

        let value_networth = total_amount + house_price + stock_gain;
        value_networth
    }

    /// This calculates the approximate monthly deduction value. However since the deduction
    /// depends on the loan and the loan is paid off slowly it can't be 100% accurate.  
    pub fn calculate_monthly_tax_deduced_interest_amount(&self) -> f64 {
        let total_interest_payment = self
            .loan
            .interest_amount_by_year
            .iter()
            .fold(0.0, |acc, x| acc + x);
        let total_interest_payment_deduced = total_interest_payment * self.loan.interest_deduction;
        let yearly_deduction = total_interest_payment_deduced / self.loan.duration as f64;
        yearly_deduction / 12.0
    }

    pub fn data_points(&mut self, years: u32, scale: f64) -> egui_plot::Line {
        let range = 0..=years;

        let loan = self.loan.get_loan();
        let yearly_payment = 12.0 * self.loan.get_monthly_payment();

        // TODO: Needs to handle monthly payment after house is paid off somehow.
        // Maybe the solution is to just not check if loan is < 0.0, since a persons "net-worth" still increases.

        self.loan.interest_amount_by_year.clear();
        self.loan
            .interest_amount_by_year
            .reserve(self.loan.duration as usize);

        // Calculate the loan payment.
        let series: Vec<_> = range
            .scan(loan as f64, |remaining_loan, i| {
                // TODO: Find a more rustic way to do this.
                if i != 0 && *remaining_loan > 1.0 {
                    *remaining_loan = self.calculate_new_loan(*remaining_loan, yearly_payment);
                }

                // TODO: This should be rethought.
                self.loan
                    .interest_amount_by_year
                    .push(*remaining_loan * self.loan.interest);

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
