// TODO:
#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::prelude::*;

impl eframe::App for FinhouseApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Time between automatic calls to [`Self::save`]
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(10)
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TODO: Get rid of this once reference counted cells are working.
        self.fix_ref();

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.create_top_bar(ui);
            });
        });
        egui::SidePanel::left("side_bar").show(ctx, |ui| {
            self.create_side_panel(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.create_main_content(ui);
        });
    }
}
