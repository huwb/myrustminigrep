pub mod args;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn run(config: &args::Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = search(&config.query, &contents)?;

    for result in results {
        println!("{}", result);
    }

    Ok({})
}

fn search<'a>(query: &'a str, contents: &'a str) -> Result<Vec<&'a str>, &'static str> {
    if query.len() == 0 {
        return Err("query is empty");
    }

    Ok(
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect(),
    )
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

        assert_eq!(
            vec!["Safe, fast, productive."],
            search(query, contents).unwrap()
        );
    }

    #[test]
    fn two_results() {
        let query = "hen";
        let contents = "\
Then hen was returned once
HENs was not
chickens was not
only the two hens
";
        assert_eq!(
            vec!["Then hen was returned once", "only the two hens"],
            search(query, contents).unwrap()
        );
    }
}
