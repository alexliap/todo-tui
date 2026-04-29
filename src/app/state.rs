use std::fs;

use ratatui::widgets::ListState;

use super::screens::CurrentScreen;
use super::settings::Settings;

pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub list_state: ListState,
    pub input: String,
    pub settings: Settings,
    pub project_items: Vec<String>,
    pub project_list: ListState,
    pub selected_project: Option<String>,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            list_state: ListState::default().with_selected(Some(0)),
            input: String::new(),
            settings: Settings::new(),
            project_items: Vec::new(),
            project_list: ListState::default().with_selected(Some(0)),
            selected_project: None,
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
}
