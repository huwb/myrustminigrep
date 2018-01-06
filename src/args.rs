pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
}

impl<'a> Config<'a> {
    fn new(query: &'a String, filename: &'a String) -> Config<'a> {
        Config { query, filename }
    }
}

pub fn parse_config(args: &Vec<String>) -> Config {
    assert_eq!(
        args.len(),
        3,
        "Two arguments expected: myrustminigrep <query> <filename>"
    );

    Config::new(&args[1], &args[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grab_args() {
        let args = vec![
            String::from("execname"),
            String::from("needle"),
            String::from("haystack.txt"),
        ];

        let config = parse_config(&args);

        assert_eq!(config.query, &args[1]);
        assert_eq!(config.filename, &args[2]);
    }

    #[test]
    #[should_panic]
    fn test_grab_args_insufficient() {
        let args = vec![String::from("execname"), String::from("needle")];

        parse_config(&args);
    }

    #[test]
    #[should_panic]
    fn test_grab_args_too_many() {
        let args = vec![
            String::from("execname"),
            String::from("needle"),
            String::from("haystack.txt"),
            String::from("overflow"),
        ];

        parse_config(&args);
    }

    #[test]
    #[should_panic]
    fn test_grab_args_none() {
        parse_config(&vec![]);
    }
}
