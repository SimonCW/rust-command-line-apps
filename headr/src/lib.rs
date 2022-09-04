use clap::{App, Arg};
use std::error::Error;

type ProgResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> ProgResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Simon Weiß")
        .about("Head, implemented in Rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .long("lines")
                .short("n")
                .takes_value(true)
                .help("Number of lines to print")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .long("bytes")
                .short("c")
                .takes_value(true)
                .help("Number of bytes to print")
                .conflicts_with("lines"),
        )
        .get_matches();
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.value_of("lines").unwrap(),
        bytes: matches.value_of("bytes"),
    })
}

pub fn run(config: Config) -> ProgResult<()> {
    println!("{:#?}", config);
    Ok(())
}
