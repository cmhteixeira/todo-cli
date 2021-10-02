
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};




pub fn read_user_state() -> Result<Option<String>, String> {
    let home_dir: PathBuf = std::env::home_dir().unwrap().join(Path::new(".todo-cli"));

    if home_dir.exists() {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(home_dir.as_path()).map_err(|error| error.to_string())?;
        let mut temp_string = String::new();
        match file.read_to_string(&mut temp_string) {
            Ok(_) => Ok(Some(temp_string)),
            Err(error) => Err(error.to_string())
        }
    } else {
        Ok(None)
    }
}


pub fn persist_user_state(payload: &str) -> Result<(), String> {
    let home_dir: PathBuf = std::env::home_dir().unwrap().join(Path::new(".todo-cli"));

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(home_dir.as_path())
        .unwrap();

    file.write(payload.as_bytes()).map(|_size| ()).map_err(|error| error.to_string())
}

