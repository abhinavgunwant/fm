mod help;
mod utils;
mod content;
mod prompt;

use chrono::{ DateTime, Local, Timelike };

use prompt::draw_prompt;
use ratatui:: {
    layout::{Constraint, Rect}, style::{Color, Style}, text::Text, Frame,
};

use crate::{
    state::{ BottomLineContent, State, Tab, UserInput },
    ui::{
        content::{ draw_tab_content, draw_tab_line },
        utils::{ get_layout_h, get_layout_v, get_style },
        help::draw_help_menu,
    },
};

fn draw_bottom_line(frame: &mut Frame, area: Rect, state: &mut State) {
    let title_style = get_style(Color::Yellow, Color::Black);

    let bottom_chunk = get_layout_h(
        [ Constraint::Length(20), Constraint::Fill(1) ].as_ref(),
        area,
    );

    let status_chunks = get_layout_h(
        [ Constraint::Percentage(50), Constraint::Fill(1) ].as_ref(),
        bottom_chunk[1],
    );

    let title_text = Text::styled(" fm: File Manager", title_style);

    match state.bottom_line_content {
        BottomLineContent::HelpText => {
            let help_text = Text::styled("F1: Open help menu", Style::default());
            let exit_text = Text::styled("<Ctrl> + Q: Exit", Style::default());

            frame.render_widget(help_text, status_chunks[0]);
            frame.render_widget(exit_text, status_chunks[1]);
        }

        BottomLineContent::RefreshedAt => {
            if let Some(active_tab) = state.tabs.get(state.current_tab) {
                if let Some(active_panel) = active_tab.panels.get(active_tab.current_panel) {
                    let date_time: DateTime<Local> = active_panel.last_updated;
                    let hours = date_time.hour();
                    let minutes = date_time.minute();
                    let seconds = date_time.second();

                    let refresh_text = Text::from(format!("Refreshed at: {:02}:{:02}:{:02}", hours, minutes, seconds));
                    frame.render_widget(refresh_text, bottom_chunk[1]);
                }
            }
        }
    }


    frame.render_widget(title_text, bottom_chunk[0]);
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

    draw_bottom_line(frame, outer_chunks[1], state);

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

    if state.user_input != UserInput::None {
        draw_prompt(frame, content_chunk, state);
    }
}

