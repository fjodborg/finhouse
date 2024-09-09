// TODO: Proper type checking later (Place holder for now).
#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Percentage(pub f64);

impl Into<f64> for Percentage {
    fn into(self) -> f64 {
        self.0 / 100.0
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MultiLines {
    pub name: String,
    pub value: u64,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Loan {
    // Values are in years.
    pub house_price: f64,
    pub initial_payment: f64,
    pub interest: f64,
    pub interest_deduction: f64,
    pub duration: f64,
    // payment: f64,
}

impl Loan {
    /// Create a Loan from duration.
    ///
    /// * `years` - Duration in years for the loan to be paid off.
    /// * `house_price` - Price of the house.
    /// * `initial_payment` - Paid amount of hoise price.
    /// * `interest` - Yearly interest of the loan (0.0-1.0) not %.
    /// * `interest_deduction` - Yearly tax deduction from interest of the loan (0.0-1.0) not %.
    pub fn new(
        years: f64,
        house_price: f64,
        initial_payment: f64,
        interest: f64,
        interest_deduction: f64,
    ) -> Self {
        let loan = Self {
            house_price,
            initial_payment,
            interest,
            interest_deduction,
            duration: years,
            // payment: 0.0,
        };
        // loan.payment = loan.calculate_payment();
        loan
    }

    pub fn get_loan(&self) -> f64 {
        self.house_price - self.initial_payment
    }

    pub fn get_years(&self, monthly_payment: f64) -> f64 {
        // log(P/(P - L I))/log(1 + I)
        let payment = monthly_payment * 12.0;
        calculate_years(self.get_loan(), payment, self.interest)
    }

    /// Calculates how much needs to be paid to pay off the debt before the specified years.
    pub fn get_yearly_payment(&self) -> f64 {
        // (L I (1 + I)^Y)/(-1 + (1 + I)^Y)
        calculate_yearly_payment(self.get_loan(), self.duration, self.interest)
    }

    pub fn get_monthly_payment(&self) -> f64 {
        self.get_yearly_payment() / 12.0
    }
}

// We need these outside Loan to avoid compile errors when using them inside a closure that already borrows a mut of loan.

pub fn calculate_yearly_payment(loan: f64, years: f64, yearly_interest: f64) -> f64 {
    // (L I (1 + I)^Y)/(-1 + (1 + I)^Y)
    let years = years.round();
    let accum_interest = (1.0 + yearly_interest).powf(years);
    let yearly_payment = (loan * yearly_interest * accum_interest) / (accum_interest - 1.0);
    yearly_payment
}

pub fn calculate_years(loan: f64, yearly_payment: f64, yearly_interest: f64) -> f64 {
    // log(P/(P - L I))/log(1 + I)
    ((yearly_payment / (yearly_payment - loan * yearly_interest)).ln())
        / ((1.0 + yearly_interest).ln())
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
            let interest = Percentage(percentage).into();
            let monthly_payment =
                calculate_yearly_payment(loan as f64, years as f64, interest) / 12.0;
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
            let interest = Percentage(percentage).into();
            let duration = calculate_years(loan as f64, 12.0 * monthly_payment as f64, interest);
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
            let interest = Percentage(4.0).into();
            let loan = 3_000_000 as f64;
            let years = 30 as f64;
            let monthly_payment = calculate_yearly_payment(loan, years, interest) / 12.0;
            let new_years = calculate_years(loan, 12.0 * monthly_payment, interest);

            assert_float_relative_eq!(years, new_years, 0.000000001);
            assert_float_relative_eq!(14457.524783415327, monthly_payment, CYCLE_EPI);
        }
        #[test]
        fn cyclic_test_2() {
            let interest = Percentage(4.0).into();
            let loan = 2_000_000 as f64;
            let monthly_payment = 30000 as f64;
            let years = calculate_years(loan, 12.0 * monthly_payment, interest);
            let new_monthly_payment = calculate_yearly_payment(loan, years, interest) / 12.0;

            assert_float_relative_eq!(new_monthly_payment, monthly_payment, 0.000000001);
            assert_float_relative_eq!(6.407696547962448, years, CYCLE_EPI);
        }
    }
}
