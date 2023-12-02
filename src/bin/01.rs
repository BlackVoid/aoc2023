pub fn parse_part_one(_input: &str) -> Vec<String>  {
    _input.lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>())
        .map(|line| {
            let first: &char = line.first().unwrap();
            let last: &char = line.last().unwrap();

            let mut result: String = String::new();
            result.push(*first);
            result.push(*last);

            result
        }).collect()
}


pub fn part_one(_input: &str) -> Option<u32> {

    let data = parse_part_one(_input);
    Some(data.iter().map(|line| (*line).parse::<u32>().unwrap()).sum())
}



pub fn parse_part_two(_input: &str) -> Vec<String> {
    let res: String = _input.lines().map(|line| {
        let chars = line.chars().collect::<Vec<char>>();
        let mut res = String::new();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            if c.is_digit(10) {
                res.push(c);
                i += 1;
                continue
            }
            let mut found = false;
            for s in 3..=5 {
                let j = i+s;
                if j > chars.len() {
                    break
                }
                let word = String::from_iter(chars[i..j].iter());

                let num = match word.as_str() {
                    "zero" => Some('0'),
                    "one" => Some('1'),
                    "two" => Some('2'),
                    "three" => Some('3'),
                    "four" => Some('4'),
                    "five" => Some('5'),
                    "six" => Some('6'),
                    "seven" => Some('7'),
                    "eight" => Some('8'),
                    "nine" => Some('9'),
                    _ => None
                };

                match num {
                    Some(n) => {
                        res.push(n);
                        i += s-1;
                        found = true;
                        break
                    },
                    None => {
                        continue
                    }
                }
            }

            if !found {
                i += 1
            }
        }
        res
    }).collect::<Vec<String>>().join("\n");

    parse_part_one(res.as_str())
}

pub fn part_two(_input: &str) -> Option<u32> {
    let data = parse_part_two(_input);
    Some(data.iter().map(|line| (*line).parse::<u32>().unwrap()).sum())
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_part_one() {
        let result = parse_part_one(&advent_of_code::template::read_file("examples", 1));
        let mut correct_result = Vec::new();
        correct_result.push("12");
        correct_result.push("38");
        correct_result.push("15");
        correct_result.push("77");
        assert_eq!(result, correct_result);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_parse_part_two() {
        let example = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let result = parse_part_two(example);
        let correct_result: Vec<&str> = Vec::from(["29", "83", "13", "24", "42", "14", "76"]);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn test_part_two() {
        let example = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let result = part_two(example);
        assert_eq!(result, Some(281));
    }
}
