use ratatui::crossterm::event::KeyCode;

use super::projects::Project;
use super::screens::{CurrentScreen, ProjectScreen, SettingScreen};
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
                            self.current_screen = CurrentScreen::Projects(ProjectScreen::Main);
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
