use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.input, &contents)
    } else {
        search(&config.input, &contents)
    };

    for line in results{
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub input: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config{
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let input = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input string"),
        }



        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {input, filename, ignore_case})
    }
}

pub fn search<'a>(input: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(input))
        .collect()
}

pub fn search_case_insensitive<'a>(input: &str, contents: &'a str) -> Vec<&'a str> {
    let input = input.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&input){
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn case_sensitive() {
        let input = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Duct tape

        ";

        assert_eq!(vec!["Safe, fast, productive."], search(input, contents));
    }

    #[test]
    fn case_insensitive() {
        let input = "rUsT";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Trust me.

        ";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(input, contents));
    }
}
