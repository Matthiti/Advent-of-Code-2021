pub mod part1 {
    use crate::util::input_parser;

    pub fn start() -> usize {
        let mut fish: Vec<u32> = input_parser::parse_file_raw("inputs/day06.txt").unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let days = 80;
        for _ in 0..days {
            for i in 0..fish.len() {
                if fish[i] == 0 {
                    fish.push(8);
                    fish[i] = 6;
                } else {
                    fish[i] = fish[i] - 1;
                }
            }
        }

        fish.len()
    }
}

pub mod part2 {
    use crate::util::input_parser;

    pub fn start() -> u64 {
        let input: Vec<u32> = input_parser::parse_file_raw("inputs/day06.txt").unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let mut cycles = vec![0; 9];
        input.into_iter().for_each(|i| cycles[i as usize] += 1);

        let days = 256;
        for _ in 0..days {
            let mut new_cycles = vec![0; 9];
            for i in 0..cycles.len() - 1 {
                new_cycles[i] = cycles[i + 1];
            }
            new_cycles[6] += cycles[0];
            new_cycles[8] = cycles[0];
            cycles = new_cycles;
        }

        cycles.iter().sum()
    }
}