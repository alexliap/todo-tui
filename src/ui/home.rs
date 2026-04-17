use crate::app::App;
use ratatui::style::{Color, Modifier, Style};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, Paragraph},
};

pub fn ui_home(frame: &mut Frame, app: &mut App) {
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

    let title = Paragraph::new(Text::styled(
        "Rust TODO TUI App",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    ui_home_main(frame, app, chunks[1]);

    ui_home_footer(frame, chunks[2]);
}

pub fn ui_home_main(frame: &mut Frame, app: &mut App, chunk: Rect) {
    let state = &mut app.list_state;

    let items = ["Projects", "Exit"];
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, chunk, state);
}

pub fn ui_home_footer(frame: &mut Frame, chunk: Rect) {
    let current_navigation_text = Span::styled("Home", Style::default().fg(Color::Green));

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = Span::styled(
        "(q) to quit / (p) go to Projects",
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
