use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Din't get a query string"),
        };

        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("Didn't get a filepath"),
        };

        let ignore_case_flag = Self::parse_flags(args)?;

        let ignore_case = env::var("IGNORE_CASE").map_or_else(
            |_| ignore_case_flag,
            |v| {
                v.parse::<bool>().expect(&format!(
                    "Invalid environmental variable value, expected <bool> got \"{v}\""
                ))
            },
        );

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    fn parse_flags(flags: impl Iterator<Item = String>) -> Result<bool, &'static str> {
        let mut ignore_case_flag = false;

        for flag in flags {
            if flag == "--ignore_case" {
                ignore_case_flag = true;
            } else {
                return Err("unknown flag");
            }
        }

        Ok(ignore_case_flag)
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
