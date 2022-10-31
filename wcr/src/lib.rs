use clap::Parser;
use std::error::Error;

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

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
