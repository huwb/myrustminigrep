extern crate myrustminigrep;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = myrustminigrep::args::Config::new(&args).unwrap_or_else(|err| {
        println!("Error occurred while reading config: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = myrustminigrep::run(&config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
