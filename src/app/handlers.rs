use ratatui::crossterm::event::KeyCode;
use std::path::Path;
use super::project::Project;
use super::note::Note;
use super::screens::{CurrentScreen, NoteScreen, ProjectScreen, SettingScreen};
use super::state::App;

impl App {
    pub fn handle_home_screen(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Down => {
                self.list_state.select_next();
                false
            }
            KeyCode::Up => {
                self.list_state.select_previous();
                false
            }
            KeyCode::Enter => {
                if let Some(index) = self.list_state.selected() {
                    match index {
                        0 => {
                            self.current_screen = CurrentScreen::Projects(ProjectScreen::Main);
                            false
                        }
                        1 => {
                            self.current_screen = CurrentScreen::Settings(SettingScreen::Main);
                            self.list_state.select(Some(0));
                            false
                        }
                        2 => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            KeyCode::Char('p') => {
                self.current_screen = CurrentScreen::Projects(ProjectScreen::Main);
                false
            }
            KeyCode::Char('q') => {
                self.current_screen = CurrentScreen::Exiting;
                true
            }
            _ => false,
        }
    }

    pub fn handle_projects_screen(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Down => {
                self.list_state.select_next();
                false
            }
            KeyCode::Up => {
                self.list_state.select_previous();
                false
            }
            KeyCode::Enter => {
                if let Some(index) = self.list_state.selected() {
                    match index {
                        0 => {
                            self.current_screen = CurrentScreen::Projects(ProjectScreen::Create);
                            false
                        }
                        1 => {
                            self.available_projects();
                            self.current_screen = CurrentScreen::Projects(ProjectScreen::Open);
                            false
                        }
                        2 => {
                            self.current_screen = CurrentScreen::Main;
                            // reset the index when changing screen
                            self.list_state.select(Some(0));
                            false
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            KeyCode::Char('c') => {
                self.current_screen = CurrentScreen::Projects(ProjectScreen::Create);
                false
            }
            KeyCode::Char('q') => {
                self.current_screen = CurrentScreen::Main;
                false
            }
            _ => false,
        }
    }

    pub fn handle_projects_open(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Down => {
                self.project_list.select_next();
                false
            }
            KeyCode::Up => {
                self.project_list.select_previous();
                false
            }
            KeyCode::Enter => {
                if let Some(index) = self.project_list.selected() {
                    if let Some(name) = self.project_items.get(index).cloned() {
                        self.selected_project = Some(name);
                        self.current_screen = CurrentScreen::Notes(NoteScreen::Main);
                        self.project_list.select(Some(0));
                        self.list_state.select(Some(0));
                    }
                }
                false
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.current_screen = CurrentScreen::Projects(ProjectScreen::Main);
                self.list_state.select(Some(0));
                false
            }
            _ => false,
        }
    }

    pub fn handle_notes_open(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Down => {
                self.note_list.select_next();
                false
            }
            KeyCode::Up => {
                self.note_list.select_previous();
                false
            }
            KeyCode::Enter => {
                if let Some(index) = self.note_list.selected() {
                    if let Some(name) = self.note_items.get(index).cloned() {
                        self.load_note(&name);
                        self.current_screen = CurrentScreen::Notes(NoteScreen::Edit);
                    }
                }
                false
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.current_screen = CurrentScreen::Notes(NoteScreen::Main);
                self.list_state.select(Some(0));
                false
            }
            _ => false,
        }
    }

    pub fn handle_notes_screen(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Down => {
                self.list_state.select_next();
                false
            }
            KeyCode::Up => {
                self.list_state.select_previous();
                false
            }
            KeyCode::Enter => {
                if let Some(index) = self.list_state.selected() {
                    match index {
                        0 => {
                            self.current_screen = CurrentScreen::Notes(NoteScreen::Create);
                            false
                        }
                        1 => {
                            self.available_notes();
                            self.current_screen = CurrentScreen::Notes(NoteScreen::Open);
                            false
                        }
                        2 => {
                            self.current_screen = CurrentScreen::Projects(ProjectScreen::Main);
                            // reset the index when changing screen
                            self.list_state.select(Some(0));
                            false
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.selected_project = None;
                self.current_screen = CurrentScreen::Projects(ProjectScreen::Main);
                self.list_state.select(Some(0));
                false
            }
            _ => false,
        }
    }

    pub fn handle_projects_create(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Char(c) => {
                self.input.push(c);
                false
            }
            KeyCode::Backspace => {
                self.input.pop();
                false
            }
            KeyCode::Esc => {
                self.input.clear();
                self.current_screen = CurrentScreen::Projects(ProjectScreen::Main);
                false
            }
            KeyCode::Enter => {
                Project::new(self.input.clone()).create();
                self.input.clear();
                self.current_screen = CurrentScreen::Projects(ProjectScreen::Main);
                false
            }
            _ => false,
        }
    }

    pub fn handle_notes_create(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Char(c) => {
                self.input.push(c);
                false
            }
            KeyCode::Backspace => {
                self.input.pop();
                false
            }
            KeyCode::Esc => {
                self.input.clear();
                self.current_screen = CurrentScreen::Notes(NoteScreen::Main);
                false
            }
            KeyCode::Enter => {
                let Some(project) = &self.selected_project else { return false; };
                let path = Path::new(&self.settings.base_path).join(project);
                
                // let project_path: &Path = Path::new(&self.selected_project);
                // let note_path: PathBuf = project_path.join(Path::new(&self.name));
                Note::new(self.input.clone()).create(&path);
                self.input.clear();
                self.current_screen = CurrentScreen::Notes(NoteScreen::Main);
                false
            }
            _ => false,
        }
    }

    pub fn handle_notes_edit(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Char(c) => {
                self.note_buffer.push(c);
                false
            }
            KeyCode::Enter => {
                self.note_buffer.push('\n');
                false
            }
            KeyCode::Backspace => {
                self.note_buffer.pop();
                false
            }
            KeyCode::Esc => {
                self.save_note();
                self.note_buffer.clear();
                self.selected_note = None;
                self.current_screen = CurrentScreen::Notes(NoteScreen::Main);
                false
            }
            _ => false,
        }
    }

    pub fn handle_settings_screen(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Down => {
                self.list_state.select_next();
                false
            }
            KeyCode::Up => {
                self.list_state.select_previous();
                false
            }
            KeyCode::Enter => {
                if let Some(index) = self.list_state.selected() {
                    match index {
                        0 => {
                            self.current_screen = CurrentScreen::Settings(SettingScreen::Set);
                            false
                        }
                        1 => {
                            self.current_screen = CurrentScreen::Main;
                            // reset the index when changing screen
                            self.list_state.select(Some(0));
                            false
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            KeyCode::Char('q') => {
                self.current_screen = CurrentScreen::Main;
                self.list_state.select(Some(0));
                false
            }
            _ => false,
        }
    }

    pub fn handle_settings_set(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Char(c) => {
                self.settings.base_path.push(c);
                false
            }
            KeyCode::Backspace => {
                self.settings.base_path.pop();
                false
            }
            KeyCode::Esc => {
                self.settings.base_path.clear();
                self.current_screen = CurrentScreen::Settings(SettingScreen::Main);
                false
            }
            KeyCode::Enter => {
                self.settings.save();
                self.current_screen = CurrentScreen::Settings(SettingScreen::Main);
                false
            }
            _ => false,
        }
    }
}
