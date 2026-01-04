use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::Text, Frame};

const HELP_TEXT: [(&str, &str); 3] = [
    ("<F1>", "Show (this) help menu."),
    ("j/\u{f063}", "Move one item down the list"),
    ("k/\u{f062}", "Move one item up the list"),
];

pub fn draw_help_menu(frame: &mut Frame) {
    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ].as_ref())
        .split(frame.area());

    let title_style = Style::default()
        .bg(Color::Yellow)
        .fg(Color::Black);

    let title_text = Text::styled("fm Help", title_style).centered();
    let bottom_text = Text::from("<Esc>: Go back");

    let mut rect = outer_chunks[1];
    rect.height = 1;
    rect.y += 1;

    for item in HELP_TEXT.iter() {
        let line_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(10),
                Constraint::Min(1),
            ].as_ref())
            .split(rect);

        let left = Text::from(format!("  {}", item.0));
        let right = Text::from(format!("  {}", item.1));

        frame.render_widget(left, line_chunks[0]);
        frame.render_widget(right, line_chunks[1]);

        rect.y += 1;
    }

    frame.render_widget(title_text, outer_chunks[0]);
    frame.render_widget(bottom_text, outer_chunks[2]);
}

