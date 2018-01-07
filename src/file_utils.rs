use std::fs::File;
use std::io::prelude::*;

use std::error::Error;

pub fn read_file(file_name: &String) -> Result<String, Box<Error>> {
    let mut f = File::open(file_name)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}
