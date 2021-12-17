pub mod part1 {
    use std::cmp::min;
    use crate::util::input_parser;

    pub fn start() -> i32 {
        let input = input_parser::parse_file_raw("inputs/day17.txt").unwrap();
        let (x, y) = input.split_once(", ").unwrap();
        let (target_x_min, target_x_max): (i32, i32) = x.replace("target area: x=", "")
            .split_once("..")
            .map(|(from, to)| (from.parse().unwrap(), to.parse().unwrap()))
            .unwrap();
        let (target_y_min, target_y_max): (i32, i32) = y.replace("y=", "")
            .split_once("..")
            .map(|(from, to)| (from.parse().unwrap(), to.parse().unwrap()))
            .unwrap();

        // The highest possible y is such that the x movement is minimal
        let mut min_velocity_x = 0;
        let mut x_distance = 0;
        while x_distance < target_x_min {
            min_velocity_x += 1;
            x_distance += min_velocity_x;
        }

        for velocity_y in (1..=100).rev() {
            let (end_x, end_y) = fly(min_velocity_x, velocity_y, target_x_max, target_y_min);
            if end_x >= target_x_min && end_x <= target_x_max && end_y >= target_y_min && end_y <= target_y_max {
                return find_top(velocity_y);
            }
        }

        -1
    }

    fn fly(velocity_x: i32, velocity_y: i32, bound_x: i32, bound_y: i32) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        let mut velocity_x = velocity_x;
        let mut velocity_y = velocity_y;
        while x + velocity_x <= bound_x && y + velocity_y >= bound_y {
            x += velocity_x;
            y += velocity_y;
            if velocity_x > 0 {
                velocity_x -= 1;
            } else if velocity_x < 0 {
                velocity_x += 1;
            }
            velocity_y -= 1;
        }
        (x, y)
    }

    fn find_top(velocity_y: i32) -> i32 {
        let mut velocity_y = velocity_y;
        let mut y = 0;
        while velocity_y > 0 {
            y += velocity_y;
            velocity_y -= 1;
        }
        y
    }
}

pub mod part2 {
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input = input_parser::parse_file_raw("inputs/day17.txt").unwrap();
        let (x, y) = input.split_once(", ").unwrap();
        let (target_x_min, target_x_max): (i32, i32) = x.replace("target area: x=", "")
            .split_once("..")
            .map(|(from, to)| (from.parse().unwrap(), to.parse().unwrap()))
            .unwrap();
        let (target_y_min, target_y_max): (i32, i32) = y.replace("y=", "")
            .split_once("..")
            .map(|(from, to)| (from.parse().unwrap(), to.parse().unwrap()))
            .unwrap();

        let mut count = 0;
        for velocity_x in 1..=1000 {
            for velocity_y in -1000..=1000 {
                let (end_x, end_y) = fly(velocity_x, velocity_y, target_x_max, target_y_min);
                if end_x >= target_x_min && end_x <= target_x_max && end_y >= target_y_min && end_y <= target_y_max {
                    count += 1;
                }
            }
        }
        count
    }

    fn fly(velocity_x: i32, velocity_y: i32, bound_x: i32, bound_y: i32) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        let mut velocity_x = velocity_x;
        let mut velocity_y = velocity_y;
        while x + velocity_x <= bound_x && y + velocity_y >= bound_y {
            x += velocity_x;
            y += velocity_y;
            if velocity_x > 0 {
                velocity_x -= 1;
            } else if velocity_x < 0 {
                velocity_x += 1;
            }
            velocity_y -= 1;
        }
        (x, y)
    }
}