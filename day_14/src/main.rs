use common::itertools::Itertools;
use common::time_execution;

static INPUT: &str = include_str!("input");

fn main() {
    let platform = parse_input(INPUT);

    let res1 = part_1(&platform);
    println!("Part 1 result: {}", res1);

    let res2 = time_execution("part 2", || part_2(&platform));
    println!("Part 2 result: {}", res2);
}

fn part_1(platform: &Platform) -> usize {
    let mut p = platform.clone();
    tilt_north(&mut p);
    measure_load(&p)
}

fn part_2(platform: &Platform) -> usize {
    fn find_loop(platform: &Platform) -> (Vec<Platform>, usize) {
        let mut steps = vec![platform.clone()];
        loop {
            let next_p = next_cycle(steps.last().unwrap());
            if let Some((j, _)) = steps.iter().find_position(|&p| p == &next_p) {
                let loop_steps = steps.into_iter().skip(j).collect_vec();
                return (loop_steps, j);
            } else {
                steps.push(next_p);
            }
        }
    }

    let (loop_steps, loop_start) = find_loop(platform);
    let loop_length = loop_steps.len();
    println!("loop_start={}", loop_start);
    println!("loop_length={}", loop_steps.len());

    // number of cycles = loop_start + loop_iterations * loop_length + remainder
    // To know the end state, we just need to calculate the remainder to know where we are in the
    // loop when we stop.
    let effective_total_length = 1_000_000_000 - loop_start;
    let loop_iterations = effective_total_length / loop_length;
    let remainder = effective_total_length % loop_length;
    println!("loop_iter={}", loop_iterations);
    println!("remainder={}", remainder);

    let last_iter = &loop_steps[remainder];
    measure_load(last_iter)
}

fn measure_load(platform: &Platform) -> usize {
    platform
        .tiles
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let multiplier = platform.height - y;
            row.iter().filter(|&&t| t == Tile::RoundRock).count() * multiplier
        })
        .sum()
}

fn next_cycle(platform: &Platform) -> Platform {
    let mut p = platform.clone();
    tilt_north(&mut p);
    tilt_west(&mut p);
    tilt_south(&mut p);
    tilt_east(&mut p);
    p
}

fn tilt_north(p: &mut Platform) {
    for y in 0..p.height {
        for x in 0..p.width {
            if p.tiles[y][x] == Tile::RoundRock {
                let mut new_y = y;
                while new_y > 0 && p.tiles[new_y - 1][x] == Tile::Empty {
                    new_y -= 1;
                }
                if new_y != y {
                    p.tiles[new_y][x] = Tile::RoundRock;
                    p.tiles[y][x] = Tile::Empty;
                }
            }
        }
    }
}

fn tilt_west(p: &mut Platform) {
    for x in 0..p.width {
        for y in 0..p.height {
            if p.tiles[y][x] == Tile::RoundRock {
                let mut new_x = x;
                while new_x > 0 && p.tiles[y][new_x - 1] == Tile::Empty {
                    new_x -= 1;
                }
                if new_x != x {
                    p.tiles[y][new_x] = Tile::RoundRock;
                    p.tiles[y][x] = Tile::Empty;
                }
            }
        }
    }
}

fn tilt_south(p: &mut Platform) {
    for y in (0..p.height).rev() {
        for x in 0..p.width {
            if p.tiles[y][x] == Tile::RoundRock {
                let mut new_y = y;
                while new_y < p.height - 1 && p.tiles[new_y + 1][x] == Tile::Empty {
                    new_y += 1;
                }
                if new_y != y {
                    p.tiles[new_y][x] = Tile::RoundRock;
                    p.tiles[y][x] = Tile::Empty;
                }
            }
        }
    }
}

fn tilt_east(p: &mut Platform) {
    for x in (0..p.width).rev() {
        for y in 0..p.height {
            if p.tiles[y][x] == Tile::RoundRock {
                let mut new_x = x;
                while new_x < p.width - 1 && p.tiles[y][new_x + 1] == Tile::Empty {
                    new_x += 1;
                }
                if new_x != x {
                    p.tiles[y][new_x] = Tile::RoundRock;
                    p.tiles[y][x] = Tile::Empty;
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Platform {
    let tiles = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Tile::RoundRock,
                    '#' => Tile::CubeRock,
                    _ => Tile::Empty,
                })
                .collect_vec()
        })
        .collect_vec();
    let height = tiles.len();
    let width = tiles[0].len();
    Platform { height, width, tiles }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Platform {
    height: usize,
    width: usize,
    tiles: Vec<Vec<Tile>>,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(TEST_INPUT)), 136);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(TEST_INPUT)), 64);
    }

    #[test]
    fn test_cycle() {
        let p0 = parse_input(TEST_INPUT);
        let p1 = next_cycle(&p0);
        let p2 = next_cycle(&p1);
        let p3 = next_cycle(&p2);

        assert_eq!(
            p1,
            parse_input(
                ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
            )
        );

        assert_eq!(
            p2,
            parse_input(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
            )
        );

        assert_eq!(
            p3,
            parse_input(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
            )
        );
    }
}
