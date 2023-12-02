use std::collections::HashMap;

struct Game {
    id: u32,
    reveals: Vec<HashMap<String, u32>>
}

impl Game {
    pub fn from_str(_input: &str) -> Game {
        let (game, rest) = _input.split_once(":").expect("Failed to find ':'");
        let game_id = game.split_once(" ").unwrap().1.parse::<u32>().unwrap();
        let reveals: Vec<HashMap<String, u32>> = rest.split(";")
            .map(|group| {
                group.split(",").map(|item| {
                    let (count, color) = item.trim().split_once(" ").unwrap();
                    (color.to_string(), count.parse::<u32>().unwrap())
                }).collect()
            }).collect();

        Game {
            id: game_id,
            reveals,
        }
    }

    fn is_possible(&self, limits: &HashMap<&str, u32>) -> bool {
        for reveal in &self.reveals {
            for (color, count) in limits {
                match reveal.get(*color) {
                    Some(cnt) => {
                        if *cnt > *count {
                            return false;
                        }
                    }
                    None => {}
                }
            }
        }
        true
    }

    fn power(&self) -> u32 {
        ["red", "green", "blue"].map(|color| {
            self.reveals.iter()
                .map(|reveal| reveal.get(color))
                .filter(Option::is_some)
                .map(Option::unwrap)
                .max().unwrap().clone()
        }).iter().copied().reduce(|x, y| x * y).unwrap()
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let filters: HashMap<&str, u32> = [
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ].into_iter().collect();

    let sum = _input
        .lines()
        .map(Game::from_str)
        .filter(|game| game.is_possible(&filters))
        .map(|game| game.id)
        .sum();
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let sum = _input
        .lines()
        .map(Game::from_str)
        .map(|game| game.power())
        .sum();
    Some(sum)

}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let result = Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(result.id, 1);
        assert_eq!(result.reveals.len(), 3);
        assert_eq!(result.reveals[0]["blue"], 3);
        assert_eq!(result.reveals[0]["red"], 4);
        assert_eq!(result.reveals[1]["red"], 1);
        assert_eq!(result.reveals[1]["green"], 2);
        assert_eq!(result.reveals[1]["blue"], 6);
        assert_eq!(result.reveals[2]["green"], 2);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(2286));
    }
}
