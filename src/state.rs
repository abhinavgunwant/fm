use std::{ path::PathBuf, rc::Rc };
use crate::fs::dir_contents::DirectoryContents;

pub struct Panel {
    pub current_path: PathBuf,
    pub row: u32,

    // This is the index of the item at the top
    pub list_start_index: u32,
    pub current_dir_content: Rc<DirectoryContents>,
}

pub struct Tab {
    pub panels: Vec<Panel>,
    pub current_panel: usize,
}

pub struct State {
    pub show_help_menu: bool,
    pub current_tab: usize,
    pub tabs: Vec<Tab>,
}

impl Panel {
    pub fn new() -> Self {
        let current_path: PathBuf = match dirs_next::home_dir() {
            Some(home) => if home.exists() {
                home
            } else {
                PathBuf::default()
            },
            None => PathBuf::default(),
        };

        Self {
            current_path: current_path.clone(),
            row: 0,
            list_start_index: 0,
            current_dir_content: Rc::new(DirectoryContents::default()),
        }
    }

    pub fn new_with_path(path: PathBuf) -> Self {
        Self {
            current_path: path.clone(),
            row: 0,
            list_start_index: 0,
            current_dir_content: Rc::new(DirectoryContents::default())
        }
    }
}

impl Tab {
    pub fn new() -> Self {
        let mut panels: Vec<Panel> = Vec::with_capacity(2);
        panels.push(Panel::new());

        Self { panels, current_panel: 0 }
    }

    pub fn new_with_path(path: PathBuf) -> Self {
        let mut panels: Vec<Panel> = Vec::with_capacity(2);
        panels.push(Panel::new_with_path(path));

        Self { panels, current_panel: 0 }
    }
}

impl State {
    pub fn new() -> Self {
        let mut tabs: Vec<Tab> = Vec::with_capacity(2);

        tabs.push(Tab::new());

        Self {
            show_help_menu: false,
            current_tab: 0,
            tabs,
        }
    }
}

