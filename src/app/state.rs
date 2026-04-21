use ratatui::widgets::ListState;

use super::screens::CurrentScreen;
use super::settings::Settings;

pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub list_state: ListState,
    pub input: String,
    pub settings: Settings,
}

impl App {
    pub fn new() -> App {
        let list_state = ListState::default().with_selected(Some(0));

        App {
            current_screen: CurrentScreen::Main,
            list_state,
            input: String::new(),
            settings: Settings::new(),
        }
    }
}
