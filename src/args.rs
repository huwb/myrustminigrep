pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Config<'a>, &'static str> {
        match args.len() {
            3 => Ok(Config {
                query: &args[1],
                filename: &args[2],
            }),
            i if i < 3 => return Err("not enough arguments"),
            _ => return Err("too many arguments"),
        }
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

        let config = Config::new(&args).unwrap();

        assert_eq!(config.query, &args[1], "query config incorrect");
        assert_eq!(config.filename, &args[2], "filename config incorrect");
    }

    #[test]
    #[should_panic]
    fn test_grab_args_insufficient() {
        let args = vec![String::from("execname"), String::from("needle")];

        Config::new(&args).unwrap();
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

        Config::new(&args).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_grab_args_none() {
        Config::new(&vec![]).unwrap();
    }
}
