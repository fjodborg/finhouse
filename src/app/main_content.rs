use super::FinhouseApp;

pub trait MainContent {
    fn create_main_content(&mut self, ui: &mut egui::Ui);
}

impl MainContent for FinhouseApp {
    fn create_main_content(&mut self, ui: &mut egui::Ui) {
        ui.heading("Finhouse.");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add(egui::Label::new(
                    "Antal år at basere plot m.m. på (Kan ikke gå under største låneperiode):",
                ));
                let plot_years: &mut u32 = &mut self.plot_years.borrow_mut();

                // Find the maximum year of all entries.
                let max_years = self
                    .entries
                    .iter()
                    .map(|x| x.loan.duration)
                    .max()
                    .unwrap_or(1);

                ui.add(
                    egui::DragValue::new(plot_years)
                        .suffix(" År")
                        .range(max_years..=100)
                        .speed(0.1),
                );
            });
        });

        ui.vertical_centered(|ui| {
            ui.heading("Lån over tid.");
        });
        egui_plot::Plot::new("my_plot")
            .view_aspect(2.0)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .x_axis_label("År")
            .y_axis_label("Million Dkk")
            .y_axis_min_width(40.0)
            .legend(
                egui_plot::Legend::default()
                    .background_alpha(0.75)
                    .position(egui_plot::Corner::RightTop)
                    .text_style(egui::TextStyle::Body),
            )
            .show(ui, |plot_ui| {
                let scale = 1e6;
                for (i, entry) in self.entries.iter_mut().enumerate() {
                    // TODO: Add slider for max years.
                    let line = entry.data_points(*self.plot_years.borrow(), scale);
                    plot_ui.line(line.name(format!("{} {}", i, entry.name)));
                }
            });
    }
}
