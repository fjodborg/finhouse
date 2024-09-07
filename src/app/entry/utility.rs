// TODO: Proper type checking later (Place holder for now).
#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Percentage(pub f64);

impl Into<f64> for Percentage {
    fn into(self) -> f64 {
        self.0 / 100.0
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MultiLines {
    pub name: String,
    pub value: u64,
}
