use common::itertools::Itertools;

static INPUT: &str = include_str!("input");

fn main() {
    let universe = parse_input(INPUT);

    let res1 = part_1(&universe);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&universe);
    println!("Part 2 result: {}", res2);
}

fn part_1(universe: &Universe) -> usize {
    total_distances(universe, 2)
}

fn part_2(universe: &Universe) -> usize {
    total_distances(universe, 1_000_000)
}

fn total_distances(universe: &Universe, empty_space_multiplier: usize) -> usize {
    let galaxies = universe.all_galaxies();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(idx, p1)| {
            galaxies
                .iter()
                .skip(idx + 1)
                .map(|p2| universe.distance(p1, p2, empty_space_multiplier))
        })
        .sum()
}

fn parse_input(input: &str) -> Universe {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let spaces = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '#' => Space::Galaxy,
                    _ => panic!("Invalid space {}", c),
                })
                .collect_vec()
        })
        .collect_vec();
    let empty_rows = (0..height)
        .filter(|&y| spaces[y].iter().all(|&s| s == Space::Empty))
        .collect_vec();
    let empty_columns = (0..width)
        .filter(|&x| spaces.iter().all(|row| row[x] == Space::Empty))
        .collect_vec();

    Universe { height, width, spaces, empty_rows, empty_columns }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Universe {
    fn all_galaxies(&self) -> Vec<Point> {
        let mut galaxies = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.spaces[y][x] == Space::Galaxy {
                    galaxies.push(Point { x, y });
                }
            }
        }
        galaxies
    }

    fn distance(&self, p1: &Point, p2: &Point, empty_space_multiplier: usize) -> usize {
        let x_min = p1.x.min(p2.x);
        let x_max = p1.x.max(p2.x);
        let horiz_dist = (x_min..x_max).fold(0, |acc, x| {
            if self.empty_columns.contains(&x) {
                acc + empty_space_multiplier
            } else {
                acc + 1
            }
        });

        let y_min = p1.y.min(p2.y);
        let y_max = p1.y.max(p2.y);
        let vert_dist = (y_min..y_max).fold(0, |acc, y| {
            if self.empty_rows.contains(&y) {
                acc + empty_space_multiplier
            } else {
                acc + 1
            }
        });

        horiz_dist + vert_dist
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Universe {
    height: usize,
    width: usize,
    spaces: Vec<Vec<Space>>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Space {
    Empty,
    Galaxy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let g = parse_input(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(g.empty_rows, vec![3, 7]);
        assert_eq!(g.empty_columns, vec![2, 5, 8]);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(&parse_input(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            )),
            374
        );
    }
}
