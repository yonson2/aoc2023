use grid::{grid, Grid};
use itertools::Itertools;
use std::cmp::{max, min};

pub fn solve(input: Vec<String>) {
    let grid = parse(input);
    println!("Day 11, part one: {}", part_one(&grid, 1));
    println!("Day 11, part two: {}", part_one(&grid, 999999));
}

fn part_one(grid: &Grid<Universe>, factor: usize) -> usize {
    let expansion = expand_universe(&grid, factor);

    grid // find the galaxies
        .indexed_iter()
        .fold(Vec::new(), |mut acc, curr| match curr {
            ((y, x), Universe::Galaxy) => {
                acc.push(Point { x, y });
                acc
            }
            _ => acc,
        })
        .into_iter()
        .combinations(2) // find the unique combinations
        .map(|p| calculate_distance(&p[0], &p[1], &expansion)) // calculate the distance.
        .sum()
}

fn calculate_distance(p1: &Point, p2: &Point, expansion: &Expansion) -> usize {
    let distance = p1.distance(&p2);
    let (min_x, min_y) = (min(p1.x, p2.x), min(p1.y, p2.y));
    let (max_x, max_y) = (max(p1.x, p2.x), max(p1.y, p2.y));

    let rows = expansion
        .x
        .iter()
        .filter(|&&r| min_y < r && max_y > r)
        .count()
        * expansion.factor;
    let cols = expansion
        .y
        .iter()
        .filter(|&&c| min_x < c && max_x > c)
        .count()
        * expansion.factor;

    distance + rows + cols
}

fn expand_universe(grid: &Grid<Universe>, factor: usize) -> Expansion {
    let (_, empty_rows) = grid.iter_rows().fold((0, vec![]), |mut acc, curr| {
        if curr.into_iter().all(|l| matches!(l, Universe::Space)) {
            acc.1.push(acc.0);
        }
        acc.0 += 1; // increment the index.
        acc
    });
    let (_, empty_cols) = grid.iter_cols().fold((0, vec![]), |mut acc, curr| {
        if curr.into_iter().all(|l| matches!(l, Universe::Space)) {
            acc.1.push(acc.0);
        }
        acc.0 += 1;
        acc
    });

    Expansion {
        x: empty_rows,
        y: empty_cols,
        factor,
    }
}

fn parse(input: Vec<String>) -> Grid<Universe> {
    let rows = input
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Universe::Space,
                    '#' => Universe::Galaxy,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut grid: Grid<Universe> = grid![];

    for row in rows {
        grid.push_row(row);
    }

    grid
}

#[derive(Debug)]
enum Universe {
    Space,
    Galaxy,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    // taxicab distance
    fn distance(&self, other: &Point) -> usize {
        (max(self.x, other.x) - min(self.x, other.x))
            + (max(self.y, other.y) - min(self.y, other.y))
    }
}

/// `Expansion` tells us where our universe has expanded.
#[derive(Debug)]
struct Expansion {
    x: Vec<usize>,
    y: Vec<usize>,
    factor: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_11_part_one() {
        let input = [
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let grid = parse(input);

        assert_eq!(part_one(&grid, 1), 374);
    }

    #[test]
    fn test_day_11_part_two() {
        let input = [
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let grid = parse(input);

        assert_eq!(part_one(&grid, 99), 8410);
        assert_eq!(part_one(&grid, 9), 1030);
    }
}
