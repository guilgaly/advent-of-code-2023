use common::itertools::Itertools;
use sscanf::sscanf;
use std::cmp::min;
use std::collections::HashSet;

static INPUT: &str = include_str!("input");

fn main() {
    let scratchcards = parse_input(INPUT);

    let res1 = part_1(&scratchcards);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&scratchcards);
    println!("Part 2 result: {}", res2);
}

fn part_1(scratchcards: &[Scratchcard]) -> usize {
    scratchcards.iter().map(|g| g.points()).sum()
}

fn part_2(scratchcards: &[Scratchcard]) -> usize {
    let mut all_cards = scratchcards.iter().map(|c| (c, 1usize)).collect_vec();
    let last_card_idx = all_cards.len() - 1;

    for idx in 0..=last_card_idx {
        let (card, count) = all_cards[idx];
        let m = card.matches();
        if m > 0 && idx < last_card_idx {
            let idx_min = idx + 1;
            let idx_max = min(idx + m, last_card_idx);
            for i in idx_min..=idx_max {
                all_cards[i].1 += count;
            }
        }
    }

    all_cards.iter().map(|(_, count)| count).sum()
}

fn parse_input(input: &str) -> Vec<Scratchcard> {
    input
        .lines()
        .map(|line| {
            let (_, winning_nbrs_str, nbrs_you_have_str) =
                sscanf!(line, "{str}: {str} | {str}").unwrap();
            let winning_nbrs = parse_numbers(winning_nbrs_str);
            let nbrs_you_have = parse_numbers(nbrs_you_have_str);
            Scratchcard { winning_nbrs, nbrs_you_have }
        })
        .collect()
}

fn parse_numbers(input: &str) -> HashSet<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

impl Scratchcard {
    fn matches(&self) -> usize {
        self.winning_nbrs.intersection(&self.nbrs_you_have).count()
    }
    fn points(&self) -> usize {
        let m = self.matches() as u32;
        if m == 0 {
            0
        } else {
            usize::pow(2, m - 1)
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Scratchcard {
    winning_nbrs: HashSet<usize>,
    nbrs_you_have: HashSet<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(TEST_INPUT)), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(TEST_INPUT)), 30);
    }
}
