use common::itertools::Itertools;
use common::maplit::hashmap;
use common::time_execution;
use std::collections::HashMap;

static INPUT: &str = include_str!("input");

fn main() {
    let records = parse_input(INPUT);

    let res1 = time_execution("part 1", || part_1(&records));
    println!("Part 1 result: {}", res1);

    let res2 = time_execution("part 2", || part_2(&records));
    println!("Part 2 result: {}", res2);
}

fn part_1(records: &[Record]) -> usize {
    records
        .iter()
        .map(|record| count_arrangements(record))
        .sum()
}

fn part_2(records: &[Record]) -> usize {
    records
        .iter()
        .map(|record| count_arrangements(&unfold(record)))
        .sum()
}

fn count_arrangements(Record { springs, damaged_groups }: &Record) -> usize {
    // Before we start, we haven't yet counted any spring within the first group, and there is only one permutation possible for this
    let init_permutations = hashmap! { DamagedGroup {idx: 0, amount: 0} => 1 };
    springs
        .iter()
        .fold(init_permutations, |acc, &spring| {
            let mut permutation_counts = HashMap::new();
            let mut increment_permutations = |idx: usize, amount: usize, permutations: usize| {
                let group_amount = DamagedGroup { idx, amount };
                *permutation_counts.entry(group_amount).or_default() += permutations;
            };

            match spring {
                None => {
                    for (DamagedGroup { idx, amount }, permutations) in acc {
                        // amount = 0: we can add a damaged spring (1) or an undamaged spring (2)
                        // 0 < amount < group_size: we're in the middle of a group, we can only add a damaged spring (1)
                        // amount = group_size: we've reached the end of a group, we can only add an undamaged spring and must increment the index (3)

                        if idx < damaged_groups.len() && amount < damaged_groups[idx] {
                            // 1
                            increment_permutations(idx, amount + 1, permutations);
                        }

                        if amount == 0 {
                            // 2
                            increment_permutations(idx, 0, permutations);
                        } else if amount == damaged_groups[idx] {
                            // 3
                            increment_permutations(idx + 1, 0, permutations);
                        }
                    }
                }
                Some(damaged_spring) => {
                    for (DamagedGroup { idx, amount }, permutations) in acc {
                        if damaged_spring
                            && idx < damaged_groups.len()
                            && amount < damaged_groups[idx]
                        {
                            // One more damaged spring in the current group
                            increment_permutations(idx, amount + 1, permutations);
                        } else if !damaged_spring && amount == 0 {
                            // One more undamaged spring (and we're not in the middle of a group of damaged springs)
                            increment_permutations(idx, 0, permutations);
                        } else if !damaged_spring && amount == damaged_groups[idx] {
                            // The current group of damaged springs ends with this undamaged spring
                            increment_permutations(idx + 1, 0, permutations);
                        }
                    }
                }
            };

            permutation_counts
        })
        .iter()
        .filter_map(|(&DamagedGroup { idx, amount }, &permutations)| {
            // We count only the cases where we correctly reached the end:
            // - we went past the last group of damaged springs and didn't count any more damaged spring
            // - we're on the last group of damaged springs and we have the expected amount of those
            if (idx == damaged_groups.len() - 1 && amount == *damaged_groups.last().unwrap())
                || (idx == damaged_groups.len() && amount == 0)
            {
                Some(permutations)
            } else {
                None
            }
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let mut s = line.split_whitespace();
            let springs = s
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '#' => Some(true),
                    '.' => Some(false),
                    '?' => None,
                    _ => panic!("Illegal spring state {}", c),
                })
                .collect_vec();
            let damaged_groups = s
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();
            Record { springs, damaged_groups }
        })
        .collect_vec()
}

fn unfold(record: &Record) -> Record {
    let mut springs = Vec::new();
    for i in 1..=5 {
        record.springs.iter().for_each(|&v| springs.push(v));
        if i != 5 {
            springs.push(None);
        }
    }

    let mut damaged_groups = Vec::new();
    for _ in 1..=5 {
        record
            .damaged_groups
            .iter()
            .for_each(|&g| damaged_groups.push(g));
    }

    Record { springs, damaged_groups }
}

/// idx: index of this group of broken springs
/// amount: number of broken springs already counted in this group
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct DamagedGroup {
    idx: usize,
    amount: usize,
}

struct Record {
    springs: Vec<Option<bool>>,
    damaged_groups: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(TEST_INPUT)), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(TEST_INPUT)), 525152);
    }
}
