use super::prelude::*;

impl Entry {
    /// Calculates how much needs to be paid to pay off the debt before the specified years.
    fn calculate_monthly_payment(&self, loan: f64, years: f64) -> f64 {
        // (I (1 + I)^Y L)/(-1 + (1 + I)^Y)

        let interest: f64 = self.interest.into();
        let monthly_interest: f64 = interest / 12.0;
        let num_payments = years * 12.0;

        let acc_interest = (1.0 + monthly_interest).powf(num_payments);
        let first_interest = monthly_interest * loan;
        let yearly_payments = (first_interest * acc_interest) / (acc_interest - 1.0);
        yearly_payments
    }

    fn calculate_payment_duration(&self, loan: f64, monthly_payment: f64) -> f64 {
        // log(-P/(I L - P))/log(1 + I)
        // Same as expression as above, just solved for years instead of payments.

        let interest: f64 = self.interest.into();
        let monthly_interest: f64 = interest / 12.0;
        let payment = monthly_payment;

        let first_interest = monthly_interest * loan;
        if payment - first_interest <= 0.0 {
            return f64::INFINITY;
        }

        // (-payment / (first_interest - payment)).log10() / (1.0 + interest).log10()
        (payment / (payment - first_interest)).log10() / (1.0 + monthly_interest).log10() / 12.0
    }

    fn calculate_new_loan(&self, loan: f64) -> f64 {
        // TODO: Needs rework.
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

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[allow()]
    use assert_float_eq::{
        afe_abs, afe_is_relative_eq, afe_relative_error_msg, assert_float_relative_eq,
    };

    const BANK_EPI: f64 = 0.04; // This needs to be high because even the banks seems to disagree a lot.
    const CYCLE_EPI: f64 = 0.0001;
    mod monthly_payment {
        use super::*;
        fn monthly_payment_helper(percentage: f64, loan: u64, years: u64, expected: u64) {
            let mut entry = Entry::default();
            entry.interest = Percentage(percentage);
            let monthly_payment = entry.calculate_monthly_payment(loan as f64, years as f64);
            assert_float_relative_eq!(expected as f64, monthly_payment, BANK_EPI);
        }

        // Data taken from 2 different banks.
        mod bank1 {
            use super::*;
            #[test]
            fn bank1_test1() {
                monthly_payment_helper(4.80, 3_000_000, 30, 15950);
            }

            #[test]
            fn bank1_test2() {
                monthly_payment_helper(4.80, 3_000_000, 20, 19839);
            }
            #[test]
            fn bank1_test3() {
                monthly_payment_helper(4.82, 2_000_000, 10, 20763);
            }
        }

        mod bank2 {
            use super::*;
            #[test]
            fn bank2_test1() {
                monthly_payment_helper(4.66, 3_000_000, 30, 15526);
            }

            #[test]
            fn bank2_test2() {
                monthly_payment_helper(4.66, 3_000_000, 20, 19370);
            }
            #[test]
            fn bank2_test3() {
                monthly_payment_helper(4.68, 2_000_000, 10, 20463);
            }
        }
    }
    mod payment_duration {
        use super::*;

        fn payment_duration_helper(
            percentage: f64,
            loan: u64,
            monthly_payment: u64,
            expected: u64,
        ) {
            let mut entry = Entry::default();
            entry.interest = Percentage(percentage);
            let duration = entry.calculate_payment_duration(loan as f64, monthly_payment as f64);
            assert_float_relative_eq!(expected as f64, duration, BANK_EPI);
        }

        // Data taken from 2 different banks.
        mod bank1 {
            use super::*;
            #[test]
            fn bank1_test1() {
                payment_duration_helper(4.80, 3_000_000, 15950, 30);
            }

            #[test]
            fn bank1_test2() {
                payment_duration_helper(4.80, 3_000_000, 19839, 20);
            }
            #[test]
            fn bank1_test3() {
                payment_duration_helper(4.82, 2_000_000, 20763, 10);
            }
        }

        mod bank2 {
            use super::*;
            #[test]
            fn bank2_test1() {
                payment_duration_helper(4.66, 3_000_000, 15526, 30);
            }

            #[test]
            fn bank2_test2() {
                payment_duration_helper(4.66, 3_000_000, 19370, 20);
            }
            #[test]
            fn bank2_test3() {
                payment_duration_helper(4.68, 2_000_000, 20463, 10);
            }
        }
    }

    mod cyclic_test {
        use super::*;
        #[test]
        fn cyclic_test_1() {
            let mut entry = Entry::default();
            entry.interest = Percentage(4.0);
            let loan = 3_000_000 as f64;
            let years = 30 as f64;
            let monthly_payment = entry.calculate_monthly_payment(loan, years);
            let new_years = entry.calculate_payment_duration(loan, monthly_payment);

            assert_float_relative_eq!(years, new_years, 0.000000001);
            assert_float_relative_eq!(14322.0, monthly_payment, CYCLE_EPI);
        }
        #[test]
        fn cyclic_test_2() {
            let mut entry = Entry::default();
            entry.interest = Percentage(4.0);
            let loan = 2_000_000 as f64;
            let monthly_payment = 30000 as f64;
            let years = entry.calculate_payment_duration(loan, monthly_payment);
            let new_monthly_payment = entry.calculate_monthly_payment(loan, years);

            assert_float_relative_eq!(new_monthly_payment, monthly_payment, 0.000000001);
            assert_float_relative_eq!(6.2933, years, CYCLE_EPI);
        }
    }
}
