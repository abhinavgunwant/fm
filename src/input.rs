use crossterm::event::{read, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::state::{Panel, State, Tab};

pub fn process_input(state: &mut State) -> bool {
    match read() {
        Ok(Event::Key(key_event)) => {
            let ctrl = key_event.modifiers == KeyModifiers::CONTROL;
            let shift = key_event.modifiers == KeyModifiers::SHIFT;

            let current_tab = state.tabs.get_mut(state.current_tab).unwrap();
            let current_panel = current_tab.panels.get_mut(current_tab.current_panel).unwrap();

            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Esc => {
                        if state.show_help_menu {
                            state.show_help_menu = false;
                        } else {
                            // TODO: blink or highlight the exit command
                        }
                    }

                    KeyCode::F(1) => {
                        state.show_help_menu = true;
                    }

                    KeyCode::Char('q') => {
                        if ctrl {
                            return true;
                        }
                    }

                    KeyCode::Char('j') | KeyCode::Down => {
                        if current_panel.row < current_panel.current_dir_content.files.len() as u32 - 1 {
                            current_panel.row += 1;
                        }
                    }

                    KeyCode::Char('k') | KeyCode::Up => {
                        if current_panel.row > 0 {
                            current_panel.row -= 1;
                        }
                    }

                    KeyCode::Char('o') => {
                        if ctrl {
                            let entry = current_panel.current_dir_content.files[current_panel.row as usize].clone();
                            if entry.is_directory() {
                                let mut p = current_panel.current_path.clone();
                                p.push(entry.name);

                                state.tabs.push(Tab::new_with_path(p));
                                state.current_tab = state.tabs.len() - 1;
                            }
                        }
                    }

                    KeyCode::Char('p') => {
                        if ctrl {
                            let entry = current_panel.current_dir_content.files[current_panel.row as usize].clone();
                            if entry.is_directory() {
                                let mut p = current_panel.current_path.clone();
                                p.push(entry.name);

                                current_tab.panels.push(Panel::new_with_path(p));
                                current_tab.current_panel = current_tab.panels.len() - 1;
                            }
                        }
                    }

                    KeyCode::Char('w') => {
                        if ctrl {
                            state.tabs.remove(state.current_tab);

                            if state.current_tab > 0 {
                                state.current_tab -= 1;
                            } else {
                                std::process::exit(0);
                            }
                        }
                    }

                    KeyCode::Tab => {
                        state.current_tab = (state.current_tab + 1) % state.tabs.len();
                    }

                    KeyCode::Enter => {
                        let entry = current_panel.current_dir_content.files[current_panel.row as usize].clone();

                        if entry.is_directory() {
                            current_panel.current_path.push(entry.name);
                            current_panel.row = 0;
                        } else {
                            let mut file_path = current_panel.current_path.clone();

                            file_path.push(entry.name);

                            match file_path.to_str() {
                                Some(path_str) => {
                                    match open::that(path_str) {
                                        Ok(()) => {}
                                        Err(_e) => {
                                            // TODO: show error here...
                                        }
                                    }
                                }

                                None => {}
                            }
                        }
                    }

                    KeyCode::Backspace => {
                        current_panel.current_path.pop();
                        current_panel.row = 0;
                    }

                    KeyCode::Char('h') | KeyCode::Left => {
                        if current_tab.panels.len() > 1 && current_tab.current_panel > 0 {
                            current_tab.current_panel -= 1;
                        }
                    }

                    KeyCode::Char('l') | KeyCode::Right => {
                        if current_tab.panels.len() > 1 {
                            current_tab.current_panel = (current_tab.current_panel + 1) % current_tab.panels.len();
                        }
                    }

                    _ => {}
                }
            }
        }

        Ok(_) => {}

        Err(_) => {}
    }

    false
}

