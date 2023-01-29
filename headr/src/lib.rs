use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::string::String;

type ProgResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Config {
    #[clap(default_value = "-")]
    files: Vec<String>,
    /// Number of lines to print
    #[clap(short = 'n', long, default_value_t = 10, value_parser=parse_line_count)]
    lines: usize,
    /// Number of bytes to print
    #[clap(short = 'c', long, conflicts_with = "lines", value_parser=parse_byte_count)]
    bytes: Option<usize>,
}

pub fn get_args() -> ProgResult<Config> {
    let config = Config::parse();
    Ok(config)
}

fn parse_line_count(val: &str) -> Result<usize, String> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(format!("illegal line count -- {}", val)),
    }
}

fn parse_byte_count(val: &str) -> Result<usize, String> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(format!("illegal byte count -- {}", val)),
    }
}
// #[test]
// fn test_parse_positive_int() {
//     // 3 is an OK integer
//     let res = parse_positive_int("3");
//     assert!(res.is_ok());
//     assert_eq!(res.unwrap(), 3);
//
//     // Any string is an error
//     let res = parse_positive_int("foo");
//     assert!(res.is_err());
//     assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
//
//     // A zero is an error
//     let res = parse_positive_int("0");
//     assert!(res.is_err());
//     assert_eq!(res.unwrap_err().to_string(), "0".to_string());
// }

fn open(filename: &str) -> ProgResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> ProgResult<()> {
    let n_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if n_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    )
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear()
                    }
                }
            }
        }
    }
    Ok(())
}
