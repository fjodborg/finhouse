use super::main_app;

pub trait SideBar {
    fn create_side_panel(&mut self, ui: &mut egui::Ui);
}

fn sidebar_widget(ui: &mut egui::Ui, label: &str, widget: impl egui::Widget) {
    ui.add(egui::Label::new(label));
    ui.add(widget);
    ui.end_row();
}

impl SideBar for main_app::FinhouseApp {
    fn create_side_panel(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("side_bar")
                .num_columns(2)
                .spacing([20.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    let entry = &mut self.entries[self.selected_entry];

                    sidebar_widget(ui, "Tab Navn", egui::TextEdit::singleline(&mut entry.name));
                });

            ui.separator();
            egui::warn_if_debug_build(ui);
            source_code(ui);
        });
    }
}

fn source_code(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Source code: ");
        ui.hyperlink_to("github", "https://github.com/fjodborg/finhouse");
    });
}
