fn get_adjacent_non_flashing_octopuses(energy_levels: &Vec<Vec<u32>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    cartesian_product((x as i32 - 1..=x as i32 + 1).collect(), (y as i32 - 1..=y as i32 + 1).collect())
        .into_iter()
        .filter(|&c| c != (x as i32, y as i32))
        .filter(|&(x, y)| y >= 0 && y < energy_levels.len() as i32 && x >= 0 && x < energy_levels[y as usize].len() as i32)
        .filter(|&(x, y)| energy_levels[x as usize][y as usize] < 10)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

fn cartesian_product<T>(a: Vec<T>, b: Vec<T>) -> Vec<(T, T)>
    where T: Copy {
    let mut cartesian_product = vec![];
    for &i in a.iter() {
        for &j in b.iter() {
            cartesian_product.push((i, j));
        }
    }
    cartesian_product
}

pub mod part1 {
    use crate::days::day11::get_adjacent_non_flashing_octopuses;
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let mut energy_levels: Vec<Vec<u32>> = input_parser::parse_file::<String>("inputs/day11.txt").unwrap()
            .iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let mut flashes = 0;
        let steps = 100;
        for _ in 0..steps {
            energy_levels = energy_levels.iter().map(|row| row.iter().map(|l| l + 1).collect()).collect();
            let mut flashed = vec![];
            let mut new_increased = vec![];
            let mut init = true;

            while init || new_increased.len() > 0 {
                new_increased = vec![];
                for (x, row) in energy_levels.iter().enumerate() {
                    for (y, &energy_level) in row.iter().enumerate() {
                        if energy_level > 9 && !flashed.contains(&(x, y)) {
                            flashes += 1;
                            let adjacent = get_adjacent_non_flashing_octopuses(&energy_levels, x, y);
                            for &(x, y) in adjacent.iter() {
                                new_increased.push((x, y));
                            }
                            flashed.push((x, y));
                        }
                    }
                }
                for &(x, y) in new_increased.iter() {
                    energy_levels[x][y] += 1;
                }
                init = false;
            }
            energy_levels = energy_levels.iter().map(|row| row.iter().map(|&l| if l > 9 { 0 } else { l }).collect()).collect()
        }

        flashes
    }
}

pub mod part2 {
    use crate::days::day11::get_adjacent_non_flashing_octopuses;
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let mut energy_levels: Vec<Vec<u32>> = input_parser::parse_file::<String>("inputs/day11.txt").unwrap()
            .iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let mut steps = 0;
        while !do_all_octopuses_flash(&energy_levels) {
            steps += 1;
            energy_levels = energy_levels.iter().map(|row| row.iter().map(|l| l + 1).collect()).collect();
            let mut flashed = vec![];
            let mut new_increased = vec![];
            let mut init = true;

            while init || new_increased.len() > 0 {
                new_increased = vec![];
                for (x, row) in energy_levels.iter().enumerate() {
                    for (y, &energy_level) in row.iter().enumerate() {
                        if energy_level > 9 && !flashed.contains(&(x, y)) {
                            let adjacent = get_adjacent_non_flashing_octopuses(&energy_levels, x, y);
                            for &(x, y) in adjacent.iter() {
                                new_increased.push((x, y));
                            }
                            flashed.push((x, y));
                        }
                    }
                }
                for &(x, y) in new_increased.iter() {
                    energy_levels[x][y] += 1;
                }
                init = false;
            }
            energy_levels = energy_levels.iter().map(|row| row.iter().map(|&l| if l > 9 { 0 } else { l }).collect()).collect()
        }

        steps
    }

    fn do_all_octopuses_flash(energy_levels: &Vec<Vec<u32>>) -> bool {
        for row in energy_levels.iter() {
            for &energy_level in row.iter() {
                if energy_level != 0 {
                    return false;
                }
            }
        }
        true
    }
}