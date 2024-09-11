mod entry;
mod finhouse;
mod main_content;
mod side_bar;
mod top_bar;

use entry::Entry;

pub mod prelude {
    pub use super::main_content::MainContent;
    pub use super::side_bar::SideBar;
    pub use super::top_bar::TopBar;

    pub use super::FinhouseApp;
}

use std::{cell::RefCell, rc::Rc};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct FinhouseApp {
    pub selected_entry: usize,
    pub entries: Vec<Entry>,
    pub plot_years: Rc<RefCell<u32>>,
}
