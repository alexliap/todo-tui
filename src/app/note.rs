use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub name: String,
}

impl Note {
    pub fn new(name: String) -> Note {
        Note {
            name: name,
        }
    }

    pub fn create(&self, project_path: &Path) {
        // Path::new(s) — takes anything AsRef<OsStr> (so &str, &String, etc.) and returns a &Path. 
        // It's a borrowing conversion — no allocation, 
        // just reinterpreting the string's bytes as a path view.
        let note_path: PathBuf = project_path.join(Path::new(&self.name));
        
        fs::File::create(&note_path).expect("failed to create file");
    }
}
