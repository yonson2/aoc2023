use itertools::Itertools;

pub fn solve(input: String) {
    let data = parse(input);
    println!("Day 13, part one: {}", part_one(data.clone()));
    println!("Day 13, part two: {}", part_two(data));
}

fn part_one(notes: Vec<Note>) -> usize {
    notes.iter().map(|n| n.reflection()).sum()
}

fn part_two(notes: Vec<Note>) -> usize {
    notes.iter().map(|n| n.smudged_reflection()).sum()
}

fn parse(data: String) -> Vec<Note> {
    data.split("\n\n").map_into().collect_vec()
}

#[derive(PartialEq, Eq, Clone)]
enum Ground {
    Ash,
    Rock,
}

impl From<&str> for Note {
    fn from(value: &str) -> Self {
        Self {
            grid: value
                .lines()
                .map(|l| l.chars().map_into().collect_vec())
                .collect_vec(),
        }
    }
}
impl From<char> for Ground {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            _ => Self::Rock,
        }
    }
}

#[derive(Clone)]
struct Note {
    grid: Vec<Vec<Ground>>,
}

impl Note {
    fn rows_diff(&self, y1: usize, y2: usize) -> usize {
        let mut delta = 0;
        for x in 0..self.grid[0].len() {
            if self.grid[y1][x] != self.grid[y2][x] {
                delta += 1;
            }
        }
        delta
    }

    fn columns_diff(&self, x1: usize, x2: usize) -> usize {
        let mut delta = 0;
        for y in 0..self.grid.len() {
            if self.grid[y][x1] != self.grid[y][x2] {
                delta += 1;
            }
        }
        delta
    }

    fn rows_equal(&self, y1: usize, y2: usize) -> bool {
        self.grid[y1] == self.grid[y2]
    }

    fn columns_equal(&self, x1: usize, x2: usize) -> bool {
        for line in &self.grid {
            if line[x1] != line[x2] {
                return false;
            }
        }
        true
    }

    fn reflection(&self) -> usize {
        'rows: for i in 0..self.grid.len() - 1 {
            if self.rows_equal(i, i + 1) {
                let min_distance_to_edge = i.min(self.grid.len() - i - 2);
                for d in 1..=min_distance_to_edge {
                    if !self.rows_equal(i - d, i + d + 1) {
                        continue 'rows;
                    }
                }

                //"value" of the reflection.
                return (i + 1) * 100;
            }
        }

        'columns: for i in 0..self.grid[0].len() - 1 {
            if self.columns_equal(i, i + 1) {
                let min_distance_to_edge = i.min(self.grid[0].len() - i - 2);
                for d in 1..=min_distance_to_edge {
                    if !self.columns_equal(i - d, i + d + 1) {
                        continue 'columns;
                    }
                }
                return i + 1;
            }
        }
        0
    }

    fn smudged_reflection(&self) -> usize {
        'rows: for i in 0..self.grid.len() - 1 {
            let mut diff = self.rows_diff(i, i + 1);
            if diff <= 1 {
                let min_distance_to_edge = i.min(self.grid.len() - i - 2);
                for d in 1..=min_distance_to_edge {
                    diff += self.rows_diff(i - d, i + d + 1);
                    if diff > 1 {
                        continue 'rows;
                    }
                }

                if diff == 0 {
                    continue 'rows;
                }
                return (i + 1) * 100;
            }
        }

        'columns: for i in 0..self.grid[0].len() - 1 {
            let mut diff = self.columns_diff(i, i + 1);
            if diff <= 1 {
                let min_distance_to_edge = i.min(self.grid[0].len() - i - 2);
                for d in 1..=min_distance_to_edge {
                    diff += self.columns_diff(i - d, i + d + 1);
                    if diff > 1 {
                        continue 'columns;
                    }
                }
                if diff == 0 {
                    continue 'columns;
                }
                return i + 1;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_13_part_one() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            .to_string();

        let grid = parse(input);

        assert_eq!(part_one(grid), 405);
    }

    #[test]
    fn test_day_13_part_two() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            .to_string();

        let grid = parse(input);

        assert_eq!(part_two(grid), 400);
    }
}
