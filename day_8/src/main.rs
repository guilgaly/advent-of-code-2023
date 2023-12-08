use common::itertools::Itertools;
use common::lcmx::lcmx;
use sscanf::sscanf;
use std::collections::HashMap;
use std::error::Error;
use std::iter::repeat;

static INPUT_INSTRUCTIONS: &str = include_str!("instructions");
static INPUT_NODES: &str = include_str!("nodes");

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = parse_instructions(INPUT_INSTRUCTIONS);
    let nodes = parse_nodes(INPUT_NODES);

    let res1 = part_1(&instructions, &nodes);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&instructions, &nodes);
    println!("Part 2 result: {}", res2);

    Ok(())
}

fn part_1(instructions: &[Instruction], nodes: &HashMap<Node, (Node, Node)>) -> usize {
    let mut count = 0;
    let mut current_node = nodes.keys().find(|n| n.0 == "AAA").unwrap();
    let repeated_instructions = repeat(instructions).flat_map(|i| i.iter());
    for instruction in repeated_instructions {
        if current_node.0 == "ZZZ" {
            return count;
        }
        count += 1;
        let choices = nodes.get(current_node).unwrap();
        current_node = instruction.apply(choices);
    }
    0 // never reached
}

fn part_2(instructions: &[Instruction], nodes: &HashMap<Node, (Node, Node)>) -> u64 {
    let starting_nodes = nodes.keys().filter(|n| n.0.ends_with("A")).collect_vec();

    let loop_periods = starting_nodes
        .into_iter()
        .map(|starting_node| {
            let mut count = 0u64;
            let mut current_node = starting_node;
            let repeated_instructions = repeat(instructions).flat_map(|i| i.iter());
            for instruction in repeated_instructions {
                if current_node.0.ends_with("Z") {
                    return count;
                }
                count += 1;
                let choices = nodes.get(current_node).unwrap();
                current_node = instruction.apply(choices);
            }
            0 // never reached
        })
        .collect_vec();

    lcmx(&loop_periods).unwrap()
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("{} not a valid instruction", c),
        })
        .collect()
}

fn parse_nodes(input: &str) -> HashMap<Node, (Node, Node)> {
    input
        .lines()
        .map(|line| {
            let (n, l, r) = sscanf!(line, "{String} = ({String}, {String})").unwrap();
            (Node(n), (Node(l), Node(r)))
        })
        .collect()
}

impl Instruction {
    fn apply<'a>(&self, (l, r): &'a (Node, Node)) -> &'a Node {
        match self {
            Instruction::Left => l,
            Instruction::Right => r,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Node(String);

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INSTRUCTIONS_1: &str = "LLR";
    static TEST_NODES_1: &str = "AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    static TEST_INSTRUCTIONS_2: &str = "LR";
    static TEST_NODES_2: &str = "11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                &parse_instructions(TEST_INSTRUCTIONS_1),
                &parse_nodes(TEST_NODES_1)
            ),
            6
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                &parse_instructions(TEST_INSTRUCTIONS_2),
                &parse_nodes(TEST_NODES_2)
            ),
            6
        );
    }
}
