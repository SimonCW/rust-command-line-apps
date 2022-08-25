fn main() {
    if let Err(e) = catr::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
