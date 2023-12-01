use common::itertools::Itertools;
use common::lazy_static::lazy_static;

static INPUT: &str = include_str!("input");

fn main() -> () {
    let lines = INPUT.lines().collect_vec();

    let res1 = part_1(&lines);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&lines);
    println!("Part 2 result: {}", res2);
}

fn part_1(lines: &[&str]) -> u32 {
    lines
        .iter()
        .flat_map(|line| {
            let mut digits = line.chars().flat_map(|c| c.to_digit(10));
            let first = digits.next()?;
            let last = digits.last().unwrap_or_else(|| first);
            Some(first * 10 + last)
        })
        .sum()
}

fn part_2(lines: &[&str]) -> u32 {
    fn match_digit(chars: &[char]) -> Option<u32> {
        lazy_static! {
            static ref SPELLED_DIGITS: [(Vec<char>, u32); 9] = [
                (vec!['o', 'n', 'e'], 1),
                (vec!['t', 'w', 'o'], 2),
                (vec!['t', 'h', 'r', 'e', 'e'], 3),
                (vec!['f', 'o', 'u', 'r'], 4),
                (vec!['f', 'i', 'v', 'e'], 5),
                (vec!['s', 'i', 'x'], 6),
                (vec!['s', 'e', 'v', 'e', 'n'], 7),
                (vec!['e', 'i', 'g', 'h', 't'], 8),
                (vec!['n', 'i', 'n', 'e'], 9),
            ];
        }
        SPELLED_DIGITS
            .iter()
            .find_map(|(k, v)| if chars.ends_with(k) { Some(*v) } else { None })
    }

    lines
        .iter()
        .flat_map(|line| {
            let mut chars = line.chars();
            let mut digits: Vec<u32> = Vec::new();
            let mut current_chars: Vec<char> = Vec::new();
            while let Some(c) = chars.next() {
                if let Some(d) = c.to_digit(10) {
                    digits.push(d);
                } else {
                    current_chars.push(c);
                    if let Some(d) = match_digit(&current_chars) {
                        digits.push(d);
                    }
                }
            }
            let first = digits.first()?;
            let last = digits.last()?;
            Some(first * 10 + last)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(part_1(&test_input), 142);
    }

    #[test]
    fn test_part_2() {
        let test_input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        assert_eq!(part_2(&test_input), 281);
    }
}
