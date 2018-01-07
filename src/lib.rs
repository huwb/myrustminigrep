pub mod args;
pub mod file_utils;

use std::error::Error;

pub fn run(config: &args::Config) -> Result<(), Box<Error>> {

    let con = file_utils::read_file(config.filename)?;

    println!("With contents:\n{}", con);

    Ok({})
}
