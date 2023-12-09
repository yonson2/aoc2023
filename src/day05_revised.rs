use std::cmp::{max, min};
use std::ops::Range;

use lazy_regex::regex_captures;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Map {
    source_start: i64,
    destination_start: i64,
    range: i64,
}

impl Map {
    fn source_contains(&self, input: i64) -> bool {
        (input >= self.source_start) && (input < self.source_start + self.range)
    }

    fn intersection_with_seed_range(&self, seed: &SeedRange) -> SeedRange {
        max(seed.start, self.destination_start)..min(seed.end, self.destination_start + self.range)
    }
}

/// transform takes an input (i.e. seed) and its maps and returns the next input (i.e. soil)
fn transform(input: i64, maps: &[Map]) -> i64 {
    for map in maps {
        if map.source_contains(input) {
            let diff = input - map.source_start;
            return map.destination_start + diff;
        }
    }
    input
}

fn transform_range(range: SeedRange, maps: &[Map]) -> Vec<SeedRange> {
    let mut unprocessed_queue = vec![range];
    let mut processed = vec![];

    while let Some(item) = unprocessed_queue.pop() {
        // we need to know if our range has any matching values for our map.
        let mapped_items = maps.iter().find(|&map| {
            let intersection = map.intersection_with_seed_range(&item);
            !intersection.is_empty()
        });

        let Some(&map) = mapped_items else {
            // if we don't have a match we can consider this range processed.
            processed.push(item);
            continue;
        };

        let Map {
            destination_start,
            source_start,
            range,
        } = map;

        let SeedRange {
            start: item_start,
            end: item_end,
        } = item;

        let source_end = source_start + range;

        //Now, we need to make sure that:
        // 1. Seed is contained in transformation.
        // 2. Our map doesn't include item but includes item's left boundary.
        // 3. Our map doesn't include item but includes item's right boundary.
        // 4. Our map is smaller than item and fully contained in it.
        // 5. Our map doesn't intersect with seed.

        let offset = destination_start - source_start;
        let intersection = map.intersection_with_seed_range(&item);

        processed.push(Range {
            start: intersection.start + offset,
            end: intersection.end + offset,
        });

        if item_start < source_start {
            unprocessed_queue.push(Range {
                start: item_start,
                end: intersection.start - 1,
            });
        }

        if item_end > source_end {
            unprocessed_queue.push(Range {
                start: intersection.end + 1,
                end: item_end,
            });
        }
    }
    processed.into_iter().collect()
}

pub fn solve(input: String) {
    let (seeds, maps) = parse_input(input);
    println!("Day 5, part one: {}", part_one(seeds.clone(), maps.clone()));
    println!("Day 5, part two: {}", part_two(seeds.clone(), maps.clone()));
}

fn part_one(seeds: Vec<i64>, maps: Vec<Vec<Map>>) -> i64 {
    seeds
        .par_iter()
        .map(|&s| maps.iter().fold(s, |acc, curr| transform(acc, curr)))
        .min()
        .unwrap()
}

type SeedRange = Range<i64>;

fn part_two(seeds: Vec<i64>, maps: Vec<Vec<Map>>) -> i64 {
    let actual_seeds = seeds
        .chunks_exact(2)
        .map(|seed_and_range| SeedRange {
            start: seed_and_range[0],
            end: seed_and_range[1] + seed_and_range[0],
        })
        .collect::<Vec<_>>();

    let a = actual_seeds
        .into_iter()
        .map(|sr| {
            maps.iter().fold(vec![sr], |acc, curr| {
                let previous_step = acc.clone();
                let mut processed = Vec::new();
                previous_step.iter().for_each(|r| {
                    let mut transformed = transform_range(r.clone(), curr).to_vec();
                    processed.append(&mut transformed);
                });
                processed
            })
        })
        .collect::<Vec<_>>();
    println!(
        "LEN: {}",
        a.iter().flatten().map(|a| a.start).min().unwrap()
    );
    0
}

fn parse_input(input: String) -> (Vec<i64>, Vec<Vec<Map>>) {
    let input = input.split_once("\n").unwrap();

    let seeds = &input
        .0
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse::<i64>().expect("valid seed"))
        .collect::<Vec<_>>();

    let maps = &input
        .1
        .trim()
        .split("\n\n")
        .map(|s| {
            s.split(":").collect::<Vec<_>>()[1]
                .trim()
                .split("\n")
                .map(|c| {
                    let (_, destination, source, range) =
                        regex_captures!(r#"(\d+) (\d+) (\d+)"#, c).expect("valid map line");
                    let (destination, source, range) = (
                        destination.parse::<i64>().expect("valid destination"),
                        source.parse::<i64>().expect("valid source"),
                        range.parse::<i64>().expect("valid range"),
                    );
                    Map {
                        source_start: source,
                        destination_start: destination,
                        range,
                    }
                })
                .collect::<Vec<Map>>()
        })
        .collect::<Vec<_>>();

    (seeds.to_vec(), maps.to_vec())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_day_5_part_two_revised() {
//         let input = "seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4";
//         let input = parse_input(String::from(input));
//         assert_eq!(part_two(input.0, input.1), 46);
//     }
// }
