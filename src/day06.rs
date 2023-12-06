pub fn solve(input: Vec<String>) {
    let td = parse_part_one(input.clone());
    let td2 = parse_part_two(input);
    println!("Day 6, part one: {}", part_one(td));
    println!("Day 6, part two: {}", part_two(td2));
}

fn part_one(td: Vec<RaceData>) -> usize {
    td.iter()
        .map(|r| {
            (1..r.time)
                .into_iter()
                .filter(|n| {
                    let boat = Boat::new(*n);
                    boat.beats(r)
                })
                .count()
        })
        .product()
}

fn part_two(race: RaceData) -> usize {
    (1..race.time)
        .into_iter()
        .filter(|n| {
            let boat = Boat::new(*n);
            boat.beats(&race)
        })
        .count()
}

fn parse_part_one(input: Vec<String>) -> Vec<RaceData> {
    let times = input[0]
        .split_whitespace()
        .filter_map(|d| d.parse::<usize>().ok());
    let distances = input[1]
        .split_whitespace()
        .filter_map(|d| d.parse::<usize>().ok());
    std::iter::zip(times, distances)
        .map(|(time, distance)| RaceData { time, distance })
        .collect()
}

fn parse_part_two(input: Vec<String>) -> RaceData {
    let time = input[0]
        .split_whitespace()
        .filter(|t| t.chars().all(|c| c.is_numeric()))
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .expect("valid time");

    let distance = input[1]
        .split_whitespace()
        .filter(|t| t.chars().all(|c| c.is_numeric()))
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .expect("valid distance");

    RaceData { time, distance }
}

#[derive(Clone, Debug)]
struct RaceData {
    time: usize,
    distance: usize,
}

struct Boat {
    speed: usize,
}

impl Boat {
    fn new(speed: usize) -> Self {
        Boat { speed }
    }

    fn beats(&self, race: &RaceData) -> bool {
        (self.speed * (race.time - self.speed)) > race.distance
    }
}

impl Default for Boat {
    fn default() -> Self {
        Boat { speed: 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_6_part_one() {
        let input = ["Time:      7  15   30", "Distance:  9  40  200"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

        let td = parse_part_one(input);

        assert_eq!(part_one(td), 288);
    }

    #[test]
    fn test_day_6_part_two() {
        let input = ["Time:      7  15   30", "Distance:  9  40  200"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

        let race = parse_part_two(input);

        assert_eq!(part_two(race), 71503);
    }
}
