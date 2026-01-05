//! This module deals with the file system.

pub mod dir_contents;
pub mod create;

use std::path::PathBuf;
use crate::fs::dir_contents::{ DirectoryEntry, DirectoryContents };

pub fn get_dir_contents(p: PathBuf) -> DirectoryContents {
    let path_str: &str = match p.to_str() {
        Some(pathstr) => pathstr,
        None => "",
    };

    let mut content = DirectoryContents {
        path: path_str.to_owned(),
        files: vec![],
        total_len: 0,
    };

    let mut directories: Vec<DirectoryEntry> = Vec::with_capacity(32);
    let mut files: Vec<DirectoryEntry> = Vec::with_capacity(32);

    match p.read_dir() {
        Ok(dir_entry) => {
            for entry in dir_entry {
                match entry {
                    Ok(e) => {
                        if let Ok(file_type) = e.file_type() {
                            if let Ok(file_name) = e.file_name().into_string() {
                                let mut d_entry = DirectoryEntry::new(file_name.clone());

                                if file_type.is_dir() {
                                    d_entry.set_directory(true);

                                    directories.push(d_entry);
                                } else {
                                    files.push(d_entry);
                                }

                                content.total_len += 1;
                            }
                        }
                    }

                    Err(_e) => {}
                }
            }
        }

        Err(_e) => {}
    }

    directories.append(&mut files);

    content.files = directories;

    content
}

#[test]
pub fn test_get_dir_contents() {
    let path_buf = PathBuf::from("C:\\Users\\abhig\\Projects\\3D");
    let dc = get_dir_contents(path_buf);

    use std::rc::Rc;
    let rc_dc: Rc<DirectoryContents> = Rc::new(dc);

    for (i, content) in rc_dc.files.iter().enumerate() {
        println!("{} {}", i, content.name);
    }

    println!("test!!!!");
    println!("rc_dc.total_len: {}", rc_dc.total_len);

    assert!(rc_dc.total_len == 3);
    assert!(!rc_dc.files.is_empty());
    assert!(rc_dc.files.len() == 4);
    assert!(rc_dc.files[0].name == "chair.blend");
}

