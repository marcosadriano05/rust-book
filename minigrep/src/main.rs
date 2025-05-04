use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Process failed: {err}");
        process::exit(1);
    });

    if let Err(msg) = minigrep::run(config) {
        eprintln!("Application error: {msg}");
        process::exit(1);
    }
}
