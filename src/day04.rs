use std::collections::{BTreeMap, HashSet};

use lazy_regex::regex_captures;

#[derive(Debug, Clone)]
struct ScratchCard {
    your_numbers: HashSet<usize>,
    winning_numbers: HashSet<usize>,
}

impl ScratchCard {
    fn win_amount(&self) -> usize {
        self.your_numbers
            .intersection(&self.winning_numbers)
            .collect::<Vec<&usize>>()
            .len()
    }
}

pub fn solve(input: Vec<String>) {
    let data = parse_input(&input);
    println!("Day 4, part one: {}", part_one(&data));
    println!("Day 4, part two: {}", part_two(data));
}

fn part_one(data: &[ScratchCard]) -> usize {
    data.iter()
        .map(|c| c.win_amount())
        .filter(|c| *c > 0)
        .map(compute_sum_part_one)
        .sum()
}

fn part_two(data: Vec<ScratchCard>) -> usize {
    let winnings = data.iter().map(|c| c.win_amount()).collect::<Vec<_>>();
    let table = (0..winnings.len())
        .map(|index| (index, 1))
        .collect::<BTreeMap<usize, usize>>();

    let total_scratchcards = winnings
        .iter()
        .enumerate()
        .fold(table, |mut acc, (index, wins)| {
            let cards = *acc.get(&index).expect("valid card id");
            for i in (index + 1)..(index + 1 + *wins) {
                acc.entry(i).and_modify(|v| *v += cards);
            }
            acc
        })
        .values()
        .sum();
    total_scratchcards
}

fn compute_sum_part_one(q: usize) -> usize {
    match q {
        0 => unreachable!(),
        1 => 1,
        2 => 2,
        n => 2usize
            .checked_pow(u32::try_from(n).unwrap() - 1)
            .expect("overflow"),
    }
}

fn parse_input(input: &[String]) -> Vec<ScratchCard> {
    input
        .iter()
        .map(|line| {
            let (_, your_numbers_raw, winning_numbers_raw) =
                regex_captures!(r#"\d+: (.+) \| (.+)"#, line).expect("valid scratch line");
            ScratchCard {
                your_numbers: parse_raw_numbers(your_numbers_raw),
                winning_numbers: parse_raw_numbers(winning_numbers_raw),
            }
        })
        .collect()
}

fn parse_raw_numbers(raw: &str) -> HashSet<usize> {
    raw.split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<usize>().expect("valid number"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_4_part_one() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 32 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        let data = parse_input(&input);

        assert_eq!(part_one(&data), 13);
    }

    #[test]
    fn test_day_4_part_two() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 32 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        let data = parse_input(&input);

        assert_eq!(part_two(data), 30);
    }
}
