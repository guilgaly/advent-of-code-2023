use common::itertools::Itertools;

static INPUT: &str = include_str!("input");

fn main() {
    let values = parse_input(INPUT);

    let (res1, res2) = part_1_and_2(&values);
    println!("Part 1 result: {}", res1);
    println!("Part 2 result: {}", res2);
}

fn part_1_and_2(values_list: &[Vec<i64>]) -> (i64, i64) {
    values_list
        .iter()
        .map(|values| {
            let mut last_numbers = Vec::new();
            let mut first_numbers = Vec::new();
            let mut curr_values = values.clone();
            while curr_values.iter().any(|&v| v != 0) {
                last_numbers.push(*curr_values.last().unwrap());
                first_numbers.push(*curr_values.first().unwrap());
                curr_values = curr_values
                    .iter()
                    .tuple_windows::<(_, _)>()
                    .map(|(&a, &b)| b - a)
                    .collect_vec();
            }
            (
                last_numbers.iter().rev().fold(0, |prev, next| next + prev),
                first_numbers.iter().rev().fold(0, |prev, next| next - prev),
            )
        })
        .fold((0, 0), |(acc1, acc2), (next1, next2)| {
            (acc1 + next1, acc2 + next2)
        })
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_1_and_2() {
        assert_eq!(part_1_and_2(&parse_input(TEST_INPUT)), (114, 2));
    }
}
