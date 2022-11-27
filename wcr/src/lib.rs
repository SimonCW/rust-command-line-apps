use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct Config {
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// print the newline counts
    #[arg(long, short)]
    lines: bool,

    /// print the word counts
    #[arg(long, short)]
    words: bool,

    /// print the byte counts
    #[arg(long, short = 'c')]
    bytes: bool,

    /// print the character counts
    #[arg(long, short = 'm')]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    if [config.lines, config.words, config.bytes, config.chars]
        .iter()
        .all(|&v| v == false)
    {
        Ok(Config {
            lines: true,
            words: true,
            bytes: true,
            ..config
        })
    } else {
        Ok(config)
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        num_bytes += bytes;
        num_lines += 1;
        num_chars += line.chars().count();
        num_words += line.split_whitespace().count();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let file_info = count(file)?;
                //ToDo:
                // Printing is kinda weird in GNU wc, see:
                // https://www.gnu.org/software/coreutils/manual/html_node/wc-invocation.html#wc-invocation
                println!(
                    "{:>2}{:>3}{:>3} {}",
                    //"{:>3}{:>4}{:>4} {}",
                    file_info.num_lines,
                    file_info.num_words,
                    file_info.num_bytes,
                    filename
                )
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
