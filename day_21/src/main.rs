use common::itertools::Itertools;
use common::maplit::hashset;

static INPUT: &str = include_str!("input");

fn main() {
    let garden = parse_input(INPUT);

    let res1 = part_1(&garden, 64);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&garden, 26_501_365);
    println!("Part 2 result: {}", res2);
}

fn part_1(garden: &Garden, steps: usize) -> usize {
    fn accessible_neighbours(garden: &Garden, Point { x, y }: Point) -> Vec<Point> {
        [
            Point { x: x - 1, y },
            Point { x: x + 1, y },
            Point { x, y: y - 1 },
            Point { x, y: y + 1 },
        ].into_iter().filter(|p| garden.is_open(p)).collect_vec()
    }

    let mut positions = hashset! {garden.start_plot};
    for _ in 0..steps {
        let prev_positions = positions.clone();
        positions.clear();
        for current_plot in prev_positions {
            for next_plot in accessible_neighbours(garden, current_plot) {
                positions.insert(next_plot);
            }
        }
    }
    positions.len()
}

fn part_2(garden: &Garden, steps: usize) -> f64 {
    let y0 = part_1(garden, garden.size / 2) as f64;
    let y1 = part_1(garden, (garden.size / 2) + garden.size) as f64;
    let y2 = part_1(garden, (garden.size / 2) + (garden.size * 2)) as f64;

    // Lagrange Interpolation
    let a =  (y0 / 2.) - y1 + (y2 / 2.);
    let b =  -3. * (y0 / 2.) + (2. * y1) - (y2 / 2.);
    let c = y0;

    let target = ((steps - (garden.size / 2)) / garden.size) as f64;

    a * target * target + b * target + c
}

fn parse_input(input: &str) -> Garden {
    let mut start_plot = Point { x: 0, y: 0 };
    let open_plots = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start_plot = Point { x: x as i64, y: y as i64 };
                    }
                    c != '#'
                })
                .collect_vec()
        })
        .collect_vec();
    let size = open_plots.len();
    Garden { size, open_plots, start_plot }
}

impl Garden {
    fn is_open(&self, p: &Point) -> bool {
        let size = self.size as i64;

        let x = p.x % size;
        let x = if x >= 0 { x } else { size + x };

        let y = p.y % size;
        let y = if y >= 0 { y } else { size + y };

        self.open_plots[y as usize][x as usize]
    }
}

struct Garden {
    size: usize,
    open_plots: Vec<Vec<bool>>,
    start_plot: Point,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(TEST_INPUT), 6), 16);
        assert_eq!(part_1(&parse_input(TEST_INPUT), 10), 50);
        assert_eq!(part_1(&parse_input(TEST_INPUT), 50), 1594);
        // assert_eq!(part_1(&parse_input(TEST_INPUT), 100), 6536);
        // assert_eq!(part_1(&parse_input(TEST_INPUT), 500), 167004);
        // assert_eq!(part_1(&parse_input(TEST_INPUT), 1000), 668697);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(TEST_INPUT), 5000), 16733044);
    }
}
