use super::Entry;

use super::super::entry::ParameterWidget;
use egui::{DragValue, TextEdit, Widget};

// Remember to add the widgets in the side_bar.rs file.
impl ParameterWidget for Entry {
    fn name_widget(&mut self) -> impl Widget {
        TextEdit::singleline(&mut self.name)
    }

    fn house_price_widget(&mut self) -> impl Widget {
        // TODO: Make more generic custom parser. for easier reuse.
        let default_multi = 1e6;
        DragValue::new(&mut self.house_price)
            .range(0.0..=1_000_000_000.0)
            .speed(20_000)
            .custom_formatter(move |n, _| format!("{}M {}", n / default_multi, "Dkk"))
            .custom_parser(move |s| {
                let (amount, multiplier, _) = string_parse_helper(s).ok()?;
                Some(amount * multiplier.unwrap_or(default_multi))
            })
    }

    fn initial_payment_widget(&mut self) -> impl Widget {
        // TODO: Make more generic custom parser. for easier reuse.
        let default_multi = &1e6;
        let house_price = self.house_price as f64;
        DragValue::new(&mut self.initial_payment)
            .range(0.0..=house_price)
            .speed(20_000)
            .custom_formatter(move |n, _| format!("{}M {}", n / default_multi, "Dkk"))
            .custom_parser(|s| {
                let (amount, multiplier, _) = string_parse_helper(s).ok()?;
                Some(amount * multiplier.unwrap_or(*default_multi))
            })
    }

    fn payment_duration_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.payment_duration)
            .range(1..=100)
            .speed(0.1)
            .suffix(" År")
    }

    fn value_increase_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.value_increase.0)
            .range(-100.0..=10_000.0)
            .speed(0.05)
            .suffix("%")
    }

    // TODO: avoid the .0 suffix.
    fn interest_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.interest.0)
            .range(0.0..=100.0)
            .speed(0.05)
            .suffix("%")
    }

    // TODO: avoid the .0 suffix.
    fn interest_deduction_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.interest_deduction.0)
            .range(0.0..=100.0)
            .speed(0.05)
            .suffix("%")
    }

    // Will affect loan duration.
    fn monthly_payment_widget(&mut self, after_tax: bool) -> impl Widget {
        use log::warn;
        let years = (self.payment_duration) as f64;
        let loan = (self.house_price - self.initial_payment) as f64;

        if loan < 0.0 {
            warn!("Impossible scenario occurred. This should have been handled in the initial_payment section. Doing workaround");
            self.initial_payment = self.house_price;
        }
        let mut monthly_payment = Entry::calculate_monthly_payment(loan, years, self.interest);

        // Scale with deduction if present.
        after_tax.then(|| {
            let interest_deduction: f64 = self.interest_deduction.into();
            monthly_payment *= 1.0 - interest_deduction;
        });

        // Add all expenses to it.
        monthly_payment += self
            .monthly_expenses
            .iter()
            .fold(0.0, |acc, x| acc + x.value as f64);

        // TODO: Make more generic custom parser. for easier reuse.
        let default_multi = &1.0;

        // Create copies that gets moved into the parser.
        let interest = self.interest;
        let interest_deduction = self.interest_deduction;

        // Okay so this is a bit weird. Since it represents years but as money payed per month.
        // Therefore speed is negative. The deduction is divided etc.
        // TODO: Clean this mess up. It needs to be rethought.
        DragValue::new(&mut self.payment_duration)
            .range(1.0..=1_000_000.0)
            .speed(-0.2) // Reverse direction.
            .custom_formatter(move |_, _| format!("{} {}", monthly_payment.round(), "Dkk"))
            .custom_parser(move |s| {
                let loan = loan;

                // Extract value from string.
                let (amount, multiplier, _) = string_parse_helper(s).ok()?;
                let mut amount = amount * multiplier.unwrap_or(*default_multi);

                // Scale the value according to if it is after or before tax deduction.
                after_tax.then(|| {
                    let interest_deduction: f64 = interest_deduction.into();
                    amount /= 1.0 - interest_deduction;
                });

                let years = Entry::calculate_payment_duration(loan, amount, interest);
                Some(years)
            })
    }

    fn investments_widget(&mut self) -> impl Widget {
        // TODO: Make more generic custom parser. for easier reuse.
        let default_multi = 1e3;
        DragValue::new(&mut self.investment)
            .range(0.0..=1_000_000.0)
            .speed(100)
            .custom_formatter(move |n, _| format!("{}K {}", n / 1_000.0, "Dkk"))
            .custom_parser(move |s| {
                let (amount, multiplier, _) = string_parse_helper(s).ok()?;
                Some(amount * multiplier.unwrap_or(default_multi))
            })
    }

    // TODO: avoid the .0 suffix.
    fn investments_gain_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.investment_gain.0)
            .range(-100.0..=10_000.0)
            .speed(0.02)
            .suffix("%")
    }

    // TODO: avoid the .0 suffix.
    fn investments_tax_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.investment_tax.0)
            .range(0.0..=100.0)
            .speed(0.02)
            .suffix("%")
    }

    fn monthly_expenses_widget(&mut self) -> Vec<(impl Widget, impl Widget)> {
        let default_multi = 1.0;
        self.monthly_expenses
            .iter_mut()
            .map(|e| {
                let label = egui::TextEdit::singleline(&mut e.name);

                let value = DragValue::new(&mut e.value)
                    .range(0.0..=100_000.0)
                    .speed(10)
                    .custom_formatter(move |n, _| format!("{} {}", n, "Dkk"))
                    .custom_parser(move |s| {
                        let (amount, multiplier, _) = string_parse_helper(s).ok()?;
                        Some(amount * multiplier.unwrap_or(default_multi))
                    });

                (label, value)
            })
            .collect()
    }
}

pub fn string_parse_helper(input: &str) -> Result<(f64, Option<f64>, Option<String>), String> {
    use std::str::FromStr;

    let input = input.trim();

    // Find the first non-numerical character.
    let num_end = input
        .find(|c: char| !c.is_digit(10) && c != '.')
        .unwrap_or(input.len());

    // Parse the numeric part.
    let amount: f64 = f64::from_str(&input[..num_end])
        .map_err(|_| "Failed to parse numeric value".to_string())?;

    // Check for multiplier
    let multiplier = input.to_uppercase()[num_end..].chars().next();
    let multiplier = match multiplier {
        Some('M') => Some(1e6),
        Some('K') => Some(1e3),
        _ => None,
    };

    // Check for currency
    let currency = if input.to_uppercase().ends_with("DKK") {
        Some("DKK".to_string())
    } else {
        None
    };

    Ok((amount, multiplier, currency))
}

// TODO: Implement unit tests for string_parse_helper.
