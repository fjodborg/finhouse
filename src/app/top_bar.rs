use super::{finhouse, Entry};

pub trait TopBar {
    /// Create the top bar.
    fn create_top_bar(&mut self, ui: &mut egui::Ui);
}

impl TopBar for finhouse::FinhouseApp {
    fn create_top_bar(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            // TODO: Make this depend on font size.
            egui::widgets::global_dark_light_mode_buttons(ui);
            ui.separator();

            // Add a small buffer for scrollbar.
            ui.set_height(24.0);

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.horizontal(|ui| {
                    // Add new entry if clicked and set newest entry as selected.
                    if ui.button("Ny Bolig!").highlight().clicked() {
                        // TODO, find a more idiomatic way to do this.
                        self.selected_entry = self.entries.len();
                        self.entries.push(Entry::default());
                    }

                    ui.separator();
                });

                // Generate entry tabs into scroll area.
                egui::ScrollArea::horizontal()
                    .auto_shrink([false, false])
                    .stick_to_right(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let mut remove_tab_opt = None;

                            // Generate tabs/buttons and check which button was clicked.
                            for i in 0..self.entries.len() {
                                let text = format!("{} {}", i, self.entries[i].name);
                                let button = ui.selectable_value(&mut self.selected_entry, i, text);

                                // Set clicked tab as selected.
                                if button.clicked() {
                                    self.selected_entry = i;
                                }
                                // Remove right clicked tab.
                                else if button.clicked_by(egui::PointerButton::Secondary) {
                                    remove_tab_opt = Some(i);
                                }
                            }

                            // If a tab was right clicked run removal logic.
                            if let Some(remove_tab) = remove_tab_opt {
                                // Handle edge case where last tab is deleted.
                                if self.entries.len() == 1 {
                                    // Reset the last tab if deleted.
                                    self.entries.push(Entry::default());
                                }
                                // Handle if selected entry is after deleted tab.
                                else if self.selected_entry >= remove_tab
                                    && self.selected_entry != 0
                                {
                                    self.selected_entry -= 1;
                                }
                                self.entries.remove(remove_tab);
                            }
                        });
                    });
            });
        });
    }
}
