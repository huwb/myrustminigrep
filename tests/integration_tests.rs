extern crate myrustminigrep;

use std::fs::File;

#[test]
#[should_panic]
fn missing_file() {
    // dummy file name that should not exist
    let nonexistant_filename = String::from("i_dont_exist.txt");

    // only execute this test if the file does not exist!
    if let Err(_) = File::open(&nonexistant_filename) {
        let args: Vec<String> = vec![
            String::from("execname"),
            String::from("query"),
            nonexistant_filename,
        ];

        let config = myrustminigrep::args::Config::new(&args).unwrap();

        myrustminigrep::run(&config).unwrap();
    } else {
        println!(
            "Test did not run because dummy file exists on disk: {}",
            nonexistant_filename
        );
    }
}
