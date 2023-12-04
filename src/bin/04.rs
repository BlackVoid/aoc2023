use std::collections::{HashSet};

#[derive(PartialEq, Debug)]
struct Card {
    num: u32,
    winning_numbers: Vec<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    pub fn from_str(s: &str) -> Card {
        let (first, second) = s.split_once(":").expect("Missing ':'");
        let card_num: u32 = first
            .split(" ").last().expect("Should a last one")
            .parse().expect("Failed to parse number");
        let (winners, numbers) = second.split_once("|").expect("Failed split");

        Card {
            num: card_num,
            winning_numbers: winners.trim().split(' ')
                .filter(|x| x.len() > 0)
                .map(|n| n.parse().expect("Failed parse")).collect(),
            numbers: numbers.trim().split(' ')
                .filter(|x| x.len() > 0)
                .map(|n| {
                    n.parse().expect("Failed parse")
                }).collect(),
        }
    }

    pub fn points(&self) -> u32 {
        let wins = self.winning_numbers.iter().filter(|v| self.numbers.contains(v)).count();
        if wins == 0 {
            return 0
        }
        2u32.pow((wins as u32) - 1)
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    Some(_input.lines().map(Card::from_str).map(|c| c.points()).sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_card_parse() {
        let result = Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let valid_card = Card {
            num: 1,
            winning_numbers: Vec::from([41, 48, 83, 86, 17]),
            numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
        };
        assert_eq!(result, valid_card);
    }

    #[test]
    fn test_card_points() {
        let card = Card {
            num: 1,
            winning_numbers: Vec::from([41, 48, 83, 86, 17]),
            numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
        };
        assert_eq!(card.points(), 8);

        let losing_card = Card {
            num: 1,
            winning_numbers: Vec::from([31, 48, 13, 56, 72]),
            numbers: HashSet::from([74, 77, 10, 23, 35, 67, 36, 11])
        };
        assert_eq!(losing_card.points(), 0);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, None);
    }
}
