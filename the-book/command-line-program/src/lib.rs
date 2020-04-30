use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let lines = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            case_sensitive: case_sensitive,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut ret = vec![];
    // Iterating through lines
    for line in contents.lines() {
        if line.contains(query) {
            ret.push(line);
        }
    }
    ret
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut ret = vec![];
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            ret.push(line)
        }
    }
    ret
}

// Some tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "ruSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

    #[test]
    fn case_sensitive() {
let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct me.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
