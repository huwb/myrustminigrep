pub mod args;
pub mod file_utils;

use std::error::Error;

pub fn run(config: &args::Config) -> Result<(), Box<Error>> {
    let con = file_utils::read_file(config.filename)?;

    let results = search(&config.query[..], &con[..])?;

    for result in results {
        println!("{}", result);
    }

    Ok({})
}

fn search<'a>(query: &'a str, contents: &'a str) -> Result<Vec<&'a str>, &'static str> {
    if query.len() == 0 {
        return Err("query is empty");
    }

    let mut result: Vec<&str> = Vec::new();

    for l in contents.lines() {
        if l.contains(query) {
            result.push(&l);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_matches() {
        let query = "nope";
        let contents = "\
No match here.
None here either.
No-pe - nothing.";
        assert_eq!(0, search(query, contents).unwrap().len());
    }

    #[test]
    fn no_matches_empty_content() {
        let query = "nope";
        let contents = "";
        assert_eq!(0, search(query, contents).unwrap().len());
    }

    #[test]
    #[should_panic]
    fn empty_query_error() {
        let query = "";
        let contents = "";
        assert_eq!(0, search(query, contents).unwrap().len());
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Safe, fast, productive."], search(query, contents).unwrap());
    }
}
