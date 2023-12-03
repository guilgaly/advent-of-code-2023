use common::itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;

static INPUT: &str = include_str!("input");

fn main() {
    let (symbols, numbers, stars) = parse_input(INPUT);

    let res1 = part_1(&symbols, &numbers);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&symbols, &numbers, &stars);
    println!("Part 2 result: {}", res2);
}

fn part_1(symbols: &HashSet<Point>, numbers: &[Part]) -> usize {
    valid_parts(symbols, numbers).iter().map(|p| p.number).sum()
}

fn part_2(symbols: &HashSet<Point>, numbers: &[Part], stars: &[Point]) -> usize {
    let parts = valid_parts(symbols, numbers);
    stars
        .iter()
        .filter_map(|star| {
            let neighbouring_points = star.neighbours();
            let neighbouring_parts = parts
                .iter()
                .filter(|part| {
                    part.points()
                        .iter()
                        .any(|p| neighbouring_points.contains(p))
                })
                .collect_vec();
            if neighbouring_parts.len() == 2 {
                Some(
                    neighbouring_parts
                        .iter()
                        .map(|p| p.number)
                        .product::<usize>(),
                )
            } else {
                None
            }
        })
        .sum()
}

fn valid_parts(symbols: &HashSet<Point>, numbers: &[Part]) -> Vec<Part> {
    numbers
        .iter()
        .filter(|part| part.neighbours().iter().any(|p| symbols.contains(p)))
        .cloned()
        .collect()
}

/// Return (all symbols, possible part numbers, possible gears)
fn parse_input(input: &str) -> (HashSet<Point>, Vec<Part>, Vec<Point>) {
    let mut symbols = HashSet::new();
    let mut numbers = Vec::new(); // potential parts
    let mut stars = Vec::new(); // potential gears

    input.lines().enumerate().for_each(|(y, line)| {
        let mut curr_part = None;
        line.chars().enumerate().for_each(|(x, c)| {
            if let Some(digit) = c.to_digit(10) {
                let mut n = curr_part.unwrap_or_else(|| Part { number: 0, x_min: x, x_max: x, y });
                n.x_max = x;
                n.number = n.number * 10 + (digit as usize);
                curr_part = Some(n);
            } else {
                if let Some(part) = curr_part {
                    numbers.push(part);
                    curr_part = None;
                }
                if c != '.' {
                    symbols.insert(Point { x, y });
                    if c == '*' {
                        stars.push(Point { x, y });
                    }
                }
            }
        });
        if let Some(part) = curr_part {
            numbers.push(part);
        }
    });

    (symbols, numbers, stars)
}

impl Part {
    fn neighbours(&self) -> Vec<Point> {
        // Note: we include neighbour points which exceed the max possible x and y values because they don't matter much
        let mut neighbours = Vec::new();
        if self.y > 0 && self.x_min > 0 {
            neighbours.push(Point { x: self.x_min - 1, y: self.y - 1 });
        }
        if self.y > 0 {
            for x in self.x_min..=(self.x_max + 1) {
                neighbours.push(Point { x, y: self.y - 1 });
            }
        }
        if self.x_min > 0 {
            neighbours.push(Point { x: self.x_min - 1, y: self.y });
            neighbours.push(Point { x: self.x_min - 1, y: self.y + 1 });
        }
        neighbours.push(Point { x: self.x_max + 1, y: self.y });
        for x in self.x_min..=(self.x_max + 1) {
            neighbours.push(Point { x, y: self.y + 1 });
        }
        neighbours
    }

    fn points(&self) -> Vec<Point> {
        (self.x_min..=self.x_max)
            .map(|x| Point { x, y: self.y })
            .collect()
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Part {
    number: usize,
    x_min: usize,
    x_max: usize,
    y: usize,
}

impl Point {
    fn neighbours(&self) -> Vec<Point> {
        // Note: we include neighbour points which exceed the max possible x and y values because they don't matter much
        let mut neighbours = vec![
            Point { x: self.x, y: self.y + 1 },
            Point { x: self.x + 1, y: self.y },
            Point { x: self.x + 1, y: self.y + 1 },
        ];
        if self.x > 0 && self.y > 0 {
            neighbours.push(Point { x: self.x - 1, y: self.y - 1 });
        }
        if self.x > 0 {
            neighbours.push(Point { x: self.x - 1, y: self.y });
            neighbours.push(Point { x: self.x - 1, y: self.y + 1 });
        }
        if self.y > 0 {
            neighbours.push(Point { x: self.x, y: self.y - 1 });
            neighbours.push(Point { x: self.x + 1, y: self.y - 1 });
        }
        neighbours
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_parse_input() {
        let (symbols, numbers, _) = parse_input(TEST_INPUT);
        println!("{:?}", symbols);
        println!("{:?}", numbers);
    }

    #[test]
    fn test_part_1() {
        let (symbols, numbers, stars) = parse_input(TEST_INPUT);
        assert_eq!(part_2(&symbols, &numbers, &stars), 467835);
    }
}
