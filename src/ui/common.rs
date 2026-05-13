use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Compute the cursor position inside a bordered input/editor block.
/// Counts `\n` to advance lines; everything else advances the column.
/// Naive: ignores soft-wrap, so very long lines may overflow visually.
pub fn cursor_pos(area: Rect, text: &str) -> (u16, u16) {
    let mut line: u16 = 0;
    let mut col: u16 = 0;
    for c in text.chars() {
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (area.x + 1 + col, area.y + 1 + line)
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
