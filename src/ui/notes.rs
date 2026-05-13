use super::common::{centered_rect, cursor_pos};
use crate::app::App;
use ratatui::style::{Color, Style, Modifier};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, List, Clear, Wrap},
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

    ui_notes_main(frame, app, chunks[1]);

    ui_notes_footer(frame, chunks[2]);
}

pub fn ui_notes_main(frame: &mut Frame, app: &mut App, chunk: Rect){

    let state = &mut app.list_state;

    let items = ["Create New", "Open Note", "Back"];
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, chunk, state);
}

pub fn ui_notes_footer(frame: &mut Frame, chunk: Rect) {
    let current_navigation_text = Span::styled("Notes", Style::default().fg(Color::Yellow));

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = Span::styled(
        "(q) to go back / (c) create a new note",
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

pub fn ui_notes_create(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, frame.area());

    let title_block = Block::default()
        .title(" Note name ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    let title_text = Paragraph::new(app.input.clone()).block(title_block);

    frame.render_widget(Clear, area);
    frame.render_widget(title_text, area);

    frame.set_cursor_position(cursor_pos(area, &app.input));
}

pub fn ui_notes_edit(frame: &mut Frame, app: &App) {
    let area = centered_rect(80, 80, frame.area());

    let note_name = app.selected_note.as_deref().unwrap_or("(unsaved)");
    let block = Block::default()
        .title(format!(" {note_name} — Esc to save & close "))
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    let body = Paragraph::new(app.note_buffer.clone())
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(Clear, area);
    frame.render_widget(body, area);

    frame.set_cursor_position(cursor_pos(area, &app.note_buffer));
}

pub fn ui_notes_open(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(60, 40, frame.area());

    let block = Block::default()
        .title(" Select Note ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_widget(Clear, area);

    if app.note_items.is_empty() {
        let empty: Paragraph<'_> = Paragraph::new("No notes found").block(block);
        frame.render_widget(empty, area);
    } else {
        let list = List::new(app.note_items.clone())
            .block(block)
            .style(Color::White)
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut app.note_list);
    }
}
