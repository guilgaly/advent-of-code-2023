use common::time_execution;

/// array of (time, distance)
static INPUT_1: [(u64, u64); 4] = [(56, 546), (97, 1927), (78, 1131), (75, 1139)];
/// (time, distance)
static INPUT_2: (u64, u64) = (56977875, 546192711311139);

fn main() {
    let res1 = time_execution("part 1", || part_1(&INPUT_1));
    println!("Part 1 result: {}", res1);

    let res2 = time_execution("part 2", || part_2(INPUT_2));
    println!("Part 2 result: {}", res2);
}

fn part_1(races: &[(u64, u64)]) -> usize {
    races
        .iter()
        .map(|&(total_time, target_distance)| count_ways_to_win(total_time, target_distance))
        .product()
}

fn part_2((total_time, target_distance): (u64, u64)) -> usize {
    count_ways_to_win(total_time, target_distance)
}

fn count_ways_to_win(total_time: u64, target_distance: u64) -> usize {
    (1..total_time)
        .filter(|&button_hold_time| {
            let distance = button_hold_time * (total_time - button_hold_time);
            distance > target_distance
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: [(u64, u64); 3] = [(7, 9), (15, 40), (30, 200)];
    static TEST_INPUT_2: (u64, u64) = (71530, 940200);

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&TEST_INPUT_1), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT_2), 71503);
    }
}
