use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::Saturating;
use std::{cmp::Ordering, collections::HashMap, fmt::Display, iter::zip};

use lazy_regex::regex_captures;

pub fn solve(input: Vec<String>) {
    let rounds = parse(input);
    println!("Day 7, part one: {}", part_one(rounds.clone()));
    println!("Day 7, part two: {}", part_two(rounds));
}

fn part_one(mut rounds: Vec<Round>) -> usize {
    rounds.sort_by(
        |ra, rb| match ra.hand.0.get_rank() == rb.hand.0.get_rank() {
            true => {
                let pairs = zip(&ra.hand.0, &rb.hand.0);
                for (ca, cb) in pairs {
                    if !ca.eq(&cb) {
                        return cb.cmp(&ca);
                    }
                }
                return Ordering::Equal;
            }
            false => ra.hand.0.get_rank().cmp(&rb.hand.0.get_rank()),
        },
    );

    rounds
        .iter()
        .enumerate()
        .map(|(i, r)| r.bid * (rounds.len() - i))
        .sum()
}

fn part_two(rounds: Vec<Round>) -> usize {
    let mut rounds = rounds
        .into_iter()
        .map(|r| {
            let hand = r
                .hand
                .0
                .iter()
                .map(|c| match c.clone() {
                    Card::J => Card::Joker,
                    c => c,
                })
                .collect::<Vec<Card>>();

            Round {
                hand: Hand(hand),
                bid: r.bid,
            }
        })
        .collect::<Vec<_>>();

    rounds.sort_by(
        |ra, rb| match ra.hand.0.get_rank() == rb.hand.0.get_rank() {
            true => {
                let pairs = zip(&ra.hand.0, &rb.hand.0);
                for (ca, cb) in pairs {
                    if !ca.eq(&cb) {
                        return cb.cmp(&ca);
                    }
                }
                return Ordering::Equal;
            }
            false => ra.hand.0.get_rank().cmp(&rb.hand.0.get_rank()),
        },
    );

    rounds
        .iter()
        .enumerate()
        .map(|(i, r)| r.bid * (rounds.len() - i))
        .sum()
}

fn parse(input: Vec<String>) -> Vec<Round> {
    input
        .iter()
        .map(|l| {
            let (_, raw_hand, bid) =
                regex_captures!(r#"([2-9AKQJT]{5})\s+(\d+)"#, l).expect("valid round info");
            let bid = bid.parse::<usize>().expect("valid bid");

            let hand = raw_hand
                .chars()
                .map(|c| match c {
                    'A' => Card::A,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => Card::J,
                    'T' => Card::T,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    _ => unreachable!(),
                })
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap();

            let hand = Hand(hand);

            Round { hand, bid }
        })
        .collect::<Vec<Round>>()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

#[derive(Debug, Clone)]
struct Hand(Vec<Card>);

#[derive(Debug, Clone)]
struct Round {
    hand: Hand,
    bid: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
enum HandRank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

trait HandRanker {
    fn get_rank(&self) -> HandRank;
}
impl HandRanker for Vec<Card> {
    fn get_rank(&self) -> HandRank {
        let counts =
            self.into_iter()
                .counts()
                .into_iter()
                .fold(HashMap::new(), |mut acc, (_, count)| {
                    acc.entry(count)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                    acc
                });
        let appearances = self.into_iter().counts();
        let jokers = *(appearances.get(&Card::Joker).unwrap_or(&0usize));

        match (
            counts.get(&5usize),
            counts.get(&4usize),
            counts.get(&3usize),
            counts.get(&2usize),
        ) {
            // 5 kind
            (Some(_), None, None, None) => HandRank::FiveOfAKind,
            // 4 kind + jokers
            (None, Some(_), None, None) if jokers > 0 => HandRank::FiveOfAKind,
            (None, Some(_), None, None) => HandRank::FourOfAKind,
            // full-house + jokers
            (None, None, Some(_), Some(_)) if jokers > 0 => HandRank::FiveOfAKind,
            (None, None, Some(_), Some(_)) => HandRank::FullHouse,
            // 3 kind + jokers
            (None, None, Some(_), None) if jokers > 0 => HandRank::FourOfAKind,
            (None, None, Some(_), None) => HandRank::ThreeOfAKind,
            // two-pair + jokers
            (None, None, None, Some(2)) if jokers == 2 => HandRank::FourOfAKind,
            (None, None, None, Some(2)) if jokers == 1 => HandRank::FullHouse,
            (None, None, None, Some(2)) => HandRank::TwoPair,
            // one-pair + jokers
            (None, None, None, Some(1)) if jokers > 0 => HandRank::ThreeOfAKind,
            (None, None, None, Some(1)) => HandRank::OnePair,
            _ => num::FromPrimitive::from_u32(6.saturating_sub(jokers as u32)).expect("valid rank"),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match *self {
            Card::A => 'A',
            Card::K => 'K',
            Card::Q => 'Q',
            Card::J => 'J',
            Card::T => 'T',
            Card::Nine => '9',
            Card::Eight => '8',
            Card::Seven => '7',
            Card::Six => '6',
            Card::Five => '5',
            Card::Four => '4',
            Card::Three => '3',
            Card::Two => '2',
            Card::Joker => '#',
        };
        write!(f, "{}", value)
    }
}
impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = &self.0;
        write!(
            f,
            "{}{}{}{}{}",
            cards[0], cards[1], cards[2], cards[3], cards[4]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_7_part_one() {
        let input = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

        let rounds = parse(input);

        assert_eq!(part_one(rounds), 6440);
    }

    #[test]
    fn test_day_7_part_two() {
        let input = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

        let input2 = [
            "2345A 1", "Q2KJJ 13", "Q2Q2Q 19", "T3T3J 17", "T3Q33 11", "2345J 3", "J345A 2",
            "32T3K 5", "T55J5 29", "KK677 7", "KTJJT 34", "QQQJA 31", "JJJJJ 37", "JAAAA 43",
            "AAAAJ 59", "AAAAA 61", "2AAAA 23", "2JJJJ 53", "JJJJ2 41",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

        let rounds = parse(input);
        let rounds2 = parse(input2);

        assert_eq!(part_two(rounds), 5905);
        assert_eq!(part_two(rounds2), 6839);
    }
}
