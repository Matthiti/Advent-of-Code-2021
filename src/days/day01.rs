pub mod part1 {
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<u32>("inputs/day01.txt").unwrap();

        let mut increase_count = 0;
        for i in 1..input.len() {
            if input[i] > input[i - 1] {
                increase_count += 1;
            }
        }

        return increase_count;
    }
}

pub mod part2 {
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<u32>("inputs/day01.txt").unwrap();

        let mut window_sums = vec![];
        for i in 2..input.len() {
            window_sums.push(input[i - 2] + input[i - 1] + input[i])
        }

        let mut increase_count = 0;
        for i in 1..window_sums.len() {
            if window_sums[i] > window_sums[i - 1] {
                increase_count += 1;
            }
        }

        return increase_count;
    }
}