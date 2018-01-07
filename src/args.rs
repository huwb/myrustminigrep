pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Config<'a> {
        assert_eq!(
            args.len(),
            3,
            "Two arguments expected: myrustminigrep <query> <filename>"
        );

        let query = &args[1];
        let filename = &args[2];

        Config { query, filename }
    }
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

        let config = Config::new(&args);

        assert_eq!(config.query, &args[1]);
        assert_eq!(config.filename, &args[2]);
    }

    #[test]
    #[should_panic]
    fn test_grab_args_insufficient() {
        let args = vec![String::from("execname"), String::from("needle")];

        Config::new(&args);
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

        Config::new(&args);
    }

    #[test]
    #[should_panic]
    fn test_grab_args_none() {
        Config::new(&vec![]);
    }
}
