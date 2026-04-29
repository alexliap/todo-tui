use crate::app::App;
use ratatui::style::{Color, Style};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

pub fn ui_notes(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let project_name = app.selected_project.as_deref().unwrap_or("(none)");

    let title_block = Block::default().borders(Borders::ALL).style(Style::default());
    let title = Paragraph::new(Text::styled(
        format!("Notes — {project_name}"),
        Style::default().fg(Color::Green),
    ))
    .block(title_block);
    frame.render_widget(title, chunks[0]);

    let body = Paragraph::new("Notes view (placeholder)")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(body, chunks[1]);

    let hint = Paragraph::new(Line::from(Span::styled(
        "(q) to go back",
        Style::default().fg(Color::Red),
    )))
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(hint, chunks[2]);
}
