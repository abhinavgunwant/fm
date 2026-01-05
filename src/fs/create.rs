use std::{ fs::{ create_dir_all, File }, io::ErrorKind, path::PathBuf };

fn process_error(kind: ErrorKind) -> Result<(), String> {
    match kind {
        ErrorKind::StorageFull => Err(String::from("Storage full!")),
        ErrorKind::AlreadyExists => Err(String::from("A directory with same name already exists!")),
        ErrorKind::QuotaExceeded => Err(String::from("Filesystem quota exceeded!")),
        ErrorKind::PermissionDenied => Err(String::from("Permission denied by the system.")),

        _ => Err(String::from("Some unknown error occured!")),
    }
}

pub fn create_content(path_buf: PathBuf, name: String) -> Result<(), String> {
    let mut p = path_buf;
    p.push(&name);

    if name.ends_with("/") {
        return match create_dir_all(p) {
            Ok(()) => Ok(()),
            Err(e) => process_error(e.kind()),
        };
    }

    match File::create(p) {
        Ok(_) => Ok(()),
        Err(e) => process_error(e.kind()),
    }
}

