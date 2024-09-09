use super::prelude::*;

impl Default for Currency {
    fn default() -> Self {
        Self {
            symbol: "DKK".into(),
            suffix: true,
        }
    }
}
