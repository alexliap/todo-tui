mod common;
mod home;
mod projects;

use crate::app::{App, CurrentScreen, ProjectScreen};
use ratatui::Frame;

use home::ui_home;
use projects::{ui_projects, ui_projects_create};

pub fn ui(frame: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::Main => ui_home(frame, app),
        CurrentScreen::Projects(ProjectScreen::Main) => ui_projects(frame, app),
        CurrentScreen::Projects(ProjectScreen::Create) => {
            ui_projects(frame, app);
            ui_projects_create(frame, app);
        }
        CurrentScreen::Exiting => (),
    }
}
