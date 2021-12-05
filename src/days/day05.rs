use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinate {
    x: u32,
    y: u32
}

impl Coordinate {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',')
            .map(|(x, y)| (x.trim().parse().unwrap(), y.trim().parse().unwrap()))
            .unwrap();
        Ok(Coordinate::new(x, y))
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Line {
    from: Coordinate,
    to: Coordinate
}

impl Line {
    pub fn new(from: Coordinate, to: Coordinate) -> Self {
        Self { from, to }
    }

    pub fn crossing_points(&self) -> Vec<Coordinate> {
        let x_diff = self.from.x as i32 - self.to.x as i32;

        if x_diff == 0 {
            let (y_min, y_max) = if self.from.y < self.to.y {
                (self.from.y, self.to.y)
            } else {
                (self.to.y, self.from.y)
            };
            return (y_min..=y_max).map(|y| Coordinate::new(self.from.x, y)).collect();
        }
        let y_diff = self.to.y as i32 - self.from.y as i32;
        let (x_min, x_max) = if self.from.x < self.to.x {
            (self.from.x, self.to.x)
        } else {
            (self.to.x, self.from.x)
        };

        if y_diff == 0 {
            return (x_min..=x_max).map(|x| Coordinate::new(x, self.from.y)).collect()
        }

        let c = y_diff * self.from.x as i32 + x_diff * self.from.y as i32;
        let f = |x| (-y_diff * x + c) / x_diff;
        (x_min..=x_max).map(|x| Coordinate::new(x, f(x as i32) as u32)).collect()
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once("->")
            .map(|(from, to)| (from.parse().unwrap(), to.parse().unwrap()))
            .unwrap();

        Ok(Line::new(from, to))
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

pub mod part1 {
    use std::collections::HashMap;
    use crate::days::day05::Line;
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let mut input = input_parser::parse_file::<Line>("inputs/day05.txt").unwrap();
        input.retain(|l| l.from.x == l.to.x || l.from.y == l.to.y);
        let mut points = HashMap::new();

        for line in input.iter() {
            for point in line.crossing_points() {
                points.insert(point, *points.get(&point).unwrap_or(&0_u32) + 1);
            }
        }

        points.values()
            .filter(|&&v| v > 1)
            .count() as u32
    }
}

pub mod part2 {
    use std::collections::HashMap;
    use crate::days::day05::Line;
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<Line>("inputs/day05.txt").unwrap();
        let mut points = HashMap::new();

        for line in input.iter() {
            for point in line.crossing_points() {
                points.insert(point, *points.get(&point).unwrap_or(&0_u32) + 1);
            }
        }

        points.values()
            .filter(|&&v| v > 1)
            .count() as u32
    }
}