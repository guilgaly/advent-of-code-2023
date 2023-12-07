mod part_1;
mod part_2;

use crate::part_1::part_1;
use crate::part_2::part_2;

static INPUT: &str = include_str!("input");

fn main() {
    // Code for parts 1 and 2 gets duplicated simply to avoid having to mess with the cards ordering... They are very similar otherwise.

    let res1 = part_1(INPUT);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(INPUT);
    println!("Part 2 result: {}", res2);
}
