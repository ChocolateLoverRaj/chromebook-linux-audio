pub const EXTENSION_NAME: &str = "chromebook-audio";

pub fn get_extension_dir() -> String {
    format!("/var/lib/extensions/{EXTENSION_NAME}")
}

use std::{
    fs::{create_dir, create_dir_all, metadata, read_dir},
    os::unix::fs::symlink,
};

use walkdir::WalkDir;
pub fn recursive_symlink(original: &str, link: &str) {
    for file in WalkDir::new(original) {
        let file = file.unwrap();
        let file_name = file.file_name().to_str().unwrap();
        let file_path = file.path().to_str().unwrap();
        if file.file_type().is_dir() {
            if file_path == original {
                create_dir_all(link).unwrap();
            } else {
                println!("{}", format!("{link}/{file_name} {file_name}"));
                create_dir_all(format!("{link}/{file_name}")).unwrap();
            }
        } else if file.file_type().is_file() {
            println!("{}", format!("{link}/{file_name}"));
            symlink(file_path, format!("{link}/{file_name}")).unwrap();
        }
    }
}
