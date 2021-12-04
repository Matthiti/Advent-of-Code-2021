use std::str::FromStr;

#[derive(Debug, Clone)]
struct BingoCardField {
    value: u32,
    marked: bool
}

impl BingoCardField {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            marked: false
        }
    }
}

#[derive(Debug, Clone)]
struct BingoCard {
    fields: Vec<Vec<BingoCardField>>
}

impl FromStr for BingoCard {
    // Ignore error for now
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<&str> = s.trim().split('\n').collect();
        let fields = rows.iter()
            .map(|r| r.trim().split(' ').filter(|c| !c.is_empty()).map(|c| BingoCardField::new(c.parse().unwrap())).collect())
            .collect();

        Ok(BingoCard::new(fields))
    }
}

impl BingoCard {
    pub fn new(fields: Vec<Vec<BingoCardField>>) -> Self {
        Self {
            fields
        }
    }

    pub fn update(&mut self, value: u32) {
        for i in 0..self.fields.len() {
            for j in 0..self.fields[i].len() {
                if self.fields[i][j].value == value {
                    self.fields[i][j].marked = true;
                    return;
                }
            }
        }
    }

    pub fn has_bingo(&self) -> bool {
        self.has_bingo_horizontal() || self.has_bing_vertical()
    }

    fn has_bingo_horizontal(&self) -> bool {
        for row in self.fields.iter() {
            if row.iter().all(|f| f.marked) {
                return true;
            }
        }
        false
    }

    fn has_bing_vertical(&self) -> bool {
        for i in 0..self.fields[0].len() {
            let col: Vec<&BingoCardField> = self.fields.iter().map(|r| &r[i]).collect();
            if col.iter().all(|f| f.marked) {
                return true;
            }
        }
        false
    }
}

pub mod part1 {
    use crate::days::day04::BingoCard;
    use crate::util::input_parser;

    pub fn start() -> i32 {
        let input = input_parser::parse_file_raw("inputs/day04.txt").unwrap();
        let parts = input.split_once('\n').unwrap();
        let bingo_numbers: Vec<u32> = parts.0.split(',').map(|s| s.parse().unwrap()).collect();
        let mut bingo_cards: Vec<BingoCard> = parts.1.split("\n\n").map(|s| s.parse().unwrap()).collect();

        for number in bingo_numbers {
            for bingo_card in bingo_cards.iter_mut() {
                bingo_card.update(number);
                if bingo_card.has_bingo() {
                    return (bingo_card.fields.iter()
                        .flatten()
                        .filter(|f| !f.marked)
                        .map(|f| f.value)
                        .sum::<u32>() * number) as i32;
                }
            }
        }
        // No card has bingo :(
        -1
    }
}

pub mod part2 {
    use crate::days::day04::BingoCard;
    use crate::util::input_parser;

    pub fn start() -> i32 {
        let input = input_parser::parse_file_raw("inputs/day04.txt").unwrap();
        let parts = input.split_once('\n').unwrap();
        let bingo_numbers: Vec<u32> = parts.0.split(',').map(|s| s.parse().unwrap()).collect();
        let mut bingo_cards: Vec<BingoCard> = parts.1.split("\n\n").map(|s| s.parse().unwrap()).collect();

        for number in bingo_numbers {
            let mut top_remaining_card = bingo_cards[0].clone();
            bingo_cards.iter_mut().for_each(|c| c.update(number));
            bingo_cards.retain(|c| !c.has_bingo());
            if bingo_cards.len() == 0 {
                top_remaining_card.update(number);
                return (top_remaining_card.fields.iter()
                    .flatten()
                    .filter(|f| !f.marked)
                    .map(|f| f.value)
                    .sum::<u32>() * number) as i32;
            }
        }
        // Should never happen
        -1
    }
}