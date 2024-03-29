use std::fs;
use std::error::Error;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub const HELP: &str = "usage: $> minigrep <search pattern> <target file path>";

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|l| l.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|l| l.to_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(val) => val,
            None => return Err("Did not get a query string"),
        };
        let filename = match args.next() {
            Some(val) => val,
            None => return Err("Did not get a filename"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // returns false if 
        // env variable not set, true otherwise, whatever the stored value
            
        Ok(Config { query, filename, case_sensitive })
    }
}