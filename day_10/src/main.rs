use common::itertools::Itertools;
use std::collections::HashMap;

static INPUT: &str = include_str!("input");

fn main() {
    let pipes_map = parse_input(INPUT);

    let res1 = part_1(&pipes_map);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&pipes_map);
    println!("Part 2 result: {}", res2);
}

fn part_1(pipes_map: &PipesMap) -> usize {
    trace_loop(pipes_map).len() / 2
}

fn part_2(pipes_map: &PipesMap) -> usize {
    let pipes_loop = trace_loop(pipes_map);

    let mut inside_points_count = 0;
    for y in 0..pipes_map.size {
        for x in 0..pipes_map.size {
            // Don't count points *on* the loop
            if (x < pipes_map.size - 1) && !pipes_loop.contains_key(&Point { x, y }) {
                // count intersections until a point to the right, just outside the map (which means it's also outside the loop)
                let mut intersections_count = 0;
                let mut prev = TileType::Ground;
                for x2 in (x + 1)..pipes_map.size {
                    if let Some(&tile) = pipes_loop.get(&Point { x: x2, y }) {
                        if tile == TileType::UpDown {
                            intersections_count += 1;
                        } else if tile == TileType::DownRight || tile == TileType::UpRight {
                            prev = tile;
                        } else if (tile == TileType::DownLeft && prev == TileType::UpRight)
                            || (tile == TileType::UpLeft && prev == TileType::DownRight)
                        {
                            intersections_count += 1;
                        }
                    }
                }

                // an odd number of intersections means the point is inside the loop
                if intersections_count % 2 != 0 {
                    inside_points_count += 1;
                }
            }
        }
    }
    inside_points_count
}

fn trace_loop(pipes_map: &PipesMap) -> HashMap<Point, TileType> {
    let mut pipes_loop = HashMap::new();

    let mut prev_point = pipes_map.start;
    pipes_loop.insert(prev_point, pipes_map.get_tile(&prev_point));
    let mut curr_point = pipes_map.get_connected_points(&pipes_map.start)[0];
    pipes_loop.insert(curr_point, pipes_map.get_tile(&curr_point));

    let mut size = pipes_loop.len();
    loop {
        let new_point = pipes_map
            .get_connected_points(&curr_point)
            .into_iter()
            .find(|&p| p != prev_point)
            .unwrap();
        prev_point = curr_point;
        curr_point = new_point;
        pipes_loop.insert(curr_point, pipes_map.get_tile(&curr_point));
        if pipes_loop.len() == size {
            break;
        } else {
            size += 1;
        }
    }

    pipes_loop
}

fn parse_input(input: &str) -> PipesMap {
    let size = input.lines().next().unwrap().len();
    let mut start = Point { x: 0, y: 0 };
    let mut tiles = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '|' => TileType::UpDown,
                    '-' => TileType::LeftRight,
                    'L' => TileType::UpRight,
                    'J' => TileType::UpLeft,
                    '7' => TileType::DownLeft,
                    'F' => TileType::DownRight,
                    '.' => TileType::Ground,
                    'S' => {
                        start = Point { x, y };
                        TileType::Ground // temporary
                    }
                    _ => panic!("Invalid char {}", c),
                })
                .collect_vec()
        })
        .collect_vec();

    let Point { x, y } = start;
    let connected_up = y > 0 && {
        let t = tiles[y - 1][x];
        t == TileType::UpDown || t == TileType::DownLeft || t == TileType::DownRight
    };
    let connected_right = x < size - 1 && {
        let t = tiles[y][x + 1];
        t == TileType::LeftRight || t == TileType::UpLeft || t == TileType::DownLeft
    };
    let connected_down = y < size - 1 && {
        let t = tiles[y + 1][x];
        t == TileType::UpDown || t == TileType::UpLeft || t == TileType::UpRight
    };
    let connected_left = x > 0 && {
        let t = tiles[y][x - 1];
        t == TileType::LeftRight || t == TileType::UpRight || t == TileType::DownRight
    };

    let start_type = if connected_left && connected_right {
        TileType::LeftRight
    } else if connected_up && connected_down {
        TileType::UpDown
    } else if connected_up && connected_right {
        TileType::UpRight
    } else if connected_up && connected_right {
        TileType::UpLeft
    } else if connected_down && connected_left {
        TileType::DownLeft
    } else if connected_down && connected_right {
        TileType::DownRight
    } else {
        panic!("start point {:?} is not connected", start);
    };
    tiles[start.y][start.x] = start_type;

    PipesMap { size, tiles, start }
}

impl PipesMap {
    fn get_tile(&self, coords: &Point) -> TileType {
        self.tiles[coords.y][coords.x]
    }

    fn get_connected_points(&self, coords: &Point) -> Vec<Point> {
        let &Point { x, y } = coords;

        let add_up = |v: &mut Vec<Point>| {
            if y > 0 {
                v.push(Point { x, y: y - 1 });
            }
        };
        let add_right = |v: &mut Vec<Point>| {
            if x < self.size - 1 {
                v.push(Point { x: x + 1, y });
            }
        };
        let add_down = |v: &mut Vec<Point>| {
            if y < self.size - 1 {
                v.push(Point { x, y: y + 1 });
            }
        };
        let add_left = |v: &mut Vec<Point>| {
            if x > 0 {
                v.push(Point { x: x - 1, y });
            }
        };

        match self.get_tile(coords) {
            TileType::UpDown => {
                let mut connected = Vec::new();
                add_up(&mut connected);
                add_down(&mut connected);
                connected
            }
            TileType::LeftRight => {
                let mut connected = Vec::new();
                add_left(&mut connected);
                add_right(&mut connected);
                connected
            }
            TileType::UpRight => {
                let mut connected = Vec::new();
                add_up(&mut connected);
                add_right(&mut connected);
                connected
            }
            TileType::UpLeft => {
                let mut connected = Vec::new();
                add_up(&mut connected);
                add_left(&mut connected);
                connected
            }
            TileType::DownLeft => {
                let mut connected = Vec::new();
                add_down(&mut connected);
                add_left(&mut connected);
                connected
            }
            TileType::DownRight => {
                let mut connected = Vec::new();
                add_down(&mut connected);
                add_right(&mut connected);
                connected
            }
            TileType::Ground => Vec::new(),
        }
    }
}

struct PipesMap {
    size: usize,
    tiles: Vec<Vec<TileType>>,
    start: Point,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum TileType {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    Ground,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input_1 = parse_input(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(part_1(&test_input_1), 4);

        let test_input_2 = parse_input(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(part_1(&test_input_2), 8);
    }

    #[test]
    fn test_part_2() {
        let test_input_1 = parse_input(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
...........
...........",
        );
        assert_eq!(part_2(&test_input_1), 4);

        let test_input_2 = parse_input(
            "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
..........
..........",
        );
        assert_eq!(part_2(&test_input_2), 4);

        let test_input_3 = parse_input(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................",
        );
        assert_eq!(part_2(&test_input_3), 8);

        let test_input_4 = parse_input(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................",
        );
        assert_eq!(part_2(&test_input_4), 10);
    }
}
