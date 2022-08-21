use clap::{App, Arg};

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Simon Weiß <weiss.simon@outlook.com>")
        .about("Echo written in Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Text to echo")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Omit newline at the end")
                .takes_value(false),
        )
        .get_matches();
    let text = _matches.values_of_lossy("text").unwrap();
    let omit_newline = _matches.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
