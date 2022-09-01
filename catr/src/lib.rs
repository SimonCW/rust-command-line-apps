use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    show_ends: bool,
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
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .long("number")
                .short("-n")
                .help("Print line numbers")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .long("number-nonblank")
                .short("-b")
                .help("Print line numbers for nonblank lines")
                .takes_value(false)
                .conflicts_with("number_lines"),
        )
        .arg(
            Arg::with_name("show_ends")
                .long("show-ends")
                .short("-E")
                .help("Show $ at the end of each line")
                .takes_value(false),
        )
        .get_matches();
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
        show_ends: matches.is_present("show_ends"),
    })
}

fn open(filename: &str) -> ProgResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> ProgResult<()> {
    let mut line_num = 0;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let (width, tab_char) = if config.number_lines | config.number_nonblank_lines {
                    (6, "\t")
                } else {
                    (0, "")
                };
                let line_end = if config.show_ends { "$" } else { "" };
                for line in file.lines() {
                    let line = line?;
                    let line_num_str = if config.number_lines
                        || (config.number_nonblank_lines && !line.is_empty())
                    {
                        line_num += 1;
                        line_num.to_string()
                    } else {
                        "".to_string()
                    };
                    if config.number_nonblank_lines && line.is_empty() {
                        println!("{}", line_end)
                    } else {
                        println!(
                            "{:>width$}{}{}{}",
                            line_num_str,
                            tab_char,
                            line,
                            line_end,
                            width = width
                        )
                    };
                    // if config.number_lines {
                    //     println!("{:>6}\t{}", line_num + 1, line)
                    // } else if config.number_nonblank_lines {
                    //     if !line.is_empty() {
                    //         last_num += 1;
                    //         println!("{:>6}\t{}", last_num, line);
                    //     } else {
                    //         println!();
                    //     }
                    // } else {
                    //     println!("{}", line);
                    // }
                }
            }
        }
    }
    Ok(()) // Return unit type in Ok variant to indicate success
}
