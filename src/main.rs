extern crate myrustminigrep;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = myrustminigrep::grab_args(&args);

    println!("Searching for: {}", query);
    println!("In file: {}", filename);
}
