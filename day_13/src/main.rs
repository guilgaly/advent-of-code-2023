use common::itertools::Itertools;

static INPUT: &str = include_str!("input");

fn main() {
    let patterns = parse_input(INPUT);

    let res1 = part_1(&patterns);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&patterns);
    println!("Part 2 result: {}", res2);
}

fn part_1(patterns: &[Pattern]) -> usize {
    patterns
        .iter()
        .flat_map(|pattern| find_symmetries(pattern))
        .sum()
}

fn part_2(patterns: &[Pattern]) -> usize {
    patterns
        .iter()
        .map(|pattern| find_smudged_symmetry(pattern))
        .sum()
}

fn find_smudged_symmetry(pattern: &Pattern) -> usize {
    let initial_symmetries = find_symmetries(pattern);

    let mut p = pattern.clone();
    for y in 0..p.height {
        for x in 0..p.width {
            let init_tile = p.tiles[y][x];
            p.tiles[y][x] = !init_tile;
            let new_symmetries = find_symmetries(&p)
                .into_iter()
                .filter(|s| !initial_symmetries.contains(s))
                .collect_vec();
            if !new_symmetries.is_empty() {
                return new_symmetries[0];
            }
            // reset modified tile before trying another one
            p.tiles[y][x] = init_tile;
        }
    }
    // println!("NOT FOUND");
    // println!("initial symmetry: {}", initial_symmetry);
    // print_pattern(pattern);
    panic!();
}

fn find_symmetries(pattern: &Pattern) -> Vec<usize> {
    find_horiz_symmetry_line(pattern)
        .into_iter()
        .map(|y| 100 * (y + 1))
        .chain(
            find_horiz_symmetry_line(&pattern.transpose())
                .into_iter()
                .map(|x| x + 1),
        )
        .collect()
}

fn find_horiz_symmetry_line(pattern: &Pattern) -> Vec<usize> {
    let mut res = Vec::new();
    for y in 0..(pattern.height - 1) {
        let mut offset = 0;
        let mut is_symmetry = false;
        while (offset <= y) && (y + offset <= pattern.height - 2) {
            let up = &pattern.tiles[y - offset];
            let down = &pattern.tiles[y + offset + 1];
            is_symmetry = up == down;
            if !is_symmetry {
                break;
            } else {
                offset += 1;
            }
        }
        if is_symmetry {
            res.push(y);
        }
    }
    res
}

fn parse_input(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pattern_str| parse_pattern(pattern_str))
        .collect()
}

fn parse_pattern(pattern_str: &str) -> Pattern {
    let tiles = pattern_str
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();
    let height = tiles.len();
    let width = tiles[0].len();
    Pattern { height, width, tiles }
}

impl Pattern {
    fn transpose(&self) -> Pattern {
        let height = self.width;
        let width = self.height;
        let mut tiles = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(self.tiles[x][y]);
            }
            tiles.push(row);
        }
        Pattern { height, width, tiles }
    }
}

fn print_pattern(pattern: &Pattern) {
    for row in pattern.tiles.iter() {
        for &tile in row {
            if tile {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

#[derive(Clone)]
struct Pattern {
    height: usize,
    width: usize,
    tiles: Vec<Vec<bool>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    //     #[test]
    //     fn test_horiz() {
    //         assert_eq!(
    //             find_horiz_symmetry_line(&parse_pattern(
    //                 "#...##..#
    // #....#..#
    // ..##..###
    // #####.##.
    // #####.##.
    // ..##..###
    // #....#..#"
    //             )),
    //             Some(3)
    //         );
    //     }
    //
    //     #[test]
    //     fn test_vert() {
    //         assert_eq!(
    //             find_horiz_symmetry_line(
    //                 &parse_pattern(
    //                     "#.##..##.
    // ..#.##.#.
    // ##......#
    // ##......#
    // ..#.##.#.
    // ..##..##.
    // #.#.##.#."
    //                 )
    //                 .transpose()
    //             ),
    //             Some(4)
    //         );
    //     }

    //     #[test]
    //     fn test_smudged_1() {
    //         assert_eq!(
    //             find_smudged_symmetry(&parse_pattern(
    //                 "#.##..##.
    // ..#.##.#.
    // ##......#
    // ##......#
    // ..#.##.#.
    // ..##..##.
    // #.#.##.#.")),
    //             300
    //         );
    //     }
    //
    //     #[test]
    //     fn test_smudged_2() {
    //         assert_eq!(
    //             find_smudged_symmetry(&parse_pattern(
    //                 "#...##..#
    // #....#..#
    // ..##..###
    // #####.##.
    // #####.##.
    // ..##..###
    // #....#..#")),
    //             100
    //         );
    //     }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(TEST_INPUT)), 405);
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&parse_input(TEST_INPUT)), 400);
    // }
}
