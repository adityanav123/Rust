use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // args() gives iterator to arguments passed to the program and collect() turns iterator to collection.
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments : {}", err);
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("In file {}", config.filename);

    // read contents from file
    if let Err(e) = minigrep::run(config) {
        eprintln!("application error : {}", e);
        process::exit(1);
    }
}

