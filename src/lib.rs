pub fn grab_args(args: &Vec<String>) -> (&String, &String) {
    assert_eq!(
        args.len(),
        3,
        "Two arguments expected: myrustminigrep <query> <filename>"
    );

    (&args[1], &args[2])
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

        let (query, filename) = grab_args(&args);

        assert_eq!(query, &args[1]);
        assert_eq!(filename, &args[2]);
    }

    #[test]
    #[should_panic]
    fn test_grab_args_insufficient() {
        let args = vec![String::from("execname"), String::from("needle")];

        grab_args(&args);
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

        grab_args(&args);
    }

    #[test]
    #[should_panic]
    fn test_grab_args_none() {
        grab_args(&vec![]);
    }
}
