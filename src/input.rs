use std::{ rc::Rc, cell::RefCell };

use chrono::Local;
use crossterm::event::{ read, Event, KeyCode, KeyEventKind, KeyModifiers };

use crate::{
    fs::{ create::create_content, get_dir_contents },
    state::{ BottomLineContent, Panel, State, Tab, UserInput }
};

pub fn refresh_panel(state: &mut State) {
    if let Some(tab) = state.tabs.get_mut(state.current_tab) {
        if let Some(panel) = tab.panels.get_mut(tab.current_panel) {
            panel.current_dir_content = Rc::new(RefCell::new(get_dir_contents(panel.current_path.clone())));
            panel.last_updated = Local::now();
            state.bottom_line_content = BottomLineContent::RefreshedAt;
        }
    }
}

pub fn process_input(state: &mut State) -> bool {
    match read() {
        Ok(Event::Key(key_event)) => {
            let ctrl = key_event.modifiers == KeyModifiers::CONTROL;
            let shift = key_event.modifiers == KeyModifiers::SHIFT;

            let current_tab = state.tabs.get_mut(state.current_tab).unwrap();
            let current_panel = current_tab.panels.get_mut(current_tab.current_panel).unwrap();

            if key_event.kind == KeyEventKind::Press {
                {
                    let user_input = &mut state.user_input;

                    match user_input {
                        UserInput::None => {}

                        UserInput::NewDirectory(name, pos) => {
                            match key_event.code {
                                KeyCode::Char(c) => {
                                    if c == 'q' && ctrl {
                                        return true;
                                    }
                                    name.insert(*pos as usize, c);
                                    *pos += 1;
                                }

                                KeyCode::Backspace => {
                                    if *pos > 0 {
                                        *pos -= 1;
                                        name.remove(*pos as usize);
                                    }
                                }

                                KeyCode::Delete => {
                                    if *pos < name.len() as u16 {
                                        name.remove(*pos as usize);
                                    }
                                }

                                KeyCode::Left => {
                                    if *pos > 0 {
                                        *pos -= 1;
                                    }
                                }

                                KeyCode::Right => {
                                    if (*pos as usize) < name.len() {
                                        *pos += 1;
                                    }
                                }

                                KeyCode::Home => { *pos = 0; }
                                KeyCode::End => { *pos = name.len() as u16; }
                                KeyCode::Esc => { state.user_input = UserInput::None; }

                                KeyCode::Enter => {
                                    if let Err(err_string) = create_content(current_panel.current_path.clone(), name.clone()) {
                                        state.user_input = UserInput::Error(err_string);
                                    } else {
                                        state.user_input = UserInput::None;
                                    }

                                    refresh_panel(state);
                                }

                                _ => {}
                            }

                            return false;
                        }

                        UserInput::Error(_err_str) => {
                            if let KeyCode::Enter = key_event.code {
                                state.user_input = UserInput::None;
                            }
                        }
                    }
                }

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

                    KeyCode::F(5) => {
                        if Local::now().timestamp_millis() - current_panel.last_updated.timestamp_millis() > 250 {
                            refresh_panel(state);
                        }
                    }

                    KeyCode::Char('q') => {
                        if ctrl {
                            return true;
                        }
                    }

                    KeyCode::Char('j') | KeyCode::Down => {
                        let files_len = current_panel.current_dir_content.borrow().files.len();

                        if files_len > 0 && current_panel.row < files_len as u32 - 1 {
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
                            let entry = current_panel.current_dir_content.borrow().files[current_panel.row as usize].clone();
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
                            let entry = current_panel.current_dir_content.borrow().files[current_panel.row as usize].clone();
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

                    KeyCode::Char('n') => {
                        if ctrl {
                            state.user_input = UserInput::NewDirectory(String::default(), 0);
                        }
                    }

                    KeyCode::Char('m') => {
                        if let Some(file) = current_panel.current_dir_content.borrow_mut().files.get_mut(current_panel.row as usize) {
                            file.set_marked(!file.is_marked());
                        }
                    }

                    KeyCode::Tab => {
                        state.current_tab = (state.current_tab + 1) % state.tabs.len();
                    }

                    KeyCode::Enter => {
                        let entry = current_panel.current_dir_content.borrow().files[current_panel.row as usize].clone();

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

