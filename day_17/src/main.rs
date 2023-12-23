use common::itertools::Itertools;

static INPUT: &str = include_str!("input");

fn main() {
    let values = parse_input(INPUT);

    // let res1 = part_1(&values);
    // println!("Part 1 result: {}", res1);
    //
    // let res2 = part_2(&values);
    // println!("Part 2 result: {}", res2);
}

fn part_1(heat_loss_map: &HeatLossMap) -> usize {
    let start = Point { x: 0, y: 0 };
    let target = Point { x: heat_loss_map.width - 1, y: heat_loss_map.height - 1 };

    0
}

// fn part_2(values: &[u64]) -> usize {
//     values.len()
// }

fn parse_input(input: &str) -> HeatLossMap {
    let values = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();
    let height = values.len();
    let weight = values[0].len();
    HeatLossMap { height, width: weight, values }
}

impl HeatLossMap {
    fn neighbours(&self, &Point { x, y }: &Point) -> Vec<(Point, usize)> {
        let mut res = Vec::new();
        if x > 0 {
            res.push((Point { x: x - 1, y }, self.values[y][x - 1]));
        }
        if x + 1 < self.width {
            res.push((Point { x: x + 1, y }, self.values[y][x + 1]));
        }
        if y > 0 {
            res.push((Point { x, y: y - 1 }, self.values[y - 1][x]));
        }
        if y + 1 < self.height {
            res.push((Point { x, y: y + 1 }, self.values[y + 1][x]));
        }
        res
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

struct HeatLossMap {
    height: usize,
    width: usize,
    values: Vec<Vec<usize>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(TEST_INPUT)), 102);
    }
}
