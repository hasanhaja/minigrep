use std::{env, process};

use minigrep::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file {}", config.filename);

    // We're not doing unwrap or else, because we will get () which is useless. We only need to the Result for it's side effects, so we use if let to ensure we catch and handle the error and do the side effect otherwise while ignoring the result of ().
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}
