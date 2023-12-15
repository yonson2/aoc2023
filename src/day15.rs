use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: String) {
    let data = parse(input);
    println!("Day 15, part one: {}", part_one(data.clone()));
    println!("Day 15, part two: {}", part_two(data));
}

fn part_one(sequence: Vec<Vec<char>>) -> usize {
    sequence.iter().fold(0usize, |acc, curr| {
        acc + curr
            .iter()
            .fold(0usize, |a, &c| (((c as usize) + a) * 17usize) % 256usize)
    })
}

fn hash(input: Vec<char>) -> u8 {
    (input
        .iter()
        .fold(0usize, |a, &c| (((c as usize) + a) * 17usize) % 256usize))
    .try_into()
    .unwrap()
}

fn part_two(sequence: Vec<Vec<char>>) -> usize {
    let parsed = parse_part_two(sequence);
    let boxes: HashMap<u8, Vec<Lens>> = HashMap::new();

    let boxes = parsed.iter().fold(boxes, |mut acc, curr| {
        match curr {
            Operation::Add(l) => {
                let box_id = hash(l.clone().label.chars().collect_vec());
                acc.entry(box_id)
                    .and_modify(|b| {
                        // try to get the contents of the label.
                        b.iter_mut()
                            .find(|bl| bl.label == l.label)
                            .map(|bl| *bl = l.clone())
                            .or_else(|| {
                                b.push(l.clone());
                                None
                            });
                    })
                    .or_insert(vec![l.clone()]);

                acc
            }
            Operation::Remove(label) => {
                let box_id = hash(label.chars().collect_vec());
                acc.entry(box_id)
                    .and_modify(|b| b.retain(|bl| bl.label != *label));
                acc
            }
        }
    });

    boxes
        .iter()
        .map(|(box_id, contents)| {
            contents
                .iter()
                .enumerate()
                .map(|(id, focal)| (*box_id as usize + 1) * (id + 1) * focal.focal_length as usize)
                .sum::<usize>()
        })
        .sum()
}

fn parse(input: String) -> Vec<Vec<char>> {
    input
        .trim_end_matches('\n')
        .split(',')
        .map(|seq| seq.chars().collect_vec())
        .collect_vec()
}

fn parse_part_two(input: Vec<Vec<char>>) -> Vec<Operation> {
    input
        .iter()
        .map(|s| {
            let clone = s.clone();
            match s.iter().position(|&c| c == '=') {
                Some(idx) => Operation::Add(Lens {
                    label: clone[0..idx].to_vec().iter().collect(),
                    focal_length: clone[idx + 1..clone.len()]
                        .to_vec()
                        .iter()
                        .collect::<String>()
                        .parse::<u8>()
                        .unwrap(),
                }),
                None => Operation::Remove(clone[0..clone.len() - 1].to_vec().iter().collect()),
            }
        })
        .collect_vec()
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Debug)]
enum Operation {
    Add(Lens),
    Remove(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_15_part_one() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();

        let data = parse(input);

        assert_eq!(part_one(data), 1320);
    }

    #[test]
    fn test_day_15_part_two() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();

        let data = parse(input);

        assert_eq!(part_two(data), 145);
    }
}
