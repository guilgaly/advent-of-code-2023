use common::itertools::Itertools;
use sscanf::sscanf;

static INPUT: &str = include_str!("input");

fn main() {
    let games = parse_input(INPUT);

    let res1 = part_1(&games);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&games);
    println!("Part 2 result: {}", res2);
}

fn part_1(games: &[Game]) -> usize {
    games
        .iter()
        .filter_map(|game| {
            let possible = game
                .draws
                .iter()
                .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14);
            if possible {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(games: &[Game]) -> usize {
    games
        .iter()
        .map(|game| {
            // Construct the minimum set for this game
            let red = game.draws.iter().map(|d| d.red).max().unwrap_or_else(|| 0);
            let green = game
                .draws
                .iter()
                .map(|d| d.green)
                .max()
                .unwrap_or_else(|| 0);
            let blue = game.draws.iter().map(|d| d.blue).max().unwrap_or_else(|| 0);
            red * green * blue
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .split("\n")
        .map(|line| {
            let (id, draws_str) = sscanf!(line, "Game {usize}: {str}").unwrap();
            let draws = draws_str
                .split("; ")
                .map(|draw_str| {
                    let mut draw = SetOfCubes::empty();
                    draw_str.split(", ").for_each(|color_draw| {
                        let (value, color) = sscanf!(color_draw, "{usize} {str}").unwrap();
                        if color == "red" {
                            draw.red = value;
                        } else if color == "green" {
                            draw.green = value;
                        } else if color == "blue" {
                            draw.blue = value;
                        }
                    });
                    draw
                })
                .collect_vec();
            Game { id, draws }
        })
        .collect()
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Game {
    id: usize,
    draws: Vec<SetOfCubes>,
}

impl SetOfCubes {
    fn empty() -> SetOfCubes {
        SetOfCubes { red: 0, green: 0, blue: 0 }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct SetOfCubes {
    red: usize,
    green: usize,
    blue: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_GAMES: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(TEST_GAMES)), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(TEST_GAMES)), 2286);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_GAMES),
            vec![
                Game {
                    id: 1,
                    draws: vec![
                        SetOfCubes { red: 4, green: 0, blue: 3 },
                        SetOfCubes { red: 1, green: 2, blue: 6 },
                        SetOfCubes { red: 0, green: 2, blue: 0 }
                    ]
                },
                Game {
                    id: 2,
                    draws: vec![
                        SetOfCubes { red: 0, green: 2, blue: 1 },
                        SetOfCubes { red: 1, green: 3, blue: 4 },
                        SetOfCubes { red: 0, green: 1, blue: 1 }
                    ]
                },
                Game {
                    id: 3,
                    draws: vec![
                        SetOfCubes { red: 20, green: 8, blue: 6 },
                        SetOfCubes { red: 4, green: 13, blue: 5 },
                        SetOfCubes { red: 1, green: 5, blue: 0 }
                    ]
                },
                Game {
                    id: 4,
                    draws: vec![
                        SetOfCubes { red: 3, green: 1, blue: 6 },
                        SetOfCubes { red: 6, green: 3, blue: 0 },
                        SetOfCubes { red: 14, green: 3, blue: 15 }
                    ]
                },
                Game {
                    id: 5,
                    draws: vec![
                        SetOfCubes { red: 6, green: 3, blue: 1 },
                        SetOfCubes { red: 1, green: 2, blue: 2 }
                    ]
                },
            ]
        );
    }
}
