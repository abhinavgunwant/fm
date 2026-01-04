mod help;
mod utils;
mod content;

use std::rc::Rc;

use ratatui:: {
    layout::{Constraint, Rect}, style::{Color, Style}, text::Text, Frame,
};

use crate::{
    fs::{ dir_contents::DirectoryContents, get_dir_contents },
    state::{ State, Tab },
    ui::{
        help::draw_help_menu,
        utils::{ get_style, get_style_fg, get_layout_h, get_layout_v },
        content::{ draw_tab_line, draw_tab_content },
    },
};

fn draw_bottom_line(frame: &mut Frame, area: Rect) {
    let title_style = get_style(Color::Yellow, Color::Black);

    let bottom_chunk = get_layout_h(
        [ Constraint::Percentage(33), Constraint::Fill(1) ].as_ref(),
        area,
    );

    let help_chunks = get_layout_h(
        [ Constraint::Percentage(50), Constraint::Fill(1) ].as_ref(),
        bottom_chunk[1],
    );

    let title_text = Text::styled("fm: File Manager", title_style);
    let help_text = Text::styled("F1: Open help menu", Style::default());
    let exit_text = Text::styled("<Ctrl> + Q: Exit", Style::default());

    frame.render_widget(title_text, bottom_chunk[0]);
    frame.render_widget(help_text, help_chunks[0]);
    frame.render_widget(exit_text, help_chunks[1]);
}

pub fn draw(frame: &mut Frame, state: &mut State) {
    if state.show_help_menu {
        draw_help_menu(frame);
        return;
    }

    let outer_chunks = get_layout_v(
        [ Constraint::Min(1), Constraint::Length(1) ].as_ref(),
        frame.area(),
    );

    draw_bottom_line(frame, outer_chunks[1]);

    let content_chunk: Rect;

    if state.tabs.len() > 1 {
        let content_chunk_with_tab = get_layout_v(
            [ Constraint::Length(1), Constraint::Min(1) ].as_ref(),
            outer_chunks.as_ref()[0]
        );

        content_chunk = content_chunk_with_tab[1];

        let mut tab_headings: Vec<String> = Vec::with_capacity(4);

        let mut tab_title: u16 = 0;

        for tab in state.tabs.iter() {
            let active_panel = &tab.panels[tab.current_panel];

            let file_name = match active_panel.current_path.file_name() {
                Some(f_name) => match f_name.to_str() {
                    Some(f) => format!(" {} ", f),
                    None => {
                        tab_title += 1;
                        format!("Tab {}", tab_title)
                    },
                },

                None => {
                    tab_title += 1;
                    format!("Tab {}", tab_title)
                }
            };

            tab_headings.push(file_name);
        }

        draw_tab_line(
            frame, tab_headings, state.current_tab, content_chunk_with_tab[0]
        );
    } else {
        content_chunk = outer_chunks[0];
    }

    let current_tab: &mut Tab = state.tabs.get_mut(state.current_tab).unwrap();
    draw_tab_content(frame, current_tab, content_chunk);
}

