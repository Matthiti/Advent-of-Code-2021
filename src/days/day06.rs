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
    use std::collections::HashMap;
    use crate::util::input_parser;

    pub fn start() -> u64 {
        let input: Vec<u32> = input_parser::parse_file_raw("inputs/day06.txt").unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let map = (0..9).fold(HashMap::new(), |mut acc, f| {
            acc.insert(f, 0);
            acc
        });

        let mut cycles: Vec<(u32, u64)> = input.iter().fold(map, |mut acc, f| {
            acc.insert(*f, acc.get(f).unwrap() + 1);
            acc
        }).into_iter().collect();

        let days = 256;
        for _ in 0..days {
            let mut new_cycles = vec![];
            let mut reset_fish = 0;
            for i in 0..cycles.len() {
                let (timer, count) = cycles[i];
                if timer == 0 {
                    new_cycles.push((8, count));
                    reset_fish += count;
                } else {
                    new_cycles.push((timer - 1, count));
                }
            }

            if reset_fish > 0 {
                for i in 0..new_cycles.len() {
                    let (timer, _) = new_cycles[i];
                    if timer == 6 {
                        new_cycles[i].1 += reset_fish;
                    }
                }
            }

            cycles = new_cycles;
        }

        cycles.iter()
            .map(|(_, count)| count)
            .sum()
    }
}