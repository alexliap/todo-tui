use ratatui::crossterm::event::KeyCode;
use ratatui::widgets::ListState;

pub enum CurrentScreen {
    Main,
    Projects(ProjectScreen),
    Exiting,
}

pub enum ProjectScreen {
    Main,
    Create,
    // OpenProject,
}

pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub list_state: ListState,
    pub input: String,
}

impl App {
    pub fn new() -> App {
        let list_state = ListState::default().with_selected(Some(0));

        App {
            current_screen: CurrentScreen::Main,
            list_state: list_state,
            input: String::new(),
        }
    }

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
                        1 => true,
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
                self.input.clear();
                self.current_screen = CurrentScreen::Projects(ProjectScreen::Main);
                false
            }
            _ => false,
        }
    }
}
