use std::{error::Error, io};

use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, NoteScreen, ProjectScreen, SettingScreen},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            ();
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| ui(f, app as &mut App))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => {
                    if app.handle_home_screen(key.code) {
                        return Ok(true);
                    }
                }
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Projects(ProjectScreen::Main) => {
                    if app.handle_projects_screen(key.code) {
                        return Ok(true);
                    }
                }
                CurrentScreen::Projects(ProjectScreen::Create) => {
                    if app.handle_projects_create(key.code) {
                        return Ok(true);
                    }
                }
                CurrentScreen::Projects(ProjectScreen::Open) => {
                    if app.handle_projects_open(key.code) {
                        return Ok(true);
                    }
                }
                CurrentScreen::Notes(NoteScreen::Main) => {
                    if app.handle_notes_screen(key.code) {
                        return Ok(true);
                    }
                }
                CurrentScreen::Settings(SettingScreen::Main) => {
                    if app.handle_settings_screen(key.code) {
                        return Ok(true);
                    }
                }
                CurrentScreen::Settings(SettingScreen::Set) => {
                    if app.handle_settings_set(key.code) {
                        return Ok(true);
                    }
                }
            }
        }
    }
}
