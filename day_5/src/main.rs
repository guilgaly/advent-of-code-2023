use std::error::Error;
use std::num::ParseIntError;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let values = parse_input(INPUT)?;

    let res1 = part_1(&values);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&values);
    println!("Part 2 result: {}", res2);

    Ok(())
}

fn part_1(values: &[u64]) -> usize {
    values.len()
}

fn part_2(values: &[u64]) -> usize {
    values.len()
}

fn parse_input(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input.split("\n").map(|line| line.parse::<u64>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
}
