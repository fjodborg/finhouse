mod entry;
mod main_app;
mod side_bar;
mod top_bar;

use entry::Entry;

pub mod prelude {
    pub use super::side_bar::SideBar;
    pub use super::top_bar::TopBar;

    pub use super::main_app::FinhouseApp;
}
