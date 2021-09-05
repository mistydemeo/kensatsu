use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub emitter: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            emitter: "io".to_string(),
        }
    }
}
