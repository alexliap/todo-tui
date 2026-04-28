use super::settings::Settings;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub settings: Settings,
}

impl Project {
    pub fn new(name: String) -> Project {
        Project {
            name: name,
            settings: Settings::load("./settings.toml".to_string()),
        }
    }

    pub fn create(self) {
        fs::create_dir(self.settings.base_path.clone() + &self.name)
            .expect("failed to create config dir");
    }
}
