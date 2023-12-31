use crate::utils::LookAround;
use std::collections::{HashMap, HashSet};

use grid::{grid, Grid};

pub fn solve(input: Vec<String>) {
    let map = parse_input(&input);
    println!("Day 3, part one: {}", part_one(&map.numbers));
    println!("Day 3, part two: {}", part_two(&map.gears_with_numbers));
}

fn part_one(numbers: &[EngineNumber]) -> usize {
    numbers.iter().map(|n| n.value).sum()
}

fn part_two(gears: &HashMap<(usize, usize), Vec<usize>>) -> usize {
    gears
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| numbers[0] * numbers[1])
        .sum()
}

fn parse_input(input: &[String]) -> EngineMap {
    let mut grid: Grid<EnginePiece> = grid![];
    input
        .iter()
        .map(|l| parse_input_line(l))
        .for_each(|row| grid.push_row(row));

    let mut current_numbers: Vec<(usize, usize, char)> = Vec::new();
    let mut numbers = Vec::new();
    let mut gears = HashMap::new();

    for x in 0..grid.rows() {
        //clear numbers on new row.
        current_numbers.clear();
        for y in 0..grid.cols() {
            if let Some(c) = grid.get(x, y) {
                match c.clone() {
                    EnginePiece::Numeric(c) => {
                        current_numbers.push((x, y, c));
                        // check end of row. end of "line"
                        if y == grid.cols() - 1 && !current_numbers.clone().is_empty() {
                            // Now we have the x, the y and the number.
                            if let Some(n) = get_engine_number(current_numbers.clone(), &grid) {
                                numbers.push(n);
                                current_numbers.clear();
                            }
                        }
                    }
                    _ => {
                        if !current_numbers.is_empty() {
                            if let Some(n) = get_engine_number(current_numbers.clone(), &grid) {
                                numbers.push(n);
                            }
                            current_numbers.clear();
                        }
                    }
                }
            }
        }
    }

    // find gears
    numbers.iter().for_each(|n| {
        let mut different_gears = HashSet::new();
        for y in n.y_start..=n.y_end {
            let neighbors = grid.get_neighbors(n.x, y);
            for n in neighbors {
                if let (nx, ny, Some(EnginePiece::Symbol('*'))) = n {
                    different_gears.insert((nx, ny));
                }
            }
        }
        for (x, y) in different_gears {
            gears.entry((x, y)).or_insert(Vec::new()).push(n.value)
        }
    });

    EngineMap {
        numbers,
        gears_with_numbers: gears,
    }
}

fn parse_input_line(line: &str) -> Vec<EnginePiece> {
    line.chars()
        .map(|c| match c {
            c if c.is_numeric() => EnginePiece::Numeric(c),
            '.' => EnginePiece::Period,
            c => EnginePiece::Symbol(c),
        })
        .collect()
}

fn get_engine_number(
    numbers: Vec<(usize, usize, char)>,
    grid: &Grid<EnginePiece>,
) -> Option<EngineNumber> {
    if numbers
        .iter()
        .any(|(x, y, g)| EnginePiece::Numeric(*g).is_engine_part(*x, *y, grid))
    {
        let (mut y_start, mut y_end, mut x) = (0, 0, 0);
        let value = numbers
            .iter()
            .enumerate()
            .map(|(i, n)| {
                if i == 0 {
                    y_start = n.1;
                    x = n.0;
                }
                if (i + 1) == numbers.len() {
                    y_end = n.1;
                }
                n.2
            })
            .collect::<String>()
            .parse::<usize>()
            .expect("valid number");

        Some(EngineNumber {
            y_start,
            y_end,
            x,
            value,
        })
    } else {
        None
    }
}

struct EngineNumber {
    y_start: usize,
    y_end: usize,
    x: usize,
    value: usize,
}

struct EngineMap {
    numbers: Vec<EngineNumber>,
    gears_with_numbers: HashMap<(usize, usize), Vec<usize>>,
}

#[derive(Clone)]
enum EnginePiece {
    Numeric(char),
    Symbol(char),
    Period,
}

impl EnginePiece {
    fn is_engine_part(&self, x: usize, y: usize, grid: &Grid<Self>) -> bool {
        match *self {
            Self::Period | Self::Symbol(_) => false,
            Self::Numeric(_) => grid
                .get_neighbors(x, y)
                .iter()
                .filter(|(_, _, n)| n.is_some())
                .any(|(_, _, x)| matches!(x.unwrap(), Self::Symbol(_))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_part_one() {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        let input = parse_input(&input);

        let input2 = [
            "12.......*..",
            "+.........34",
            ".......-12..",
            "..78........",
            "***....60...",
            "78.........9",
            ".5.....23..$",
            "8...90*12...",
            "............",
            "2.2......12.",
            ".*.........*",
            "1.1..503+.56",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        let input2 = parse_input(&input2);

        let input3 = [
            "12.......*..",
            "+.........34",
            ".......-12..",
            "..78........",
            "..*....60...",
            "78..........",
            ".......23...",
            "....90*12...",
            "............",
            "2.2......12.",
            ".*.........*",
            "1.1.......56",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        let input3 = parse_input(&input3);

        assert_eq!(part_one(&input.numbers), 4361);
        assert_eq!(part_one(&input2.numbers), 925);
        assert_eq!(part_one(&input3.numbers), 413);
    }

    #[test]
    fn test_day_3_part_two() {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        let input = parse_input(&input);

        assert_eq!(part_two(&input.gears_with_numbers), 467835);
    }
}
