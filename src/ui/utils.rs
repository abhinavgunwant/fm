use std::rc::Rc;

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}};

pub fn get_style(fg: Color, bg: Color) -> Style {
    Style::default().fg(fg).bg(bg)
}

pub fn get_style_fg(fg: Color) -> Style { Style::default().fg(fg) }

pub fn get_layout_h(constraints: &[Constraint], rect: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(rect)
}

pub fn get_layout_v(constraints: &[Constraint], rect: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(rect)
}

