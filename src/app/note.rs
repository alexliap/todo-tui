use super::project::Project;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub filename: String,
    pub project: Project,
}

impl Note {
    pub fn new(filename: String, project: Project) -> Note {
        Note {
            filename: filename,
            project: project,
        }
    }

    pub fn create(&self) {
        // Path::new(s) — takes anything AsRef<OsStr> (so &str, &String, etc.) and returns a &Path. 
        // It's a borrowing conversion — no allocation, 
        // just reinterpreting the string's bytes as a path view.
        let project_path: &Path = Path::new(&self.project.settings.base_path);
        let note_path: PathBuf = project_path.join(Path::new(&self.filename));
        
        fs::File::create(&note_path).expect("failed to create file");
    }
}
