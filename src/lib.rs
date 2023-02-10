use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Id";
        let contents = "\
Lorem ipsum dolor sit amet.
Id adipisci harum aut vero dolorem
vel consequatur veniam aut quis
cupiditate et maxime repellat.
LOREM IPSuM DOLOR SIt AMET.";
        let result = vec!["Id adipisci harum aut vero dolorem"];
        assert_eq!(result, search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "doLor";
        let contents = "\
Lorem ipsum dolor sit amet.
Id adipisci harum aut vero dolorem
vel consequatur veniam aut quis
cupiditate et maxime repellat.
LOREM IPSuM DOLOR SIt AMET.";
        let result = vec![
            "Lorem ipsum dolor sit amet.",
            "Id adipisci harum aut vero dolorem",
            "LOREM IPSuM DOLOR SIt AMET.",
        ];
        assert_eq!(result, search_case_insensitive(query, contents));
    }
}
