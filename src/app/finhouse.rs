use std::{cell::RefCell, rc::Rc};

use crate::app::entry::Percentage;

use super::Entry;
use super::FinhouseApp;

impl FinhouseApp {
    /// Do not call this. It is handled by the main thread (main.rs/lib.rs).
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    // TODO: This is just a hacky workaround to get references to not get initialized to 0 when loading app from storage.
    pub fn fix_ref(&mut self) {
        // Overwrite plot duration with actual value.
        for entry in &mut self.entries {
            entry.plot_duration = self.plot_years.clone();
        }
    }
}

impl Default for FinhouseApp {
    fn default() -> Self {
        use super::entry::MultiLines;
        let plot_years = Rc::new(RefCell::new(50));

        let mut entry1 = Entry::new(plot_years.clone());
        let mut entry2 = Entry::new(plot_years.clone());

        Self {
            entries: vec![entry1, entry2],
            selected_entry: 0,
            plot_years: plot_years,
        }
    }
}
