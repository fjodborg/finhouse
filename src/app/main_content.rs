use super::main_app::FinhouseApp;

pub trait MainContent {
    fn create_main_content(&mut self, ui: &mut egui::Ui);
}

impl MainContent for FinhouseApp {
    fn create_main_content(&mut self, ui: &mut egui::Ui) {
        ui.heading("finhouse");

        egui_plot::Plot::new("my_plot")
            .view_aspect(2.0)
            // TODO: Allow these.
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .legend(
                egui_plot::Legend::default()
                    .background_alpha(0.75)
                    .position(egui_plot::Corner::RightTop)
                    .text_style(egui::TextStyle::Body),
            )
            .show(ui, |plot_ui| {
                for (i, entry) in self.entries.iter().enumerate() {
                    // TODO: Add slider for max years.
                    let line = entry.data_points(50);
                    plot_ui.line(line.name(format!("{} {}", i, entry.name)));
                }
            });
    }
}
