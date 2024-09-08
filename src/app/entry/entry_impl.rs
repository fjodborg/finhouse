use super::prelude::*;

impl Entry {
    /// Calculates how much needs to be paid to pay off the debt before the specified years.
    pub fn calculate_monthly_payment(loan: f64, years: f64, interest_per: Percentage) -> f64 {
        let interest: f64 = interest_per.into();

        // (L I (1 + I)^Y)/(-1 + (1 + I)^Y)
        let accum_interest = (1.0 + interest).powf(years);
        let yearly_payment = (loan * interest * accum_interest) / (accum_interest - 1.0);
        yearly_payment / 12.0
    }

    pub fn calculate_payment_duration(
        loan: f64,
        monthly_payment: f64,
        interest_per: Percentage,
    ) -> f64 {
        let interest: f64 = interest_per.into();
        let yearly_payment = monthly_payment * 12.0;

        // log(P/(P - L I))/log(1 + I)
        ((yearly_payment / (yearly_payment - loan * interest)).ln()) / ((1.0 + interest).ln())
    }

    fn calculate_new_loan(&self, loan: f64, yearly_payment: f64) -> f64 {
        // TODO: Needs rework.
        let interest: f64 = self.interest.into();
        // let interest_deduction: f64 = self.interest_deduction.into();

        let d_loan = loan * interest;
        // let tax_deduction = d_loan * interest_deduction;
        // let yearly_payment = (self.monthly_payment * 12) as f64;
        // println!("{} {} {} {}", loan, d_loan, yearly_payment, tax_deduction);

        loan + d_loan - yearly_payment
    }

    pub fn data_points(&self, years: u32) -> egui_plot::Line {
        let range = 0..=years;
        let loan = (self.house_price - self.initial_payment) as f64;
        let years = self.payment_duration as f64;
        let yearly_payment = 12.0 * Self::calculate_monthly_payment(loan, years, self.interest);
        let series = range
            .scan(loan as f64, |remaining_loan, i| {
                // TODO: Find a more rustic way to do this.
                if i == 0 {
                    return Some([i as f64, loan]);
                }
                let loan = self.calculate_new_loan(*remaining_loan, yearly_payment);
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
            interest: Percentage(4.6),
            interest_deduction: Percentage(20.6),

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

    const BANK_EPI: f64 = 0.05; // This needs to be high because even the banks seems to disagree a lot.
    const CYCLE_EPI: f64 = 0.0001;
    mod monthly_payment {
        use super::*;
        fn monthly_payment_helper(percentage: f64, loan: u64, years: u64, expected: u64) {
            let interest = Percentage(percentage);
            let monthly_payment =
                Entry::calculate_monthly_payment(loan as f64, years as f64, interest);
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
            let interest = Percentage(percentage);
            let duration =
                Entry::calculate_payment_duration(loan as f64, monthly_payment as f64, interest);
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
            let interest = Percentage(4.0);
            let loan = 3_000_000 as f64;
            let years = 30 as f64;
            let monthly_payment = Entry::calculate_monthly_payment(loan, years, interest);
            let new_years = Entry::calculate_payment_duration(loan, monthly_payment, interest);

            assert_float_relative_eq!(years, new_years, 0.000000001);
            assert_float_relative_eq!(14457.524783415327, monthly_payment, CYCLE_EPI);
        }
        #[test]
        fn cyclic_test_2() {
            let interest = Percentage(4.0);
            let loan = 2_000_000 as f64;
            let monthly_payment = 30000 as f64;
            let years = Entry::calculate_payment_duration(loan, monthly_payment, interest);
            let new_monthly_payment = Entry::calculate_monthly_payment(loan, years, interest);

            assert_float_relative_eq!(new_monthly_payment, monthly_payment, 0.000000001);
            assert_float_relative_eq!(6.407696547962448, years, CYCLE_EPI);
        }
    }
}
