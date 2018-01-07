extern crate myrustminigrep;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = myrustminigrep::args::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error occurred while reading config: {}", err);
        process::exit(1);
    });

    if let Err(e) = myrustminigrep::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
