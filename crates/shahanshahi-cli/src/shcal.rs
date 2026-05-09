fn main() {
    if let Err(err) = shahanshahi_cli::run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}
