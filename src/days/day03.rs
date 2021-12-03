pub mod part1 {
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let inputs = input_parser::parse_file::<String>("inputs/day03.txt").unwrap();
        let bits: Vec<Vec<char>> = inputs.iter()
            .map(|b| b.chars().collect())
            .collect();

        let mut most_common_bits: Vec<char> = vec![];
        for j in 0..bits[0].len() {
            let mut ones = 0;
            for i in 0..bits.len() {
                if bits[i][j] == '1' {
                    ones += 1;
                }
            }
            most_common_bits.push(
                if ones > bits.len() - ones { '1' } else { '0' }
            );
        }

        let gamma_rate = u32::from_str_radix(&most_common_bits.iter().collect::<String>(), 2).unwrap();
        let mask: u32 = 2_u32.pow(most_common_bits.len() as u32) - 1;
        let epsilon_rate = !gamma_rate & mask;

        gamma_rate * epsilon_rate
    }
}

pub mod part2 {
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let inputs = input_parser::parse_file::<String>("inputs/day03.txt").unwrap();
        let bits: Vec<Vec<char>> = inputs.iter()
            .map(|b| b.chars().collect())
            .collect();

        let mut oxygen_bits = bits.clone();
        let mut i = 0;
        while oxygen_bits.len() > 1 {
            let mcb = most_common_bit(&oxygen_bits.iter().map(|b| b[i]).collect());
            oxygen_bits.retain(|b| b[i] == mcb);
            i += 1;
        }

        let mut co2_bits = bits.clone();
        let mut i = 0;
        while co2_bits.len() > 1 {
            let lcb = least_common_bit(&co2_bits.iter().map(|b| b[i]).collect());
            co2_bits.retain(|b| b[i] == lcb);
            i += 1;
        }

        let oxygen_generator_rating = u32::from_str_radix(&oxygen_bits[0].iter().collect::<String>(), 2).unwrap();
        let co2_scrubber_rating = u32::from_str_radix(&co2_bits[0].iter().collect::<String>(), 2).unwrap();

        oxygen_generator_rating * co2_scrubber_rating
    }

    fn most_common_bit(bits: &Vec<char>) -> char {
        let mut ones = 0;
        for &bit in bits.iter() {
            if bit == '1' {
                ones += 1;
            }
        }

        return if ones >= bits.len() - ones { '1' } else { '0' }
    }

    fn least_common_bit(bits: &Vec<char>) -> char {
        let mut ones = 0;
        for &bit in bits.iter() {
            if bit == '1' {
                ones += 1;
            }
        }

        if bits.len() - ones > ones { '1' } else { '0' }
    }
}