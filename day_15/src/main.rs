use common::itertools::Itertools;
use sscanf::sscanf;
use std::array;

static INPUT: &str = include_str!("input");

fn main() {
    let instructions = parse_input(INPUT);

    let res1 = part_1(&instructions);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&instructions);
    println!("Part 2 result: {}", res2);
}

fn part_1(instructions: &[&str]) -> usize {
    instructions
        .iter()
        .map(|instruction| hash(instruction))
        .sum()
}

fn part_2(instructions: &[&str]) -> usize {
    let operations = parse_operations(instructions);

    let mut boxes: [Vec<Lens>; 256] = array::from_fn(|_| Vec::new());
    operations.into_iter().for_each(|op| {
        let box_idx = op.lens_box();
        match op {
            Operation::Remove { label } => {
                if let Some((lens_idx, _)) =
                    boxes[box_idx].iter().find_position(|l| l.label == label)
                {
                    boxes[box_idx].remove(lens_idx);
                }
            }
            Operation::Add { lens } => {
                if let Some((lens_idx, _)) = boxes[box_idx]
                    .iter()
                    .find_position(|l| l.label == lens.label)
                {
                    let _ = std::mem::replace(&mut boxes[box_idx][lens_idx], lens);
                } else {
                    boxes[box_idx].push(lens);
                }
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, lenses)| {
            (1 + box_idx)
                * lenses
                    .iter()
                    .enumerate()
                    .map(|(lens_idx, lens)| (1 + lens_idx) * lens.value)
                    .sum::<usize>()
        })
        .sum()
}

fn hash(s: &str) -> usize {
    s.as_bytes()
        .into_iter()
        .fold(0, |acc, &c| (acc + c as usize) * 17 % 256)
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(',').collect_vec()
}

fn parse_operations(instructions: &[&str]) -> Vec<Operation> {
    instructions
        .iter()
        .map(|s| {
            if s.contains('=') {
                let (label, value) = sscanf!(s, "{String}={usize}").unwrap();
                Operation::Add { lens: Lens { label, value } }
            } else {
                let label = sscanf!(s, "{String}-").unwrap();
                Operation::Remove { label }
            }
        })
        .collect_vec()
}

#[derive(PartialEq, Eq, Debug)]
struct Lens {
    label: String,
    value: usize,
}

impl Operation {
    fn lens_box(&self) -> usize {
        match self {
            Operation::Add { lens } => hash(&lens.label),
            Operation::Remove { label } => hash(label),
        }
    }
}

enum Operation {
    Add { lens: Lens },
    Remove { label: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(TEST_INPUT)), 1320);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(TEST_INPUT)), 145);
    }
}
