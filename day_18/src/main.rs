use common::itertools::Itertools;
use common::maplit::hashset;
use sscanf::sscanf;
use std::collections::HashSet;

static INPUT: &str = include_str!("input");

fn main() {
    let digs = parse_input(INPUT);

    let res1 = part_1(&digs);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&digs);
    println!("Part 2 result: {}", res2);
}

fn part_1(digs: &[Dig]) -> usize {
    let mut curr_point = Point::new(0, 0);
    let mut border = hashset![curr_point];
    for dig in digs {
        for _ in 0..dig.length {
            curr_point = curr_point.next(&dig.direction);
            border.insert(curr_point);
        }
    }
    let min_x = border.iter().map(|p| p.x).min().unwrap();
    let max_x = border.iter().map(|p| p.x).max().unwrap();
    let min_y = border.iter().map(|p| p.y).min().unwrap();
    let max_y = border.iter().map(|p| p.y).max().unwrap();
    let trench = Trench { border, min_x, max_x, min_y, max_y };

    println!("min_x={}, max_x={}", min_x, max_x);
    println!("min_y={}, max_y={}", min_y, max_y);

    // Scan line by line
    let mut count = 0;
    for y in (trench.min_y..=trench.max_y).rev() {
        let mut is_inside = false;
        let mut is_on_wall: Option<bool> = None; // Some(true) means the wall was coming from below, Some(false) it was coming from above
        for x in trench.min_x..=(trench.max_x + 1) {
            if trench.border_contains(x, y) {
                print!("1");
                count += 1;
                if trench.border_contains(x, y - 1) && trench.border_contains(x, y + 1) {
                    is_inside = !is_inside;
                    is_on_wall = None;
                } else if is_on_wall.is_none() {
                    if trench.border_contains(x, y - 1) {
                        is_on_wall = Some(true);
                    } else {
                        is_on_wall = Some(false);
                    }
                }
            } else {
                if let Some(from_below) = is_on_wall {
                    if from_below != trench.border_contains(x, y - 1) {
                        is_inside = !is_inside;
                    }
                    is_on_wall = None;
                }
                if is_inside {
                    print!("1");
                    count += 1;
                } else {
                    print!("0");
                }
            }
        }
        println!();
    }

    count
}

fn part_2(digs: &[Dig]) -> usize {
    0
}

fn parse_input(input: &str) -> Vec<Dig> {
    input
        .lines()
        .map(|line| {
            let (dir_char, length, color) = sscanf!(line, "{char} {i64} (#{String})").unwrap();
            let direction = match dir_char {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction {}", dir_char),
            };
            Dig { direction, length, color }
        })
        .collect_vec()
}

impl Trench {
    fn border_contains(&self, x: i64, y: i64) -> bool {
        self.border.contains(&Point::new(x, y))
    }
}

struct Trench {
    border: HashSet<Point>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn next(&self, dir: &Direction) -> Point {
        match dir {
            Direction::Up => Point::new(self.x, self.y + 1),
            Direction::Down => Point::new(self.x, self.y - 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

struct Dig {
    direction: Direction,
    length: i64,
    color: String,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(TEST_INPUT)), 62);
    }
}
