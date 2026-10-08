// Integration test: uses only the PUBLIC API of the library, like a user would.

use minigrep::{run, Config};
use std::fs;

#[test]
fn searches_a_real_file() {
    let path = std::env::temp_dir().join("minigrep_it.txt");
    fs::write(&path, "alpha\nBeta\ngamma\nbeta again\n").unwrap();

    let config = Config {
        query: "beta".into(),
        path: path.to_string_lossy().into_owned(),
        ignore_case: true,
        line_numbers: true,
    };
    assert_eq!(run(&config).unwrap(), vec!["2:Beta", "4:beta again"]);

    fs::remove_file(path).unwrap();
}

#[test]
fn missing_file_is_an_error() {
    let config = Config {
        query: "x".into(),
        path: "definitely/not/here.txt".into(),
        ignore_case: false,
        line_numbers: false,
    };
    assert!(run(&config).is_err());
}
