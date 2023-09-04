use regex::Regex;
use std::path::PathBuf;
use clap::Parser ;
use tracing::{ info, Level, error };
use tracing_subscriber::FmtSubscriber;
mod error;
mod find;
mod output;
mod parse;
#[derive(Parser, Debug)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
pub struct Args {
    ///Directory to search
    dir: String,
    ///Regex to match
    regex: String,
    ///Sort by path or access time, available options: path, access
    #[clap(short, long)]
    sort: Option<String>,
    ///Verbose output
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Debug)]
pub struct ParsedArgs {
    pub dir: PathBuf,
    pub regex: Regex,
    pub sort: find::OrderType,
    pub verbose: bool,
}
fn main() {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::ERROR).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let args = Args::parse();
    let parse = match parse::try_parse_args(&args) {
        Ok(parsed_args) => {
            info!("Parsed args success");
            parsed_args
        }
        Err(e) => {
            info!("Parsed args failed");
            output::error_output(e)
        }
    };
    let find_result = match find::find(&parse) {
        Ok(find_result) => {
            info!("Find result success");
            find_result
        }
        Err(e) => {
            info!("Find result failed");
            output::error_output(e)
        }
    };
    output::result_output(&find_result, parse.verbose);
}
