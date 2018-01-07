extern crate myrustminigrep;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = myrustminigrep::args::Config::new(&args).unwrap_or_else(|err|{
        println!("Error occurred while reading config: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    let con = myrustminigrep::file_utils::read_file(config.filename);
    println!("With contents:\n{}", con);
}
