use std::error::Error;
use std::{fs, env};

const IGNORE_CASE_KEY: &'static str = "IGNORE_CASE";
const IGNORE_CASE_CMD_KEY: &'static str = "no-case";

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

fn args_has_no_case(fourth_arg: String) -> bool {
    let fourth_arg: Vec<&str> = fourth_arg.split("=").collect();
    
    if fourth_arg.len() != 2 {
        return false;
    }

    fourth_arg[0] == IGNORE_CASE_CMD_KEY && fourth_arg[1] == "1"
    
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); // ignore the name of the program;
        let (total_cmd_args, _) = args.size_hint();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file path")
        };

        let ignore_case =  total_cmd_args < 4 || match args.next() {
            Some(arg) => args_has_no_case(arg) || env::var(IGNORE_CASE_KEY).is_ok(),
            None => false
        };
        
        return Ok(
            Self { query, file_path, ignore_case }
        )
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .filter(|line| line.to_lowercase().contains(query))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, test, productive.
Pick four
Duct tape.
        ";
        assert_eq!(vec!["safe, test, productive."], search(query, contents))
    }
   
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, test, productive.
Pick four.
Trust ME.
        ";
        assert_eq!(vec!["Rust:", "Trust ME."], search_case_insensitive(query, contents))
    }
    

}