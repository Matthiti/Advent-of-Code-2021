pub mod part1 {
    use std::collections::HashMap;
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input: Vec<Vec<char>> = input_parser::parse_file::<String>("inputs/day10.txt").unwrap()
            .iter()
            .map(|s| s.chars().collect())
            .collect();

        let mut scores: HashMap<char, u32> = HashMap::new();
        scores.insert(')', 3);
        scores.insert(']', 57);
        scores.insert('}', 1197);
        scores.insert('>', 25137);

        let mut closing_opening = HashMap::new();
        closing_opening.insert(')', '(');
        closing_opening.insert(']', '[');
        closing_opening.insert('}', '{');
        closing_opening.insert('>', '<');

        input.iter()
            .map(|l| {
                let mut stack = vec![];
                for c in l.iter() {
                    if closing_opening.contains_key(c) {
                        if stack.pop().unwrap() != closing_opening.get(c).unwrap() {
                            return *scores.get(c).unwrap();
                        }
                    } else {
                        stack.push(c);
                    }
                }
                0
            })
            .sum()
    }
}

pub mod part2 {
    use std::collections::HashMap;
    use crate::util::input_parser;

    pub fn start() -> u64 {
        let input: Vec<Vec<char>> = input_parser::parse_file::<String>("inputs/day10.txt").unwrap()
            .iter()
            .map(|s| s.chars().collect())
            .collect();

        let mut scores: HashMap<char, u64> = HashMap::new();
        scores.insert(')', 1);
        scores.insert(']', 2);
        scores.insert('}', 3);
        scores.insert('>', 4);

        let mut closing_opening = HashMap::new();
        closing_opening.insert(')', '(');
        closing_opening.insert(']', '[');
        closing_opening.insert('}', '{');
        closing_opening.insert('>', '<');

        let mut line_scores = input.iter()
            .filter(|l| {
                let mut stack = vec![];
                for c in l.iter() {
                    if closing_opening.contains_key(c) {
                        if stack.pop().unwrap() != closing_opening.get(c).unwrap() {
                            return false;
                        }
                    } else {
                        stack.push(c);
                    }
                }
                true
            })
            .map(|l| {
                let mut stack = vec![];
                for c in l.iter() {
                    if closing_opening.contains_key(c) {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                }
                let mut score = 0;
                while let Some(c) = stack.pop() {
                    let closing = closing_opening.iter().find_map(|(key, value)| if value == c { Some(key) } else { None }).unwrap();
                    score = score * 5 + scores.get(closing).unwrap();
                }
                score
            })
            .collect::<Vec<u64>>();

        line_scores.sort();
        line_scores[(line_scores.len() - 1) / 2]
    }
}