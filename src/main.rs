use std::{ env, fs, path::Path, process };
use regex::Regex;
mod find;
pub use crate::find::find as _find;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("error using");
        process::exit(1);
    }

    let pattern = &args[2];
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("wrong regex '{}': {} ", pattern, err);
            process::exit(1);
        }
    };

    let get = _find::find(&args[1], &regex);
}
