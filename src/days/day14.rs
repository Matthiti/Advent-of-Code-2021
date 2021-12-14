pub mod part1 {
    use std::collections::HashMap;
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input: Vec<Vec<String>> = input_parser::parse_file::<String>("inputs/day14.txt").unwrap()
            .split(|line| *line == String::from(""))
            .map(|parts| parts.to_vec())
            .collect();

        let insertion_rules: HashMap<String, char> = input[1].iter()
            .map(|l| l.split_once(" -> ").map(|(a, b)| (a.to_string(), b.chars().nth(0).unwrap())).unwrap())
            .collect();

        let mut polymer = input[0][0].clone();
        let steps = 10;
        for _ in 0..steps {
            let mut new_polymer = polymer.clone();
            let mut next_insert_index = 1;
            for i in 0..polymer.len() - 1 {
                let pair: String = [polymer.chars().nth(i).unwrap(), polymer.chars().nth(i + 1).unwrap()].iter().collect();
                if let Some(insertion) = insertion_rules.get(&pair) {
                    new_polymer.insert(next_insert_index, *insertion);
                    next_insert_index += 2;
                } else {
                    next_insert_index += 1;
                }
            }
            polymer = new_polymer;
        }

        let counts = polymer.chars().into_iter()
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

        counts.values().max().unwrap() - counts.values().min().unwrap()
    }
}