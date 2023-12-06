use std::ops::Range;
use rayon::prelude::*;

struct SoilMap {
    from: String,
    to: String,
    ranges: Vec<(u64, u64, u64)>
}

impl SoilMap {
    pub fn from_str(data: &str) -> SoilMap {
        let lines: Vec<&str> = data.lines().collect();
        let first = *lines.first().expect("No items");
        let (from, to) = first
            .split_once(' ')
            .expect("No space found")
            .0
            .split_once("-to-")
            .expect("Should find two parts");

        let mut ranges = lines[1..].into_iter()
            .map(|&range| {
                let items: Vec<u64> = range.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
                (items[1], items[1]+items[2], items[0])
            })
            .collect::<Vec<(u64, u64, u64)>>();

        ranges.sort_by(|(sa, _, _), (sb, _, _)| sa.cmp(sb));

        SoilMap {
            from: from.to_string(),
            to: to.to_string(),
            ranges
        }
    }

    pub fn map(&self, src: u64) -> u64 {
        match self.ranges.iter().find(| &(start, end, base)| start <= &src && &src < end) {
            Some((start, _, base)) => {
                (src-start)+base
            },
            None => src
        }
    }
}

fn parse_input(_input: &str) -> (Vec<u64>, Vec<SoilMap>) {
    let chunks: Vec<&str> = _input.split("\n\n").collect();
    let seeds: Vec<u64> = chunks[0]
        .split_once(": ")
        .unwrap().1
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let map: Vec<SoilMap> = chunks[1..].into_iter()
        .map(|&data| SoilMap::from_str(data))
        .collect();

    (seeds, map)
}

fn solve (seed: u64, maps: &Vec<SoilMap>) -> u64 {
    maps.iter().fold(seed, |acc, map| map.map(acc))
}

pub fn part_one(_input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(_input);
    Some(seeds.into_iter().map(|s| solve(s, &maps)).min().unwrap())
}

pub fn part_two(_input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(_input);

    let result = seeds
        .par_chunks(2)
        .into_par_iter()
        .flat_map(|chunk| chunk[0]..(chunk[0]+chunk[1]))
        .map(|c| {
            solve(c, &maps)
        })
        .min().unwrap();

    Some(result)
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_soil_map() {
        let result = SoilMap::from_str("seed-to-soil map:\n50 98 2\n52 50 48");
        assert_eq!(result.from, "seed");
        assert_eq!(result.to, "soil");
        assert_eq!(result.ranges.len(), 2);
        assert_eq!(result.map(1), 1);
        assert_eq!(result.map(50), 52);
        assert_eq!(result.map(97), 99);
        assert_eq!(result.map(98), 50);
        assert_eq!(result.map(99), 51);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(46));
    }
}
