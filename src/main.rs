mod ui;
mod state;
mod input;
mod fs;

use crate::{ ui::draw, state::State, input::process_input };

fn main() {
    let mut terminal = ratatui::init();

    let mut state = State::new();

    loop {
        terminal.draw(|frame| draw(frame, &mut state)).expect("Failed to draw!");

        if process_input(&mut state) {
            break;
        }
    }
}
