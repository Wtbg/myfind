use regex::Regex;
use std::path::PathBuf;
use crate::find;
use crate::error;
pub fn try_parse_args(args: &crate::Args) -> Result<crate::ParsedArgs, error::MyError> {
    let dir = try_parse_dir(args)?;
    let regex = try_parse_regex(args)?;
    let sort = try_parse_sort(args)?;
    let verbose = try_parse_verbose(args);
    Ok(crate::ParsedArgs {
        dir,
        regex,
        sort,
        verbose,
    })
}

fn try_parse_dir(args: &crate::Args) -> Result<PathBuf, error::MyError> {
    let dir = PathBuf::from(&args.dir);
    if !dir.is_dir() {
        return Err(
            error::MyError::new_with_description(
                error::MyErrorKind::NotFound,
                format!("{} is not a directory", dir.display()),
                "dir".to_string()
            )
        );
    }
    Ok(dir)
}

fn try_parse_regex(args: &crate::Args) -> Result<Regex, error::MyError> {
    let regex = Regex::new(&args.regex)?;
    Ok(regex)
}

fn try_parse_sort(args: &crate::Args) -> Result<find::OrderType, error::MyError> {
    if let Some(sort) = &args.sort {
        match sort.as_str() {
            "path" => Ok(find::OrderType::Path),
            "access" => Ok(find::OrderType::Access),
            _ => {
                Err(
                    error::MyError::new_with_description(
                        error::MyErrorKind::Parse,
                        format!("Invalid sort type {}", sort),
                        "sort".to_string()
                    )
                )
            }
        }
    } else {
        Ok(find::OrderType::Path)
    }
}

fn try_parse_verbose(args: &crate::Args) -> bool {
    args.verbose
}
