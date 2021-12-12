pub mod part1 {
    use std::collections::{HashMap, HashSet};
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use crate::util::input_parser;

    pub fn start() -> usize {
        let input: HashMap<String, HashSet<String>> = input_parser::parse_file::<String>("inputs/day12.txt").unwrap()
            .iter()
            .map(|l| l.split_once('-').map(|(from, to)| (from.to_string(), to.to_string())).unwrap())
            .fold(HashMap::new(), |mut acc, (from, to)| {
                if from != "end" && to != "start" {
                    acc.entry(from.clone()).or_insert(HashSet::new()).insert(to.clone());
                }

                if from != "start" && to != "end" {
                    acc.entry(to).or_insert(HashSet::new()).insert(from);
                }
                acc
            });

        let start_cave = String::from("start");
        let paths = find_all_paths(&input, vec![], &start_cave).unwrap();
        paths.len()
    }

    #[derive(Debug)]
    struct InvalidPathError {}

    impl Display for InvalidPathError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Invalid path")
        }
    }

    impl Error for InvalidPathError {
        fn description(&self) -> &str {
            "Invalid path"
        }
    }

    fn find_all_paths<'a>(cave_system: &'a HashMap<String, HashSet<String>>, mut visited: Vec<&'a String>, start_cave: &'a String) -> Result<Vec<Vec<(&'a String, &'a String)>>, InvalidPathError> {
        let mut paths = vec![];
        if start_cave == &"end".to_string() {
            return Ok(paths);
        }

        if &start_cave.to_lowercase() == start_cave {
            if visited.contains(&start_cave) {
                return Err(InvalidPathError {});
            } else {
                visited.push(start_cave);
            }
        }

        for next_cave in cave_system.get(start_cave).unwrap().iter() {
            if let Ok(mut new_paths) = find_all_paths(cave_system, visited.clone(), &next_cave) {
                if *next_cave == String::from("end") {
                    let mut new_path = vec![];
                    new_path.push((start_cave, next_cave));
                    paths.push(new_path);
                } else {
                    for path in new_paths.iter_mut() {
                        path.insert(0, (start_cave, next_cave))
                    }
                    paths.append(&mut new_paths);
                }
            }
        }
        Ok(paths)
    }
}