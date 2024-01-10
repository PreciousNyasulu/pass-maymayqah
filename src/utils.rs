use std::fs::{self, File};
use std::io::Write;

pub fn folder_exists(folder_path: &str) -> bool {
    fs::metadata(folder_path).is_ok()
}

pub fn create_program_folder(folder_path: &str) -> bool {
    if !folder_exists(folder_path) {
        match fs::create_dir_all(folder_path) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Error creating folder: {}", e);
                false
            }
        }
    }else {
        true
    }
}

pub fn create_password_file(file_path:&str) -> bool{
    match File::create(&file_path) {
        Ok(mut file) => {
            // Write content to the file
            if let Err(e) = file.write_all("".as_bytes()) {
                eprintln!("Error writing to file: {}", e);
                return false;
            }
            true
        }
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            false
        }
    }
}