use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: Vec<String>) {
    let data = parse(input);
    println!("Day 12, part one: {}", part_one(data.clone()));
    println!("Day 12, part two: {}", part_two(data));
}

fn part_one(data: Vec<SpringRow>) -> usize {
    let mut memo = HashMap::new();
    data.iter().map(|r| all_solutions(&mut memo, r)).sum()
}

fn part_two(data: Vec<SpringRow>) -> usize {
    let mut memo = HashMap::new();
    data.iter()
        .map(|r| all_solutions(&mut memo, &r.expand()))
        .sum()
}

fn all_solutions(memo: &mut HashMap<SpringRow, usize>, record: &SpringRow) -> usize {
    if let Some(&v) = memo.get(record) {
        return v;
    }

    //Edge cases.
    if record.damaged_group.is_empty() {
        let v = match record.springs.iter().any(|c| *c == Spring::Damaged) {
            true => 0,
            false => 1,
        };
        memo.insert(record.clone(), v);
        return v;
    }

    if record.springs.len()
        < record.damaged_group.iter().sum::<usize>() + record.damaged_group.len() - 1
    {
        memo.insert(record.clone(), 0);
        return 0;
    }

    if record.springs[0] == Spring::Operational {
        let solutions = all_solutions(
            memo,
            &SpringRow::new(record.springs[1..].to_vec(), record.damaged_group.clone()),
        );
        memo.insert(record.clone(), solutions);
        return solutions;
    }

    // possible solutions.
    let mut solutions = 0;
    let cur = record.damaged_group[0];
    let all_non_operational = record.springs[0..cur]
        .iter()
        .all(|c| *c != Spring::Operational);
    let end = (cur + 1).min(record.springs.len());
    if all_non_operational
        && ((record.springs.len() > cur && record.springs[cur] != Spring::Damaged)
            || record.springs.len() <= cur)
    {
        solutions = all_solutions(
            memo,
            &SpringRow::new(
                record.springs[end..].to_vec(),
                record.damaged_group[1..].to_vec(),
            ),
        );
    }

    if record.springs[0] == Spring::Unknown {
        solutions += all_solutions(
            memo,
            &SpringRow::new(record.springs[1..].to_vec(), record.damaged_group.clone()),
        );
    }

    memo.insert(record.clone(), solutions);
    solutions
}

fn parse(input: Vec<String>) -> Vec<SpringRow> {
    input
        .iter()
        .map(|l| {
            let split_line = l.split(' ').collect_vec();

            let springs = split_line[0]
                .chars()
                .map(|c| match c {
                    '#' => Spring::Damaged,
                    '.' => Spring::Operational,
                    '?' => Spring::Unknown,
                    _ => unreachable!(),
                })
                .collect_vec();
            let damaged_group = split_line[1]
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect_vec();

            SpringRow::new(springs, damaged_group)
        })
        .collect_vec()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SpringRow {
    springs: Vec<Spring>,
    damaged_group: Vec<usize>,
}

impl SpringRow {
    fn new(springs: Vec<Spring>, damaged_group: Vec<usize>) -> Self {
        Self {
            springs,
            damaged_group,
        }
    }
    fn expand(&self) -> Self {
        let springs = self
            .springs
            .iter()
            .cloned()
            .chain([Spring::Unknown].iter().cloned())
            .cycle()
            .take(self.springs.len() * 5 + 4)
            .collect();
        let groups = self
            .damaged_group
            .iter()
            .cloned()
            .cycle()
            .take(self.damaged_group.len() * 5)
            .collect();

        Self::new(springs, groups)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_12_part_one() {
        let input = [
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let grid = parse(input);

        assert_eq!(part_one(grid), 21);
    }

    #[test]
    fn test_day_12_part_two() {
        let input = [
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let grid = parse(input);

        assert_eq!(part_two(grid), 525152);
    }
}
