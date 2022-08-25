use clap::{App, Arg};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type ProgResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> ProgResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Simon Weiss")
        .about("cat in Rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Files to cat")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("-n")
                .help("Print line numbers")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("-b")
                .help("Print line numbers for nonblank lines")
                .takes_value(false),
        )
        .get_matches();
    Ok(Config())
}

pub fn run() -> ProgResult<()> {
    println!("Hello, world!");
    Ok(()) // Return unit type in Ok variant to indicate success
}
