use crate::error;
use clap::error::ErrorKind;
use std::fs::DirEntry;
use colored::*;
use std::fs;
use chrono::{ DateTime, Utc };
pub fn error_output(e: error::MyError) -> ! {
    let mut cmd = clap::Command::new("myfind [OPTIONS] <DIR> <REGEX>");
    let err = cmd.error(ErrorKind::InvalidValue, e.message);
    err.exit()
}

pub fn result_output(matches: &Vec<DirEntry>, verbose: bool) {
    if !verbose {
        for entry in matches {
            let path = entry.path().to_path_buf();
            let removed_filename = path.parent().unwrap();
            let filename = path.file_name().unwrap();
            let removed_filename = removed_filename.to_string_lossy().to_string();
            let removed_filename_str = removed_filename.as_str();
            let divide = if cfg!(windows) { "\\" } else { "/" };
            let filename = filename.to_string_lossy().to_string();
            let filename_str = filename.as_str();
            println!("{}{}{}", removed_filename_str.blue(), divide, filename_str.green());
        }
    } else {
        let count = matches.len();
        let forprint = format!("Found {} matches", count);
        let forprint = forprint.as_str();
        println!("{}", forprint.green());
        println!("-----------------");
        println!(
            "{:<15} {:<15} {:<30}",
            "LastAccessed".green(),
            "LastModified".green(),
            "Path".green()
        );
        for entry in matches {
            let path = entry.path().to_path_buf();
            let metadata = fs::metadata(&path).unwrap();
            let access = metadata.accessed().unwrap();
            let access: DateTime<Utc> = access.into();
            let access = access.format("%Y-%m-%d").to_string();
            let modified = metadata.modified().unwrap();
            let modified: DateTime<Utc> = modified.into();
            let modified = modified.format("%Y-%m-%d").to_string();
            let path = entry.path().to_path_buf();
            let removed_filename = path.parent().unwrap();
            let filename = path.file_name().unwrap();
            let removed_filename = removed_filename.to_string_lossy().to_string();
            let removed_filename_str = removed_filename.as_str();
            let divide = if cfg!(windows) { "\\" } else { "/" };
            let filename = filename.to_string_lossy().to_string();
            let filename_str = filename.as_str();
            println!(
                "{:<15} {:<15} {}{}{}",
                access,
                modified,
                removed_filename_str.blue(),
                divide,
                filename_str.green()
            );
        }
    }
}
