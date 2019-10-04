use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_argument_config() {
        let config = Config::new(&[]);

        if let Err(e) = config {
            assert_eq!(e, "not enough arguments");
        }
    }

    #[test]
    fn one_argument_config() {
        let config = Config::new(&[String::from("hello")]);

        if let Err(e) = config {
            assert_eq!(e, "not enough arguments");
        }
    }

    // #[test]
    // fn run_no_file() {
    //     if let Err(e) = run(Config {
    //         query: String::from("needle"),
    //         filename: String::from("haystack.txt"),
    //     }) {
    //         assert_eq!(e, "no file or process");
    //     }
    // }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust;
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// Box<dyn Error> is a trait object - The function returns a type
// that will implement the Error trait
// dyn is short for dynamic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(()) // Success type is the unit type ().
           //Indicates that we are calling run only for it's side effects
}
