pub fn parse_input(_input: &str) -> Vec<Vec<char>>{
    _input.lines().map(|x| x.chars().collect()).collect()
}

pub fn count_score(data: &Vec<Vec<char>>, loc: (usize, (usize, usize))) -> u32 {
    let number = String::from_iter(data[loc.0][loc.1.0..=loc.1.1].iter()).parse::<u32>().unwrap();
    let width = data.get(0).unwrap().len();
    let mut rows = Vec::from([loc.0]);
    if loc.0 > 0 {
        rows.push(loc.0-1)
    }
    if loc.0 < data.len()-1 {
        rows.push(loc.0+1)
    }

    let symbols: usize = rows.into_iter().map(|row_index| {
        let coordinates: Vec<(usize, usize)> = if row_index == loc.0 {
            let mut cols = Vec::new();
            if loc.1.0 > 0 {
                cols.push((row_index, loc.1.0 -1))
            }
            if loc.1.1 < data.len()-1 {
                cols.push((row_index, loc.1.1 + 1))
            }
            cols
        } else {
            let min = if loc.1.0 > 0 { loc.1.0 - 1 } else { loc.1.0 };
            let max = if loc.1.1 < width-1 { loc.1.1 + 1 } else { loc.1.1 };
            (min..=max)
                .map(|x| (row_index, x))
                .collect::<Vec<(usize, usize)>>()
        };
        let characters: Vec<char> = coordinates.into_iter().filter(|(_, ci)| {
            *ci < width
        }).map(|(ri, ci)| data[ri][ci]).collect();

        characters.into_iter().filter(|c| {
            match *c {
                '0'..='9' => false,
                '.' => false,
                _ => true,
            }
        }).count()
    }).sum();

    if symbols > 0 {
        number
    } else {
        0
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let data = parse_input(_input);
    let width = data[0].len();
    let mut sum: u32 = 0;
    for (row_index, row) in data.iter().enumerate() {
        let mut i = 0;
        let mut j = 0;
        let mut search = true;
        for (column_index, col) in row.iter().enumerate() {
            if col.is_digit(10) {
                if search {
                    search = false;
                    i = column_index;
                }
                j = column_index;
                if column_index + 1 < width {
                    continue
                }
            }

            if !search {
                sum += count_score(&data, (row_index, (i, j)));
                search = true;
            }
        }
    }
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", 3));
        assert_eq!(result, Some(535235));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, None);
    }
}
