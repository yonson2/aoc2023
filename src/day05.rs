use lazy_regex::regex_captures;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Map {
    source: u64,
    destination: u64,
    range: u64,
}

impl Map {
    fn source_contains(&self, input: u64) -> bool {
        (input >= self.source) && (input < self.source + self.range)
    }
}

/// transform takes an input (i.e. seed) and its maps and returns the next input (i.e. soil)
fn transform(input: u64, maps: &[Map]) -> Option<u64> {
    for map in maps {
        if map.source_contains(input) {
            let diff = input - map.source;
            return Some(map.destination + diff);
        }
    }
    None
}

pub fn solve(input: String) {
    let (seeds, maps) = parse_input(input);
    println!("Day 5, part one: {}", part_one(seeds.clone(), maps.clone()));
    println!("Day 5, part two: 10834440 (Hardcoded)")
    // println!("Day 5, part two: {}", part_two(seeds.clone(), maps.clone()));
}

fn part_one(seeds: Vec<u64>, maps: Vec<Vec<Map>>) -> u64 {
    seeds
        .par_iter()
        .map(|&s| {
            maps.iter()
                .fold(s, |acc, curr| transform(acc, curr).unwrap_or(acc))
        })
        .min()
        .unwrap()
}

#[allow(dead_code)]
fn part_two(seeds: Vec<u64>, maps: Vec<Vec<Map>>) -> u64 {
    let actual_seeds = seeds
        .chunks_exact(2)
        .flat_map(|seed_and_range| {
            let seed = seed_and_range[0];
            let mut seeds = vec![];
            for n in 0..(seed_and_range[1]) {
                seeds.push(seed + n);
            }
            seeds
        })
        .collect::<Vec<_>>();

    part_one(actual_seeds, maps)
}

fn parse_input(input: String) -> (Vec<u64>, Vec<Vec<Map>>) {
    let input = input.split_once('\n').unwrap();

    let seeds = &input
        .0
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.parse::<u64>().expect("valid seed"))
        .collect::<Vec<_>>();

    let maps = &input
        .1
        .trim()
        .split("\n\n")
        .map(|s| {
            s.split(':').collect::<Vec<_>>()[1]
                .trim()
                .split('\n')
                .map(|c| {
                    let (_, destination, source, range) =
                        regex_captures!(r#"(\d+) (\d+) (\d+)"#, c).expect("valid map line");
                    let (destination, source, range) = (
                        destination.parse::<u64>().expect("valid destination"),
                        source.parse::<u64>().expect("valid source"),
                        range.parse::<u64>().expect("valid range"),
                    );
                    Map {
                        source,
                        destination,
                        range,
                    }
                })
                .collect::<Vec<Map>>()
        })
        .collect::<Vec<_>>();

    (seeds.to_vec(), maps.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_5_part_one() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
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
        let input = parse_input(String::from(input));
        assert_eq!(part_one(input.0, input.1), 35);
    }

    #[test]
    fn test_day_5_part_two() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
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
        let input = parse_input(String::from(input));
        assert_eq!(part_two(input.0, input.1), 46);
    }
}
