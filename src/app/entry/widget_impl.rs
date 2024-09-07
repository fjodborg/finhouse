use super::Entry;

use super::super::entry::ParameterWidget;
use egui::{DragValue, TextEdit, Widget};

// Remember to add the widgets in the side_bar.rs file.
impl ParameterWidget for Entry {
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

    fn name_widget(&mut self) -> impl Widget {
        TextEdit::singleline(&mut self.name)
    }

    // Will loan duration.
    fn monthly_payment_widget(&mut self) -> impl Widget {
        egui::Label::new("TODO")
    }

    fn payment_duration_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.payment_duration)
            .range(0..=100)
            .speed(0.1)
            .suffix(" År")
    }

    fn investments_widget(&mut self) -> impl Widget {
        DragValue::new(&mut self.investment)
            .range(0.0..=1_000_000.0)
            .speed(100)
            .custom_formatter(move |n, _| format!("{}K {}", n / 1_000.0, "Dkk"))
    }
}
