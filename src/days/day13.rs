pub mod part1 {
    use crate::util::input_parser;

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum Field {
        Empty,
        Dot
    }

    pub fn start() -> usize {
        let input: Vec<String> = input_parser::parse_file::<String>("inputs/day13.txt").unwrap();

        let dots_and_folds: Vec<Vec<String>> = input
            .split(|line| *line == String::from(""))
            .map(|parts| parts.to_vec())
            .collect();

        let dots: Vec<(usize, usize)> = dots_and_folds[0].iter()
            .map(|l| l.split_once(',').map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())).unwrap())
            .collect();

        let folds: Vec<(String, usize)> = dots_and_folds[1].iter()
            .map(|l| l.replace("fold along ", "").split_once("=").map(|(axis, value)| (axis.to_string(), value.parse().unwrap())).unwrap())
            .collect();

        let max_x = dots.iter().max_by_key(|(x, _)| x).unwrap().0;
        let max_y = dots.iter().max_by_key(|(_, y)| y).unwrap().1;

        let mut fields = vec![vec![Field::Empty; max_x + 1]; max_y + 1];
        dots.into_iter().for_each(|(x, y)| fields[y][x] = Field::Dot);

        let (fold_axis, fold_value) = &folds[0];
        fields = match fold_axis.as_str() {
            "x" => {
                let mut left_part: Vec<Vec<Field>> = fields.iter().map(|row| row[..*fold_value].to_vec()).collect();
                let right_part: Vec<Vec<Field>> = fields.iter().map(|row| row[*fold_value + 1..].to_vec()).collect();
                for (y, row) in right_part.iter().enumerate() {
                    for (x, &field) in row.iter().enumerate() {
                        if field == Field::Dot {
                            left_part[y][fold_value - 1 - x] = Field::Dot;
                        }
                    }
                }
                left_part
            },
            "y" => {
                let mut top_part = fields[..*fold_value].to_vec();
                let bottom_part = fields[*fold_value + 1..].to_vec();
                for (y, row) in bottom_part.iter().enumerate() {
                    for (x, &field) in row.iter().enumerate() {
                        if field == Field::Dot {
                            top_part[fold_value - 1 - y][x] = Field::Dot;
                        }
                    }
                }
                top_part
            },
            _ => fields
        };

        fields.iter()
            .flatten()
            .filter(|field| **field == Field::Dot)
            .count()
    }
}

pub mod part2 {
    use crate::util::input_parser;

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum Field {
        Empty,
        Dot
    }

    pub fn start() {
        let input: Vec<String> = input_parser::parse_file::<String>("inputs/day13.txt").unwrap();

        let dots_and_folds: Vec<Vec<String>> = input
            .split(|line| *line == String::from(""))
            .map(|parts| parts.to_vec())
            .collect();

        let dots: Vec<(usize, usize)> = dots_and_folds[0].iter()
            .map(|l| l.split_once(',').map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())).unwrap())
            .collect();

        let folds: Vec<(String, usize)> = dots_and_folds[1].iter()
            .map(|l| l.replace("fold along ", "").split_once("=").map(|(axis, value)| (axis.to_string(), value.parse().unwrap())).unwrap())
            .collect();

        let max_x = dots.iter().max_by_key(|(x, _)| x).unwrap().0;
        let max_y = dots.iter().max_by_key(|(_, y)| y).unwrap().1;

        let mut fields = vec![vec![Field::Empty; max_x + 1]; max_y + 1];
        dots.into_iter().for_each(|(x, y)| fields[y][x] = Field::Dot);

        for (fold_axis, fold_value) in folds {
            fields = match fold_axis.as_str() {
                "x" => {
                    let mut left_part: Vec<Vec<Field>> = fields.iter().map(|row| row[..fold_value].to_vec()).collect();
                    let right_part: Vec<Vec<Field>> = fields.iter().map(|row| row[fold_value + 1..].to_vec()).collect();
                    for (y, row) in right_part.iter().enumerate() {
                        for (x, &field) in row.iter().enumerate() {
                            if field == Field::Dot {
                                left_part[y][fold_value - 1 - x] = Field::Dot;
                            }
                        }
                    }
                    left_part
                },
                "y" => {
                    let mut top_part = fields[..fold_value].to_vec();
                    let bottom_part = fields[fold_value + 1..].to_vec();
                    for (y, row) in bottom_part.iter().enumerate() {
                        for (x, &field) in row.iter().enumerate() {
                            if field == Field::Dot {
                                top_part[fold_value - 1 - y][x] = Field::Dot;
                            }
                        }
                    }
                    top_part
                },
                _ => fields
            };
        }

        for row in fields.iter() {
            for &field in row.iter() {
                print!("{}", match field {
                    Field::Dot => "##",
                    Field::Empty => ".."
                });
            }
            println!();
        }
    }
}