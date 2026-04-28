use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub base_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            base_path: "./".to_string(),
        }
    }
}

impl Settings {
    pub fn new() -> Settings {
        let config_path = Path::new("./settings.toml");
        if config_path.exists() {
            Self::load("./settings.toml".to_string())
        } else {
            let defaults = Settings::default();
            let serialized =
                toml::to_string(&defaults).expect("failed to serialize default settings");
            if let Some(parent) = config_path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent).expect("failed to create config dir");
                }
            }
            fs::write(config_path, serialized).expect("failed to write settings.toml");
            defaults
        }
    }

    pub fn load(config_path: String) -> Settings {
        let contents = fs::read_to_string(config_path).expect("failed to read settings.toml");
        toml::from_str(&contents).expect("failed to parse settings.toml")
    }

    pub fn save(&mut self) {
        let serialized = toml::to_string(&self).expect("failed to serialize default settings");
        fs::write("./settings.toml", serialized).expect("failed to write settings.toml");
    }
}
