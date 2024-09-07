#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct FinhouseApp {
    pub value: f64,
    pub value2: u64,
}

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
}

impl Default for FinhouseApp {
    fn default() -> Self {
        Self {
            value: 1.0,
            value2: 0,
        }
    }
}
