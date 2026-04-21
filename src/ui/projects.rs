use super::common::centered_rect;
use crate::app::App;
use ratatui::style::{Color, Modifier, Style};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, Paragraph},
};

pub fn ui_projects(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("Projects", Style::default().fg(Color::Green)))
        .block(title_block);

    frame.render_widget(title, chunks[0]);

    ui_projects_main(frame, app, chunks[1]);

    ui_projects_footer(frame, chunks[2]);
}

pub fn ui_projects_main(frame: &mut Frame, app: &mut App, chunk: Rect) {
    let state = &mut app.list_state;

    let items = ["Create New", "Open Project", "Back"];
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, chunk, state);
}

pub fn ui_projects_footer(frame: &mut Frame, chunk: Rect) {
    let current_navigation_text = Span::styled("Projects", Style::default().fg(Color::Yellow));

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = Span::styled(
        "(q) to go back / (c) create a new project",
        Style::default().fg(Color::Red),
    );

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunk);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}

pub fn ui_projects_create(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, frame.area());

    let title_block = Block::default()
        .title(" Enter a project title ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    let title_text = Paragraph::new(app.input.clone()).block(title_block);

    frame.render_widget(Clear, area);
    frame.render_widget(title_text, area);
}
