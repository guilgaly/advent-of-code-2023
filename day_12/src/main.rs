use common::itertools::Itertools;

static INPUT: &str = include_str!("input");

fn main() {
    let records = parse_input(INPUT);

    let res1 = part_1(&records);
    println!("Part 1 result: {}", res1);

    // let res2 = part_2(&records);
    // println!("Part 2 result: {}", res2);
}

fn part_1(records: &[Record]) -> usize {
    records
        .iter()
        .map(|record| count_arrangements(record))
        .sum()
}

// fn part_2(records: &[Record]) -> usize {
//     unfold(records).iter()
//         .map(|record| count_arrangements(record))
//         .sum()
// }

fn count_arrangements(Record { damaged_springs, damaged_groups }: &Record) -> usize {
    fn recurs(tmp: &mut Vec<bool>, unknowns: &[usize], damaged_groups: &[usize]) -> usize {
        if unknowns.is_empty() {
            if group_damaged_springs(tmp) == damaged_groups {
                1
            } else {
                0
            }
        } else {
            let (curr, rest) = unknowns.split_at(1);
            tmp[curr[0]] = false;
            let c1 = recurs(tmp, rest, damaged_groups);
            tmp[curr[0]] = true;
            let c2 = recurs(tmp, rest, damaged_groups);
            c1 + c2
        }
    }

    let unknowns = damaged_springs
        .iter()
        .enumerate()
        .filter_map(|(idx, v)| match v {
            None => Some(idx),
            _ => None,
        })
        .collect_vec();
    let mut tmp = damaged_springs
        .iter()
        .map(|v| match v {
            None => false,
            Some(true) => true,
            Some(false) => false,
        })
        .collect_vec();

    recurs(&mut tmp, &unknowns, damaged_groups)
}

fn group_damaged_springs(damaged_springs: &[bool]) -> Vec<usize> {
    let grouped = damaged_springs.into_iter().group_by(|&&damaged| damaged);
    grouped
        .into_iter()
        .filter_map(|(value, group)| if value { Some(group.count()) } else { None })
        .collect_vec()
}

fn parse_input(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let mut s = line.split_whitespace();
            let damaged_springs = s
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
            Record { damaged_springs, damaged_groups }
        })
        .collect_vec()
}

fn unfold(records: &[Record]) -> Vec<Record> {
    records
        .iter()
        .map(|record| {
            let mut damaged_springs = Vec::new();
            for i in 1..=5 {
                record
                    .damaged_springs
                    .iter()
                    .for_each(|&v| damaged_springs.push(v));
                if i != 5 {
                    damaged_springs.push(None);
                }
            }

            let mut damaged_groups = Vec::new();
            for _ in 1..=5 {
                record
                    .damaged_groups
                    .iter()
                    .for_each(|&g| damaged_groups.push(g));
            }

            Record { damaged_springs, damaged_groups }
        })
        .collect_vec()
}

struct Record {
    damaged_springs: Vec<Option<bool>>,
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

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&parse_input(TEST_INPUT)), 525152);
    // }
}
