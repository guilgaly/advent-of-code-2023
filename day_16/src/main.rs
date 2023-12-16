use common::itertools::Itertools;
use common::maplit::hashset;
use common::time_execution;
use std::collections::HashSet;

static INPUT: &str = include_str!("input");

fn main() {
    let cave = parse_input(INPUT);

    let res1 = time_execution("Part 1", || part_1(&cave));
    println!("Part 1 result: {}", res1);

    let res2 = time_execution("Part 2", || part_2(&cave));
    println!("Part 2 result: {}", res2);
}

fn part_1(cave: &Cave) -> usize {
    count_energized_tiles(cave, (Point::new(0, 0), Direction::Right))
}

fn part_2(cave: &Cave) -> usize {
    (0..cave.width)
        .map(|x| (Point::new(x, 0), Direction::Down))
        .chain((0..cave.width).map(|x| (Point::new(x, cave.height - 1), Direction::Up)))
        .chain((0..cave.height).map(|y| (Point::new(0, y), Direction::Right)))
        .chain((0..cave.height).map(|y| (Point::new(cave.width - 1, y), Direction::Left)))
        .map(|start| count_energized_tiles(&cave, start))
        .max()
        .unwrap()
}

fn count_energized_tiles(cave: &Cave, start: (Point, Direction)) -> usize {
    let mut energized = hashset! { start };
    let mut current = vec![start];
    while !current.is_empty() {
        let mut new_current = Vec::new();

        let mut add_next = |next_point: Point, next_dir: Direction| {
            let next_value = (next_point, next_dir);
            if energized.insert(next_value) {
                new_current.push(next_value);
            }
        };

        let mut calculate_and_add_next = |p: Point, next_dir: Direction| {
            match next_dir {
                Direction::Left if p.x > 0 => add_next(Point::new(p.x - 1, p.y), next_dir),
                Direction::Right if p.x < cave.width - 1 => {
                    add_next(Point::new(p.x + 1, p.y), next_dir)
                }
                Direction::Up if p.y > 0 => add_next(Point::new(p.x, p.y - 1), next_dir),
                Direction::Down if p.y < cave.height - 1 => {
                    add_next(Point::new(p.x, p.y + 1), next_dir)
                }
                _ => {}
            };
        };

        for (p, dir) in current {
            match cave.tiles[p.y][p.x] {
                Tile::Empty => calculate_and_add_next(p, dir),
                Tile::Mirror => match dir {
                    Direction::Left => calculate_and_add_next(p, Direction::Down),
                    Direction::Right => calculate_and_add_next(p, Direction::Up),
                    Direction::Up => calculate_and_add_next(p, Direction::Right),
                    Direction::Down => calculate_and_add_next(p, Direction::Left),
                },
                Tile::AntiMirror => match dir {
                    Direction::Left => calculate_and_add_next(p, Direction::Up),
                    Direction::Right => calculate_and_add_next(p, Direction::Down),
                    Direction::Up => calculate_and_add_next(p, Direction::Left),
                    Direction::Down => calculate_and_add_next(p, Direction::Right),
                },
                Tile::HorizSplit => match dir {
                    Direction::Left | Direction::Right => calculate_and_add_next(p, dir),
                    Direction::Up | Direction::Down => {
                        calculate_and_add_next(p, Direction::Left);
                        calculate_and_add_next(p, Direction::Right);
                    }
                },
                Tile::VertSplit => match dir {
                    Direction::Left | Direction::Right => {
                        calculate_and_add_next(p, Direction::Up);
                        calculate_and_add_next(p, Direction::Down);
                    }
                    Direction::Up | Direction::Down => calculate_and_add_next(p, dir),
                },
            }
        }
        current = new_current;
    }

    let energized_points: HashSet<_> = energized.iter().map(|(p, _)| *p).collect();
    energized_points.len()
}

fn parse_input(input: &str) -> Cave {
    let tiles = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '/' => Tile::Mirror,
                    '\\' => Tile::AntiMirror,
                    '-' => Tile::HorizSplit,
                    '|' => Tile::VertSplit,
                    _ => Tile::Empty,
                })
                .collect_vec()
        })
        .collect_vec();
    let height = tiles.len();
    let width = tiles[0].len();
    Cave { height, width, tiles }
}

struct Cave {
    height: usize,
    width: usize,
    tiles: Vec<Vec<Tile>>,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Empty,      // .
    Mirror,     // /
    AntiMirror, // \
    HorizSplit, // -
    VertSplit,  // |
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(TEST_INPUT)), 46);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(TEST_INPUT)), 51);
    }
}
