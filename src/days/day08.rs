pub mod part1 {
    use crate::util::input_parser;

    pub fn start() -> u32 {
        input_parser::parse_file::<String>("inputs/day08.txt").unwrap()
            .into_iter()
            .flat_map(|s| s.split_once('|').unwrap().1.trim().split(' ').map(|i| i.to_string()).collect::<Vec<String>>())
            .fold(0, |acc, el| {
                match el.len() {
                    2 | 3 | 4 | 7 => acc + 1,
                    _ => acc
                }
            })
    }
}

pub mod part2 {
    use std::collections::HashMap;
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input: Vec<(Vec<String>, Vec<String>)> = input_parser::parse_file::<String>("inputs/day08.txt").unwrap()
            .into_iter()
            .map(|s| s.split_once('|').map(|(i, o)| (i.to_string(), o.to_string())).unwrap())
            .map(|(i, o) | (i.trim().split(' ').map(|a| a.to_string()).collect(), o.trim().split(' ').map(|a| a.to_string()).collect()))
            .collect();

        /*
        ----- 1 -----
        |           |
        7           2
        |           |
        ----- 3 -----
        |           |
        6           4
        |           |
        ----- 5 -----
         */

        input.iter()
            .map(|(inp, outp)| {
                let wiring = find_wiring(inp);
                outp.iter().map(|s| {
                    let mut digit_pos = s.chars().map(|c| *wiring.get(&c).unwrap()).collect::<Vec<u8>>();
                    digit_pos.sort();
                    digit_pos
                }).collect::<Vec<Vec<u8>>>()
            })
            .fold(0, |acc, output| {
                acc + output.iter().enumerate().fold(0, |acc, (i, digit_pos)| {
                    let digit = digit_positions().into_iter().position(|a| a == *digit_pos).unwrap();
                    acc + 10_u32.pow(output.len() as u32 - 1 - i as u32) * digit as u32
                })
            })
    }

    // Ugly 😬
    fn find_wiring(input: &Vec<String>) -> HashMap<char, u8> {
        let mut wiring = HashMap::new();
        for a in 1..=7 {
            wiring.insert('a', a);
            for b in 1..=7 {
                if wiring.values().any(|&val| val == b) {
                    continue;
                }
                wiring.insert('b', b);
                for c in 1..=7 {
                    if wiring.values().any(|&val| val == c) {
                        continue;
                    }
                    wiring.insert('c', c);
                    for d in 1..=7 {
                        if wiring.values().any(|&val| val == d) {
                            continue;
                        }
                        wiring.insert('d', d);
                        for e in 1..=7 {
                            if wiring.values().any(|&val| val == e) {
                                continue;
                            }
                            wiring.insert('e', e);
                            for f in 1..=7 {
                                if wiring.values().any(|&val| val == f) {
                                    continue;
                                }
                                wiring.insert('f', f);
                                for g in 1..=7 {
                                    if wiring.values().any(|&val| val == g) {
                                        continue;
                                    }
                                    wiring.insert('g', g);
                                    if test(&wiring, input) {
                                        return wiring;
                                    }
                                    wiring.remove(&'g');
                                }
                                wiring.remove(&'f');
                            }
                            wiring.remove(&'e');
                        }
                        wiring.remove(&'d');
                    }
                    wiring.remove(&'c');
                }
                wiring.remove(&'b');
            }
            wiring.remove(&'a');
        }

        HashMap::new()
    }

    fn digit_positions() -> Vec<Vec<u8>> {
        vec![
            // 0
            vec![1, 2, 4, 5, 6, 7],
            // 1
            vec![2, 4],
            // 2
            vec![1, 2, 3, 5, 6],
            // 3
            vec![1, 2, 3, 4, 5],
            // 4
            vec![2, 3, 4, 7],
            // 5
            vec![1, 3, 4, 5, 7],
            // 6
            vec![1, 3, 4, 5, 6, 7],
            // 7
            vec![1, 2, 4],
            // 8
            vec![1, 2, 3, 4, 5, 6, 7],
            // 9
            vec![1, 2, 3, 4, 5, 7]
        ]
    }

    fn test(wiring: &HashMap<char, u8>, input: &Vec<String>) -> bool {
        for s in input.iter() {
            let mut possible_positions = digit_positions();
            possible_positions.retain(|a| a.len() == s.len());

            let mut wiring_positions: Vec<u8> = s.chars()
                .map(|c| *wiring.get(&c).unwrap())
                .collect();
            wiring_positions.sort();

            if !possible_positions.contains(&wiring_positions) {
                return false;
            }
        }
        true
    }
}