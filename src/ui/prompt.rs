use ratatui::{
    layout::{ Constraint, Rect },
    style::{ Color, Stylize },
    widgets::{ Block, Clear },
    Frame, text::Text,
};

use crate::state::{ State, UserInput };

use super::{ content::get_panel_chunks, utils::{ get_layout_v, get_style } };

pub fn draw_prompt(frame: &mut Frame, tab_area: Rect, state: &State) {
    match &state.user_input {
        UserInput::None => {}

        UserInput::NewDirectory(dirname, pos) => {
            if let Some(tab) = state.tabs.get(state.current_tab) {
                let panel_chunks = get_panel_chunks(&tab.panels, tab_area);

                let current_panel_chunk = panel_chunks[tab.current_panel];

                let prompt_chunk = get_layout_v(
                    [ Constraint::Fill(1), Constraint::Length(5) ].as_ref(),
                    current_panel_chunk
                );

                let block = Block::bordered().title("Create file/directory");

                let block_chunk = block.inner(prompt_chunk[1]);

                let name_label = Text::from("Name:");

                let mut cursor_chunk = block_chunk.clone();
                let mut bottom_block_chunk = block_chunk.clone();
                let mut name_chunk = block_chunk.clone();

                cursor_chunk.x += 6 + pos;
                cursor_chunk.width = 1;
                cursor_chunk.height = 1;

                name_chunk.x += 6;
                name_chunk.width -= 6;
                name_chunk.height = 1;

                bottom_block_chunk.height = 2;
                bottom_block_chunk.y += 1;

                let bottom_text = Text::from("Append name with \"/\" to create a directory.\n<Enter>: OK   <Esc>: Cancel.");

                let cursor = Block::new().on_dark_gray();

                let name_block = Block::new().title(dirname.clone()).style(get_style(Color::Black, Color::LightCyan));

                frame.render_widget(Clear, prompt_chunk[1]);
                frame.render_widget(block, prompt_chunk[1]);
                frame.render_widget(name_label, block_chunk);
                frame.render_widget(name_block, name_chunk);
                frame.render_widget(cursor, cursor_chunk);
                frame.render_widget(bottom_text, bottom_block_chunk);
            }
        }

        UserInput::Error(err_str) => {
            if let Some(tab) = state.tabs.get(state.current_tab) {
                let panel_chunks = get_panel_chunks(&tab.panels, tab_area);

                let current_panel_chunk = panel_chunks[tab.current_panel];

                let prompt_chunk = get_layout_v(
                    [ Constraint::Fill(1), Constraint::Length(5) ].as_ref(),
                    current_panel_chunk
                );

                let block = Block::bordered().title("Error").on_red();
                let block_inner_chunk = block.inner(prompt_chunk[1]);
                let mut help_chunk = block_inner_chunk.clone();
                help_chunk.height = 1;
                help_chunk.y = block_inner_chunk.y + block_inner_chunk.height - 1;

                let error_text = Text::from(err_str.clone());
                let help_text = Text::from("<Enter>: Close");

                frame.render_widget(Clear, prompt_chunk[1]);
                frame.render_widget(block, prompt_chunk[1]);
                frame.render_widget(error_text, block_inner_chunk);
                frame.render_widget(help_text, help_chunk);
            }
        }
    }
}

