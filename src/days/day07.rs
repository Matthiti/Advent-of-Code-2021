pub mod part1 {
    use std::collections::HashMap;
    use crate::util::input_parser;

    pub fn start() -> i32 {
        let input = input_parser::parse_file_horizontal::<i32>("inputs/day07.txt", ",").unwrap();
        let min = *input.iter().min().unwrap();
        let max = *input.iter().max().unwrap();

        let costs = (min..=max)
            .fold(HashMap::new(), |mut acc, target| {
                let cost = input.iter()
                    .fold(0, |cost, crab| cost + (crab - target).abs());
                acc.insert(target, cost);
                acc
            });

        *costs.values().min().unwrap()
    }
}

pub mod part2 {
    use std::collections::HashMap;
    use crate::util::input_parser;

    pub fn start() -> i32 {
        let input = input_parser::parse_file_horizontal::<i32>("inputs/day07.txt", ",").unwrap();
        let min = *input.iter().min().unwrap();
        let max = *input.iter().max().unwrap();

        let costs = (min..=max)
            .fold(HashMap::new(), |mut acc, target| {
                let cost = input.iter()
                    .fold(0, |cost, crab|
                        cost + (1..=(crab - target).abs())
                            .reduce(|a, b| a + b)
                            .unwrap_or(0)
                    );
                acc.insert(target, cost);
                acc
            });

        *costs.values().min().unwrap()
    }
}