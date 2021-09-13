use std::{env, process};
use minigrep::Config;

fn main() {
    // Vec<String> annotated so collect knows what type to collect as
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    match minigrep::run(config) {
        Ok(run) => run,
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}

