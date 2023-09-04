use std::{ fs::{ self, DirEntry }, path::Path };
use regex::Regex;
use crate::{ ParsedArgs, error };

#[derive(Debug)]
pub enum OrderType {
    Path,
    Access,
}

pub fn find(parsed_args: &ParsedArgs) -> Result<Vec<DirEntry>, error::MyError> {
    let mut matches = Vec::new();
    let regex = &parsed_args.regex;
    let dir = &parsed_args.dir;
    walk_tree(dir, regex, &mut matches)?;
    let order = &parsed_args.sort;
    sort_entry(order, &mut matches);
    Ok(matches)
}

pub fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches: &mut Vec<DirEntry>
) -> Result<(), error::MyError> {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(dir.to_path_buf());
    while let Some(entry) = queue.pop_front() {
        if entry.is_dir() {
            for entry in fs::read_dir(entry)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    queue.push_back(path.clone());
                } else if let Some(file_name) = path.file_name() {
                    if regex.is_match(file_name.to_string_lossy().as_ref()) {
                        matches.push(entry);
                    }
                }
            }
        }
    }
    Ok(())
}

//sort direntry by access time
pub fn sort_entry(order: &OrderType, matches: &mut [DirEntry]) {
    match order {
        OrderType::Path => (),
        OrderType::Access =>
            matches.sort_by_key(|b| std::cmp::Reverse(b.metadata().unwrap().accessed().unwrap())),
    }
}
