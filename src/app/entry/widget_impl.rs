use super::Entry;

use super::super::entry::ParameterWidget;
use egui::{DragValue, TextEdit, Widget};

// Remember to add the widgets in the side_bar.rs file.
impl ParameterWidget for Entry {
    fn name_widget(&mut self) -> impl Widget {
        TextEdit::singleline(&mut self.name)
    }

    fn house_price_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.house_price)
            .range(0.0..=1_000_000_000.0)
            .speed(20_000)
            .custom_formatter(move |n, _| format!("{}M {}", n / 1_000_000.0, "Dkk"))
    }

    fn initial_payment_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.initial_payment)
            .range(0.0..=1_000_000_000.0)
            .speed(20_000)
            .custom_formatter(move |n, _| format!("{}M {}", n / 1_000_000.0, "Dkk"))
    }

    fn payment_duration_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.payment_duration)
            .range(0..=100)
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
    fn monthly_payment_widget(&mut self) -> impl Widget {
        egui::Label::new("TODO")
    }

    fn investments_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.investment)
            .range(0.0..=1_000_000.0)
            .speed(100)
            .custom_formatter(move |n, _| format!("{}K {}", n / 1_000.0, "Dkk"))
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
        self.monthly_expenses
            .iter_mut()
            .map(|e| {
                let label = egui::TextEdit::singleline(&mut e.name);

                let value = DragValue::new(&mut e.value)
                    .range(0.0..=100_000.0)
                    .speed(10)
                    .custom_formatter(move |n, _| format!("{} {}", n, "Dkk"));

                (label, value)
            })
            .collect()
    }
}
