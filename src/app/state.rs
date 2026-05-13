use std::fs;
use std::path::Path;
use ratatui::widgets::ListState;

use super::screens::CurrentScreen;
use super::settings::Settings;

pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub list_state: ListState,
    pub input: String,
    pub settings: Settings,
    pub project_list: ListState,
    pub project_items: Vec<String>,
    pub selected_project: Option<String>,
    pub note_list: ListState,
    pub note_items: Vec<String>,
    pub selected_note: Option<String>,
    pub note_buffer: String,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            list_state: ListState::default().with_selected(Some(0)),
            input: String::new(),
            settings: Settings::new(),
            project_list: ListState::default().with_selected(Some(0)),
            project_items: Vec::new(),
            selected_project: None,
            note_list: ListState::default().with_selected(Some(0)),
            note_items: Vec::new(),
            selected_note: None,
            note_buffer: String::new(),
        }
    }

    pub fn available_projects(&mut self) {
        self.project_items = fs::read_dir(&self.settings.base_path)
            .ok()
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
                    .filter_map(|e| e.file_name().into_string().ok())
                    .collect()
            })
            .unwrap_or_default();

        self.project_list.select(Some(0));
    }

    pub fn available_notes(&mut self) {
        let Some(project) = &self.selected_project else { return; };
        let path = Path::new(&self.settings.base_path).join(project);

        self.note_items = fs::read_dir(path)
            .ok()
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
                    .filter_map(|e| e.file_name().into_string().ok())
                    .collect()
            })
            .unwrap_or_default();

        self.note_list.select(Some(0));
    }

    pub fn load_note(&mut self, name: &str) {
        let Some(project) = &self.selected_project else { return; };
        let path = Path::new(&self.settings.base_path).join(project).join(name);

        self.note_buffer = fs::read_to_string(&path).unwrap_or_default();
        self.selected_note = Some(name.to_string());
    }

    pub fn save_note(&self) {
        let (Some(project), Some(name)) = (&self.selected_project, &self.selected_note) else { return; };
        let path = Path::new(&self.settings.base_path).join(project).join(name);

        let _ = fs::write(&path, &self.note_buffer);
    }
}
