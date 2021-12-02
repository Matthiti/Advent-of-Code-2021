use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
struct CommandParseError {
    message: String
}

impl Error for CommandParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Display for CommandParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse command: {}", self.message)
    }
}

impl CommandParseError {
    fn new(message: String) -> Self {
        CommandParseError { message }
    }
}

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32)
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(CommandParseError::new(String::from("Too many elements in line")))
        }

        let value = parts[1].parse().map_err(|_| CommandParseError::new(format!("Second part is not an unsigned integer: {}", parts[1])))?;
        match parts[0] {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(CommandParseError::new(format!("Unknown command: {}", parts[0])))
        }
    }
}

pub mod part1 {
    use crate::days::day02::Command;
    use crate::util::input_parser;

    pub fn start() -> i32 {
        let input = input_parser::parse_file::<Command>("inputs/day02.txt").unwrap();

        let mut h_pos = 0;
        let mut d_pos = 0;
        for command in input.iter() {
            match command {
                Command::Forward(val) => h_pos += *val as i32,
                Command::Down(val) => d_pos += *val as i32,
                Command::Up(val) => d_pos -= *val as i32
            }
        }

        h_pos * d_pos
    }
}

pub mod part2 {
    use crate::days::day02::Command;
    use crate::util::input_parser;

    pub fn start() -> i32 {
        let input = input_parser::parse_file::<Command>("inputs/day02.txt").unwrap();

        let mut h_pos = 0;
        let mut d_pos = 0;
        let mut a_pos = 0;
        for command in input.iter() {
            match command {
                Command::Forward(val) => {
                    h_pos += *val as i32;
                    d_pos += *val as i32 * a_pos;
                },
                Command::Down(val) => a_pos += *val as i32,
                Command::Up(val) => a_pos -= *val as i32
            }
        }

        h_pos * d_pos
    }
}