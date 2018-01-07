extern crate myrustminigrep;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = myrustminigrep::args::Config::new(&args);

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    let con = myrustminigrep::file_utils::read_file(config.filename);
    println!("With contents:\n{}", con);
}
