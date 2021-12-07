use std::error::Error;
use std::{fmt, fs, io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError {
    message: String
}

impl ParseError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        Self::new(err.to_string())
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub fn parse_file<T: FromStr>(file: &str) -> Result<Vec<T>, ParseError> {
    read_lines(file)?
        .map(|line| match line?.parse() {
            Err(_) => Err(ParseError::new(String::from("Cannot parse value"))),
            Ok(value) => Ok(value)
        })
        .collect()
}

pub fn parse_file_raw(file: &str) -> std::io::Result<String> {
    fs::read_to_string(file)
}

pub fn parse_file_horizontal<T: FromStr>(file: &str, delimiter: &str) -> Result<Vec<T>, ParseError> {
    parse_file_raw(file)?
        .split(delimiter)
        .map(|item| match item.parse() {
            Err(_) => Err(ParseError::new(String::from("Cannot parse value"))),
            Ok(value) => Ok(value)
        })
        .collect()
}

// Source: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}