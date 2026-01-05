const HIDDEN_FLAG: u8 = 0b00000001;
const DIRECTORY_FLAG: u8 = 0b00000010;
const MARKED_FLAG: u8 = 0b00000100;

#[derive(Clone, Default)]
pub struct DirectoryEntry {
    pub name: String,
    flags: u8,
}

impl DirectoryEntry {
    pub fn new(name: String) -> Self {
        Self { name, flags: 0 }
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        if hidden {
            self.flags |= HIDDEN_FLAG;
        } else {
            self.flags &= !HIDDEN_FLAG;
        }
    }

    pub fn set_directory(&mut self, is_dir: bool) {
        if is_dir {
            self.flags |= DIRECTORY_FLAG;
        } else {
            self.flags &= !DIRECTORY_FLAG;
        }
    }

    pub fn set_marked(&mut self, is_marked: bool) {
        if is_marked {
            self.flags |= MARKED_FLAG;
        } else {
            self.flags &= !MARKED_FLAG;
        }
    }

    pub fn is_hidden(&self) -> bool { self.flags & HIDDEN_FLAG > 0 }
    pub fn is_directory(&self) -> bool { self.flags & DIRECTORY_FLAG > 0 }
    pub fn is_marked(&self) -> bool { self.flags & MARKED_FLAG > 0 }
}

#[derive(Clone, Default)]
pub struct DirectoryContents {
    pub path: String,
    pub total_len: usize,
    pub files: Vec<DirectoryEntry>,
}

#[test]
pub fn test_directory_entry() {
    let test_dir_entry = DirectoryEntry::new("test".to_owned());

    assert!(test_dir_entry.name == "test");
    assert!(!test_dir_entry.is_hidden());
    assert!(!test_dir_entry.is_directory());
}

