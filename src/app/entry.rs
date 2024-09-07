#[derive(serde::Deserialize, serde::Serialize)]
pub struct Entry {
    // #[serde(skip)]
    pub name: String,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            name: "No name".to_owned(),
        }
    }
}
