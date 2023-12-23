use std::collections::HashSet;
use sscanf::sscanf;
use common::itertools::Itertools;

static INPUT: &str = include_str!("input");

fn main() {
    let bricks = parse_input(INPUT);

    let res1 = part_1(&bricks);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&bricks);
    println!("Part 2 result: {}", res2);
}

fn part_1(init_bricks: &[Brick]) -> usize {
    let mut falling_bricks = init_bricks.iter().collect_vec();
    let mut settled_bricks = Vec::new();
    let mut settled_cubes = HashSet::new();



    0
}

fn part_2(bricks: &[Brick]) -> usize {
    0
}

fn parse_input(input: &str) -> Vec<Brick> {
    input.lines()
        .map(|line| {
            let (x0, y0, z0, x1, y1, z1) = sscanf!(line, "{usize},{usize},{usize}~{usize},{usize},{usize}").unwrap();
            Brick(
                Cube { x: x0, y: y0, z: z0},
                Cube { x: x1, y: y1, z: z1},
            )
        })
        .collect_vec()
}

impl Brick {
    fn cubes(&self) -> Vec<Cube> {
        if self.0.x == self.1.x && self.0.y == self.1.y {
            (self.0.z.min(self.1.z)..=self.0.z.max(self.1.z)).map(|z| Cube { x: self.0.x, y: self.0.y, z}).collect_vec()
        } else if self.0.x == self.1.x && self.0.z == self.1.z {
            (self.0.y.min(self.1.y)..=self.0.y.max(self.1.y)).map(|y| Cube { x: self.0.x, y, z: self.0.z}).collect_vec()
        } else if self.0.y == self.1.y && self.0.z == self.1.z {
            (self.0.x.min(self.1.x)..=self.0.x.max(self.1.x)).map(|x| Cube { x, y: self.0.y, z: self.0.z}).collect_vec()
        } else {
            panic!("Invalid brick {:?}", self)
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Brick(Cube, Cube);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
}
