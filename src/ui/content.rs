use std::rc::Rc;

use ratatui::{layout::{Constraint, Rect}, style::Color, text::Text, Frame};

use crate::{ fs::{dir_contents::DirectoryContents, get_dir_contents}, state::{ Panel, Tab }, ui::utils::{ get_layout_h, get_style, get_style_fg } };

pub fn draw_tab_line(
    frame: &mut Frame, tab_headings: Vec<String>, active_tab: usize, rect: Rect
) {
    let tab_style = get_style_fg(Color::Magenta);
    let tab_active_style = get_style(Color::Black, Color::Magenta);

    let mut tab_constraints: Vec<Constraint> = Vec::with_capacity(2);

    let mut last_increment = 0;
    let total_tabs = tab_headings.len() as u16;

    let percent = (1.0f32/(total_tabs as f32) * 100.0f32) as u16;

    if percent * (total_tabs as u16) != 100 {
        last_increment = 100 - percent * total_tabs;
    }

    for i in 0..total_tabs {
        if i == total_tabs - 1 {
            tab_constraints.push(Constraint::Percentage(percent + last_increment));
            continue;
        }

        tab_constraints.push(Constraint::Percentage(percent));
    }

    let tab_chunks = get_layout_h(tab_constraints.as_ref(), rect);

    for (i, tab_heading) in tab_headings.iter().enumerate() {
        frame.render_widget(
            Text::styled(
                tab_heading,
                if i == active_tab { tab_active_style } else { tab_style }
            ),
            tab_chunks[i]
        );
    }
}

pub fn draw_tab_content(frame: &mut Frame, tab: &mut Tab, rect: Rect) {
    let mut tab_panel_constraints: Vec<Constraint> = Vec::with_capacity(2);

    let total_panels = tab.panels.len();
    let percent = (1.0f32/(total_panels as f32) * 100.0f32) as u16;

    let mut last_increment = 0;

    if percent * (total_panels as u16) != 100 {
        last_increment = 100 - percent * (total_panels as u16);
    }

    for i in 0..total_panels {
        if i == total_panels - 1 {
            tab_panel_constraints.push(Constraint::Percentage(percent + last_increment));
            continue;
        }

        tab_panel_constraints.push(Constraint::Percentage(percent));
    }

    let panel_chunks = get_layout_h(tab_panel_constraints.as_ref(), rect);

    for i in 0..total_panels {
        match tab.panels.get_mut(i) {
            Some(panel) => {
                draw_panel_content(frame, panel, panel_chunks[i]);
            }

            None => {}
        }
    }
}

pub fn draw_panel_content(frame: &mut Frame, panel: &mut Panel, original_rect: Rect) {
    let dir_style = get_style_fg(Color::Yellow);
    let dir_style_selected = get_style(Color::Black, Color::Yellow);
    let file_style = get_style_fg(Color::Blue);
    let file_style_selected = get_style(Color::Black, Color::Blue);

    let mut rect: Rect = original_rect.clone();
    rect.height = 1;

    let contents: Rc<DirectoryContents>;

    if panel.current_path == panel.current_dir_content.path {
        contents = panel.current_dir_content.clone();
    } else {
        contents = Rc::new(get_dir_contents(panel.current_path.clone()));
        panel.current_dir_content = contents.clone();
    };

    let last_index = panel.list_start_index + original_rect.height as u32 - 1;

    if panel.row > last_index {
        panel.list_start_index = panel.row - original_rect.height as u32 + 1;
    } else if panel.row < panel.list_start_index {
        panel.list_start_index = panel.row;
    }

    let mut items_displayed: usize = 0;

    for (i, content) in contents.files.iter().enumerate() {
        if i < panel.list_start_index as usize {
            continue;
        }

        let entry;

        if content.is_directory() {
            entry = if i == panel.row as usize {
                Text::styled(format!("\u{f4d3} {}", content.name.clone()), dir_style_selected)
            } else {
                Text::styled(format!("\u{f4d3} {}", content.name.clone()), dir_style)
            };
        } else {
            entry = if i == panel.row as usize {
                Text::styled(format!("\u{f15b} {}", content.name.clone()), file_style_selected)
            } else {
                Text::styled(format!("\u{f15b} {}", content.name.clone()), file_style)
            };
        }

        frame.render_widget(entry, rect);
        rect.y += 1;
        items_displayed += 1;

        if items_displayed == original_rect.height as usize {
            break;
        }
    }
}

