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
        DragValue::new(&mut self.loan.house_price)
            .range(0.0..=1_000_000_000.0)
            .speed(20_000)
            .custom_formatter(move |n, _| format!("{}M {}", n / default_multi, "Dkk"))
            .custom_parser(move |s| {
                let (amount, multiplier, _) = string_parse_helper(s).ok()?;
                Some(amount * multiplier.unwrap_or(default_multi))
            })
    }

    fn income_widget(&mut self) -> impl Widget {
        // TODO: Make more generic custom parser. for easier reuse.
        let default_multi = 1.0;
        DragValue::new(&mut self.income)
            .range(0.0..=1_000_000.0)
            .speed(100)
            .custom_formatter(move |n, _| format!("{} {}", n / default_multi, "Dkk"))
            .custom_parser(move |s| {
                let (amount, multiplier, _) = string_parse_helper(s).ok()?;
                Some(amount * multiplier.unwrap_or(default_multi))
            })
    }

    fn initial_payment_widget(&mut self) -> impl Widget {
        // TODO: Make more generic custom parser. for easier reuse.
        let default_multi = &1e6;
        DragValue::new(&mut self.loan.initial_payment)
            .range(0.0..=self.loan.house_price)
            .speed(20_000)
            .custom_formatter(move |n, _| format!("{}M {}", n / default_multi, "Dkk"))
            .custom_parser(|s| {
                let (amount, multiplier, _) = string_parse_helper(s).ok()?;
                Some(amount * multiplier.unwrap_or(*default_multi))
            })
    }

    fn available_amount_widget(&mut self) -> impl Widget {
        let available_amount = self.calculate_available_amount();

        // TODO: Make a method/custom widget for this specific setup.
        let text = egui::RichText::new(format!("{:.0} {}", available_amount, "Dkk"))
            .strong()
            .underline();
        egui::Label::new(text)
    }

    fn money_paid_house_widget(&mut self, only_interest: bool) -> impl Widget {
        // TODO: Make method for this.
        let monthly_payment = self.loan.get_monthly_payment();
        let yearly_payment = 12.0 * monthly_payment;
        let money_paid_for_house = yearly_payment * self.loan.duration;

        let loan = self.loan.get_loan();
        let interest_paid = money_paid_for_house - loan;
        let interest_paid_deduced = interest_paid * self.loan.interest_deduction;

        let money = match only_interest {
            true => interest_paid - interest_paid_deduced,
            false => loan + interest_paid - interest_paid_deduced,
        };

        // TODO: Make a method/custom widget for this specific setup.
        let scale = 1e6;
        let text = egui::RichText::new(format!("{:0.2}M {}", money / scale, "Dkk"));
        egui::Label::new(text)
    }

    fn money_paid_all_widget(&mut self) -> impl Widget {
        let monthly_payment = self.loan.get_monthly_payment();
        let yearly_payment = 12.0 * monthly_payment;
        let money_paid_for_house = yearly_payment * self.loan.duration;

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

        // TODO: Make a method/custom widget for this specific setup.
        let scale = 1e6;
        let text = egui::RichText::new(format!("{:0.2}M {}", money_paid / scale, "Dkk"))
            .strong()
            .underline();
        egui::Label::new(text)
    }

    fn value_and_worth_widget(&mut self) -> impl Widget {
        let yearly_available_amount = 12.0 * self.calculate_available_amount();
        let years = *self.plot_duration.borrow() as f64;
        let total_amount = years * yearly_available_amount;
        let house_price = self.loan.house_price;

        let investment = self.investment as f64;
        let stock_gain = investment + {
            let interest: f64 = self.investment_gain.into();
            let tax: f64 = self.investment_tax.into();
            let gain = investment * (1.0 + interest).powf(years);
            let delta = gain - investment;
            // TODO: Maybe reconsider this? A match or if seems more readable.
            (delta > 0.0).then(|| delta * tax).unwrap_or(delta)
        };

        let value_networth = total_amount + house_price + stock_gain;
        let scale = 1e6;

        let text = egui::RichText::new(format!("{:.2}M {}", value_networth / scale, "Dkk"))
            .strong()
            .underline();
        egui::Label::new(text)
    }

    fn payment_duration_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.loan.duration)
            .range(1..=100)
            .speed(0.1)
            .custom_formatter(|n, _| format!("{} År", n.round()))
            .custom_parser(|s| Some(s.parse::<f64>().ok()?.round()))
    }

    fn value_increase_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.value_increase)
            .range(-100.0..=10_000.0)
            .speed(0.05)
            .custom_formatter(move |n, _| format!("{:.2}%", n * 100.0))
            .custom_parser(|s| Some(s.parse::<f64>().ok()? / 100.0))
    }

    // TODO: avoid the .0 suffix.
    fn interest_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.loan.interest)
            .range(0.0001..=1.0)
            .speed(0.0005)
            .custom_formatter(move |n, _| format!("{:.2}%", n * 100.0))
            .custom_parser(|s| Some(s.parse::<f64>().ok()? / 100.0))
    }

    // TODO: avoid the .0 suffix.
    fn interest_deduction_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.loan.interest_deduction)
            .range(0.0..=1.0)
            .speed(0.0005)
            .custom_formatter(move |n, _| format!("{:.2}%", n * 100.0))
            .custom_parser(|s| Some(s.parse::<f64>().ok()? / 100.0))
    }

    // Will affect loan duration.
    fn monthly_payment_widget(&mut self, after_tax: bool) -> impl Widget {
        use log::warn;
        let loan = self.loan.get_loan();

        if loan < 0.0 {
            warn!("Impossible scenario occurred. This should have been handled in the initial_payment section. Doing workaround");
            self.loan.initial_payment = self.loan.house_price;
        }
        let mut monthly_payment = self.loan.get_monthly_payment();

        // Scale with deduction if present.
        after_tax.then(|| {
            monthly_payment *= 1.0 - self.loan.interest_deduction;
        });

        // Add all expenses to it.
        // monthly_payment += self
        //     .monthly_expenses
        //     .iter()
        //     .fold(0.0, |acc, x| acc + x.value as f64);

        // TODO: Make more generic custom parser. for easier reuse.
        let default_multi = &1.0;

        // Create copies that gets moved into the parser.
        let interest = self.loan.interest;
        let interest_deduction = self.loan.interest_deduction;

        // Okay so this is a bit weird. Since it represents years but as money payed per month.
        // Therefore speed is negative. The deduction is divided etc.
        // TODO: Clean this mess up. It needs to be rethought.
        DragValue::new(&mut self.loan.duration)
            .range(1.0..=1_000_000.0)
            .speed(-0.2) // Reverse direction.
            .custom_formatter(move |_, _| format!("{} {}", monthly_payment.round(), "Dkk"))
            .custom_parser(move |s| {
                // Extract value from string.
                let (mut payment, multiplier, _) = string_parse_helper(s).ok()?;
                payment *= multiplier.unwrap_or(*default_multi);

                // Scale the value according to if it is after or before tax deduction.
                after_tax.then(|| {
                    let interest_deduction: f64 = interest_deduction.into();
                    payment /= 1.0 - interest_deduction;
                });

                let yearly_payment = payment * 12.0;

                let years = super::utility::calculate_years(loan, yearly_payment, interest);
                Some(years)
            })
    }

    fn investments_widget(&mut self) -> impl Widget {
        // TODO: Make more generic custom parser. for easier reuse.
        let default_multi = 1e3;
        DragValue::new(&mut self.investment)
            .range(0.0..=100_000_000.0)
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
