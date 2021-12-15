pub mod part1 {
    use std::collections::HashMap;
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input: Vec<Vec<u32>> = input_parser::parse_file::<String>("inputs/day15.txt").unwrap()
            .iter()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let rings = input.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (y, row)| {
                for (x, _) in row.iter().enumerate() {
                    acc.entry(2 * input.len() - x - y).or_insert(vec![]).push((x, y));
                }
                acc
            });

        let mut rings: Vec<_> = rings.iter().collect();
        rings.sort_by(|a, b| a.0.cmp(&b.0));

        let rings: Vec<_> = rings.iter()
            .map(|(_, c)| c)
            .collect();

        let mut min_paths = HashMap::new();
        min_paths.insert(rings[0][0], 0);
        for i in 1..rings.len() {
            for &(x, y) in rings[i].iter() {
                let neighbors = get_neighbors(x, y, input.len());
                let mut min_path = None;

                for (n_x, n_y) in neighbors.into_iter() {
                    if let Some(val) = min_paths.get(&(n_x, n_y)) {
                        let new_path = *val + input[n_y][n_x];
                        if min_path.is_none() || new_path < min_path.unwrap() {
                            min_path = Some(new_path);
                        }
                    }
                }

                let min_path = min_path.unwrap();
                if let Some(val) = min_paths.get(&(x, y))  {
                    if min_path < *val {
                        min_paths.insert((x, y), min_path);
                    }
                } else {
                    min_paths.insert((x, y), min_path);
                }
            }
        }

        min_paths[&(0, 0)]
    }

    fn get_neighbors(x: usize, y: usize, max: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }

        if x < max - 1 {
            neighbors.push((x + 1, y));
        }

        if y > 0 {
            neighbors.push((x, y - 1));
        }

        if y < max - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }
}

pub mod part2 {
    use std::collections::HashMap;
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input: Vec<Vec<u32>> = input_parser::parse_file::<String>("inputs/day15.txt").unwrap()
            .iter()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let mut new_map = vec![vec![0; 5 * input[0].len()]; 5 * input.len()];

        for i in 0..5 {
            for j in 0..5 {
                for (y, row) in input.iter().enumerate() {
                    for (x, val) in row.iter().enumerate() {
                        let new_val = val + i as u32 + j as u32;
                        new_map[i * input.len() + y][j * row.len() + x] = if new_val > 9 {
                            new_val % 10 + 1
                        }  else {
                            new_val
                        };
                    }
                }
            }
        }

        let rings = new_map.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (y, row)| {
                for (x, _) in row.iter().enumerate() {
                    acc.entry(2 * new_map.len() - x - y).or_insert(vec![]).push((x, y));
                }
                acc
            });

        let mut rings: Vec<_> = rings.iter().collect();
        rings.sort_by(|a, b| a.0.cmp(&b.0));

        let rings: Vec<_> = rings.iter()
            .map(|(_, c)| c)
            .collect();

        let mut min_paths = HashMap::new();
        min_paths.insert(rings[0][0], 0);
        let mut improving = true;
        while improving {
            improving = false;
            for i in 1..rings.len() {
                for &(x, y) in rings[i].iter() {
                    let neighbors = get_neighbors(x, y, new_map.len());
                    let mut min_path = min_paths.get(&(x, y)).map(|n| *n);

                    for (n_x, n_y) in neighbors.into_iter() {
                        if let Some(val) = min_paths.get(&(n_x, n_y)) {
                            let new_path = *val + new_map[n_y][n_x];
                            if min_path.is_none() || new_path < min_path.unwrap() {
                                min_path = Some(new_path);
                                improving = true;
                            }
                        }
                    }

                    let min_path = min_path.unwrap();
                    if let Some(val) = min_paths.get(&(x, y))  {
                        if min_path < *val {
                            min_paths.insert((x, y), min_path);
                            improving = true;
                        }
                    } else {
                        min_paths.insert((x, y), min_path);
                        improving = true;
                    }
                }
            }
        }

        min_paths[&(0, 0)]
    }

    fn get_neighbors(x: usize, y: usize, max: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }

        if x < max - 1 {
            neighbors.push((x + 1, y));
        }

        if y > 0 {
            neighbors.push((x, y - 1));
        }

        if y < max - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }
}