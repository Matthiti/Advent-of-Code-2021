pub mod part1 {
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input: Vec<Vec<u32>> = input_parser::parse_file::<String>("inputs/day09.txt").unwrap()
            .iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        input.iter()
            .enumerate()
            .fold(0, |acc, (y, row)| {
                acc + row.iter()
                    .enumerate()
                    .filter(|(x, &value)| adjacent_values(&input, *x, y).iter().all(|&&v| v > value))
                    .fold(0, |acc, (_, value)| {
                        acc + 1 + value
                    })
            })
    }

    fn adjacent_values<T>(values: &Vec<Vec<T>>, x: usize, y: usize) -> Vec<&T> {
        let mut adjacent = vec![];
        if x > 0 {
            adjacent.push(&values[y][x - 1]);
        }

        if x < values[y].len() - 1 {
            adjacent.push(&values[y][x + 1]);
        }

        if y > 0 {
            adjacent.push(&values[y - 1][x]);
        }

        if y < values.len() - 1 {
            adjacent.push(&values[y + 1][x]);
        }

        adjacent
    }
}

pub mod part2 {
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input: Vec<Vec<u32>> = input_parser::parse_file::<String>("inputs/day09.txt").unwrap()
            .iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let mut basins: Vec<u32> = vec![];
        let mut global_visited = vec![];
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] == 9 || global_visited.contains(&(x, y)) {
                    continue;
                }

                let mut visited = vec![];
                let mut new_visited = vec![];
                new_visited.push((x, y));
                while new_visited.len() > 0 {
                    visited.append(&mut new_visited);
                    new_visited = vec![];

                    for (x, y) in visited.iter() {
                        let (x, y) = (*x, *y);
                        if x > 0 && input[y][x - 1] != 9 && !visited.contains(&(x - 1, y)) && !new_visited.contains(&(x - 1, y)) {
                            new_visited.push((x - 1, y));
                        }

                        if x < input[y].len() - 1 && input[y][x + 1] != 9 && !visited.contains(&(x + 1, y)) && !new_visited.contains(&(x + 1, y)) {
                            new_visited.push((x + 1, y));
                        }

                        if y > 0 && input[y - 1][x] != 9 && !visited.contains(&(x, y - 1)) &&  !new_visited.contains(&(x, y - 1)) {
                            new_visited.push((x, y - 1));
                        }

                        if y < input.len() - 1 && input[y + 1][x] != 9 && !visited.contains(&(x, y + 1)) && !new_visited.contains(&(x, y + 1)) {
                            new_visited.push((x, y + 1));
                        }
                    }
                }

                basins.push(visited.len() as u32);

                global_visited.append(&mut visited);
            }
        }

        basins.sort_by(|a, b| a.cmp(&b).reverse());
        basins.iter()
            .take(3)
            .product()
    }
}