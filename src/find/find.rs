use std::{ env, fs::{self, DirEntry}, path::Path, process , io};
use regex::Regex;
enum SearchType {
    Any,
    File,
    Dir,
}
pub enum OrderType {
    Path,
    Length,
    Access,
}

pub fn find<P: AsRef<Path>>(
    root: P,
    regex: &Regex
) -> Result<Vec<DirEntry>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, &mut matches)?;
    sort_entry(OrderType::Access, &mut matches);
    for entry in &matches {
        println!("{}", entry.path().display());
    }
    Ok(matches)
}

pub fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches: &mut Vec<DirEntry>
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, matches)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(entry)
                }
            }
        }
    }
    Ok(())
}

//sort direntry by access time
pub fn sort_entry(order: OrderType, matches: &mut Vec<DirEntry>) {
    match order {
        OrderType::Path => matches.sort_by(|a, b| a.path().cmp(&b.path())),
        // OrderType::Length => matches.sort_by(|a, b| a.path().len().cmp(&b.path().len())),
        _ => matches.sort_by(|a, b| b.metadata().unwrap().accessed().unwrap().cmp(&a.metadata().unwrap().accessed().unwrap())),
    }
}