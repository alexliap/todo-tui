mod common;
mod home;
mod notes;
mod projects;
mod settings;

use crate::app::{App, CurrentScreen, NoteScreen, ProjectScreen, SettingScreen};
use ratatui::Frame;

use home::ui_home;
use notes::{ui_notes, ui_notes_create, ui_notes_open, ui_notes_edit};
use projects::{ui_projects, ui_projects_create, ui_projects_open};
use settings::{ui_settings, ui_settings_set};

pub fn ui(frame: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::Main => ui_home(frame, app),
        CurrentScreen::Projects(ProjectScreen::Main) => ui_projects(frame, app),
        CurrentScreen::Projects(ProjectScreen::Create) => {
            ui_projects(frame, app);
            ui_projects_create(frame, app);
        }
        CurrentScreen::Projects(ProjectScreen::Open) => {
            ui_projects(frame, app);
            ui_projects_open(frame, app);
        }
        CurrentScreen::Notes(NoteScreen::Main) => ui_notes(frame, app),
        CurrentScreen::Notes(NoteScreen::Create) => {
            ui_notes(frame, app);
            ui_notes_create(frame, app);
        }
        CurrentScreen::Notes(NoteScreen::Open) => {
            ui_notes(frame, app);
            ui_notes_open(frame, app);
        }
        CurrentScreen::Notes(NoteScreen::Edit) => {
            ui_notes_edit(frame, app);
        }
        CurrentScreen::Settings(SettingScreen::Main) => ui_settings(frame, app),
        CurrentScreen::Settings(SettingScreen::Set) => {
            ui_settings(frame, app);
            ui_settings_set(frame, app);
        }
        CurrentScreen::Exiting => (),
    }
}
