use common::itertools::Itertools;
use range_collections::{RangeSet, RangeSet2};
use sscanf::sscanf;
use std::collections::Bound;
use std::ops::RangeBounds;

static SEEDS: &str = include_str!("seeds");
static INPUT: &str = include_str!("input");

fn main() {
    let seeds = parse_seeds(SEEDS);
    let maps = parse_input(INPUT);

    let res1 = part_1(&seeds, &maps);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&seeds, &maps);
    println!("Part 2 result: {}", res2);
}

fn part_1(seeds: &[i64], maps: &[GardenMap]) -> i64 {
    let mut values = Vec::from(seeds);

    for map in maps {
        values.iter_mut().for_each(|v| {
            let maybe_new_value = map.entries.iter().find_map(|entry| {
                if entry.source_range.contains(v) {
                    Some(*v + entry.offset)
                } else {
                    None
                }
            });
            if let Some(new_value) = maybe_new_value {
                *v = new_value;
            };
        });
    }

    *values.iter().min().unwrap()
}

fn part_2(seeds: &[i64], maps: &[GardenMap]) -> i64 {
    let mut current_ranges: RangeSet2<i64> = RangeSet::empty();
    for (start, length) in seeds.iter().tuples::<(_, _)>() {
        let new_range: RangeSet2<i64> = RangeSet::from(*start..(start + length + 1));
        current_ranges.union_with(&new_range);
    }
    // print_range_set("current_ranges", &current_ranges);

    for map in maps {
        // print_map(map);
        let mut new_ranges: RangeSet2<i64> = RangeSet::empty();
        for entry in map.entries.iter() {
            let matched: RangeSet2<i64> = current_ranges.intersection(&entry.source_range);
            current_ranges.difference_with(&matched);

            for m in matched.iter() {
                let Bound::Included(&&start) = m.start_bound() else {
                    panic!()
                };
                let Bound::Excluded(&&end) = m.end_bound() else {
                    panic!()
                };
                let new_range: RangeSet2<i64> =
                    RangeSet::from((start + entry.offset)..(end + entry.offset));
                new_ranges.union_with(&new_range);
            }
        }
        new_ranges.union_with(&current_ranges); // Add any remaining ranges that were not matched
        current_ranges = new_ranges;
        // print_range_set("current_ranges", &current_ranges);
    }

    current_ranges
        .iter()
        .map(|r| {
            let Bound::Included(&&start) = r.start_bound() else {
                panic!()
            };
            start
        })
        .min()
        .unwrap()
}

fn parse_seeds(seeds: &str) -> Vec<i64> {
    seeds
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec()
}

fn parse_input(input: &str) -> Vec<GardenMap> {
    input
        .split("\n\n")
        .map(|map_str| {
            let mut lines = map_str.lines();
            let name = lines.next().unwrap().to_owned();
            let entries = lines
                .map(|line| {
                    let (dest_range_start, source_range_start, length) =
                        sscanf!(line, "{i64} {i64} {i64}").unwrap();
                    GardenMapEntry {
                        source_range: RangeSet::from(
                            source_range_start..(source_range_start + length),
                        ),
                        offset: dest_range_start - source_range_start,
                    }
                })
                .collect_vec();
            GardenMap { name, entries }
        })
        .collect()
}

#[derive(Debug)]
struct GardenMap {
    name: String,
    entries: Vec<GardenMapEntry>,
}

#[derive(Debug)]
struct GardenMapEntry {
    source_range: RangeSet2<i64>,
    offset: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_SEEDS: &str = "79 14 55 13";
    static TEST_INPUT: &str = "seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(&parse_seeds(TEST_SEEDS), &parse_input(TEST_INPUT)),
            35
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(&parse_seeds(TEST_SEEDS), &parse_input(TEST_INPUT)),
            46
        );
    }
}
