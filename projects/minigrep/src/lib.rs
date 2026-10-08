//! Library part of minigrep: all logic lives here so it can be unit- and
//! integration-tested; `main.rs` only handles the process boundary.

use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
    pub line_numbers: bool,
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingQuery,
    MissingPath,
    UnknownFlag(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingQuery => write!(f, "missing query"),
            ConfigError::MissingPath => write!(f, "missing file path"),
            ConfigError::UnknownFlag(s) => write!(f, "unknown flag '{s}'"),
        }
    }
}

impl Error for ConfigError {}

impl Config {
    /// Parses `<query> <path> [-i] [-n]` (flags may appear anywhere).
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, ConfigError> {
        let mut positional = Vec::new();
        let (mut ignore_case, mut line_numbers) = (false, false);
        for arg in args {
            match arg.as_str() {
                "-i" | "--ignore-case" => ignore_case = true,
                "-n" | "--line-number" => line_numbers = true,
                flag if flag.starts_with('-') => return Err(ConfigError::UnknownFlag(arg)),
                _ => positional.push(arg),
            }
        }
        let mut positional = positional.into_iter();
        let query = positional.next().ok_or(ConfigError::MissingQuery)?;
        let path = positional.next().ok_or(ConfigError::MissingPath)?;
        Ok(Config { query, path, ignore_case, line_numbers })
    }
}

/// A matching line with its 1-based line number.
#[derive(Debug, PartialEq)]
pub struct Hit<'a> {
    pub number: usize,
    pub line: &'a str,
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<Hit<'a>> {
    let needle = if ignore_case { query.to_lowercase() } else { query.to_string() };
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            if ignore_case {
                line.to_lowercase().contains(&needle)
            } else {
                line.contains(&needle)
            }
        })
        .map(|(i, line)| Hit { number: i + 1, line })
        .collect()
}

pub fn run(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.path)?;
    Ok(search(&config.query, &contents, config.ignore_case)
        .into_iter()
        .map(|h| if config.line_numbers { format!("{}:{}", h.number, h.line) } else { h.line.to_string() })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

    #[test]
    fn case_sensitive() {
        let hits = search("duct", TEXT, false);
        assert_eq!(hits, vec![Hit { number: 2, line: "safe, fast, productive." }]);
    }

    #[test]
    fn case_insensitive() {
        let hits = search("rUsT", TEXT, true);
        assert_eq!(hits, vec![Hit { number: 1, line: "Rust:" }]);
    }

    #[test]
    fn config_parsing() {
        let args = ["-n", "foo", "file.txt", "-i"].iter().map(|s| s.to_string());
        let c = Config::build(args).unwrap();
        assert_eq!(c, Config { query: "foo".into(), path: "file.txt".into(), ignore_case: true, line_numbers: true });
    }

    #[test]
    fn config_errors() {
        let none = std::iter::empty::<String>();
        assert_eq!(Config::build(none), Err(ConfigError::MissingQuery));
        let one = ["q"].iter().map(|s| s.to_string());
        assert_eq!(Config::build(one), Err(ConfigError::MissingPath));
        let bad = ["--wat"].iter().map(|s| s.to_string());
        assert_eq!(Config::build(bad), Err(ConfigError::UnknownFlag("--wat".into())));
    }
}
