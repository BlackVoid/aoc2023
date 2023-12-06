use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;
use rayon::prelude::*;

struct MapRange {
    from_range: Range<u64>,
    destination_start: u64
}

impl MapRange {
    pub fn from_str(data: &str) -> MapRange {
        let items: Vec<u64> = data.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
        MapRange {
            from_range: items[1]..(items[1]+items[2]),
            destination_start: items[0]
        }
    }

    pub fn map(&self, src: u64) -> u64 {
        (src-self.from_range.start)+self.destination_start
    }
}

struct SoilMap {
    from: String,
    to: String,
    ranges: Vec<MapRange>
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
            .map(|&x| MapRange::from_str(x))
            .collect::<Vec<MapRange>>();

        ranges.sort_by(|a, b| a.from_range.start.cmp(&b.from_range.start));

        SoilMap {
            from: from.to_string(),
            to: to.to_string(),
            ranges
        }
    }

    pub fn map(&self, src: u64) -> u64 {
        match self.ranges.binary_search_by(| x| {
            if src < x.from_range.start {
                Ordering::Greater
            } else if src >= x.from_range.end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Ok(i) => {
                let map = &self.ranges[i];
                (src-map.from_range.start)+map.destination_start
            },
            Err(_) => src
        }
    }
}

fn parse_input(_input: &str) -> (Vec<u64>, HashMap<String, SoilMap>) {
    let chunks: Vec<&str> = _input.split("\n\n").collect();
    let seeds: Vec<u64> = chunks[0]
        .split_once(": ")
        .unwrap().1
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let map: HashMap<String, SoilMap> = HashMap::from_iter(chunks[1..].into_iter()
        .map(|&data| SoilMap::from_str(data))
        .map(|map| (map.from.clone(), map))
    );

    (seeds, map)
}

fn solve (seed: u64, maps: &HashMap<String, SoilMap>) -> u64 {
    //let mut results: Vec<(String, u64)> = Vec::new();
    let mut next = &String::from("seed");
    let mut value = seed;
    //results.push((next.to_string(), value));

    loop {
        let map = match maps.get(next) {
            Some(m) => m,
            None => break
        };

        value = map.map(value);
        next = &map.to;
        //results.push((next.to_string(), value))
    }

    //println!("{}", results.clone().into_iter().map(|(x, y)| format!("{} {}", x, y)).collect::<Vec<String>>().join(", "));

    value
}

pub fn part_one(_input: &str) -> Option<u64> {
    println!("Parsing");
    let (seeds, maps) = parse_input(_input);
    println!("Solving");
    Some(seeds.into_iter().map(|s| solve(s, &maps)).min().unwrap())
}

pub fn part_two(_input: &str) -> Option<u64> {
    println!("Parsing");
    let (_seeds, maps) = parse_input(_input);

    println!("Chunking");
    let chunks: Vec<u64> = _seeds
        .chunks(2)
        .flat_map(|chunk| chunk[0]..(chunk[0]+chunk[1]))
        .collect();

    println!("Items {}", chunks.len());

    println!("Solving");

    Some(chunks.into_par_iter().map(|s| {
        solve(s, &maps)
    }).min().unwrap())
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range_map() {
        let result = MapRange::from_str("50 98 2");
        assert_eq!(result.destination_start, 50);
        assert_eq!(result.from_range, 98..100);
        assert_eq!(result.map(98), 50);
        assert_eq!(result.map(99), 51);
    }

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

    #[test]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", 5));
        assert_eq!(result, Some(261668924));
    }

    #[test]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", 5));
        assert_eq!(result, Some(261668924));
    }
}
