use itertools::Itertools;

pub fn solve(input: Vec<String>) {
    println!("Day 14, part one: {}", part_one(input.clone().into()));
    println!("Day 14, part two: {}", part_two(input.into()));
}

fn part_one(mut dish: Dish) -> usize {
    dish.push_north().count_weight()
}

fn part_two(mut dish: Dish) -> usize {
    let mut seen = vec![dish.clone()];

    loop {
        dish = dish.cycle();

        if let Some(id) = seen.iter().position(|d| dish.mirrors == d.mirrors) {
            let cycle = seen.len() - id;
            let final_cycle = id + (1_000_000_000 - id) % cycle;
            return seen[final_cycle].count_weight();
        }

        seen.push(dish.clone())
    }
}

#[derive(Debug, Clone)]
struct Dish {
    mirrors: Vec<Vec<Panel>>,
}

impl Dish {
    // One could argue that its much less code to repeat 4 times pushing north and then rotating
    // the grid clockwise than all this almost-repeated code BUT
    // I like seeing the cycle function as push_nort().push_west()...
    fn push_north(&mut self) -> Self {
        // skip the first row, we start at 1.
        let mut modify = true;
        let mut row_modified = false;

        while modify {
            row_modified = false;
            for i in 1..self.mirrors.len() {
                let clone = &self.clone();
                let row = &clone.mirrors[i];

                // check each Panel.
                for j in 0..row.len() {
                    if row[j] == Panel::RoundedRock && clone.mirrors[i - 1][j] == Panel::Space {
                        self.mirrors[i - 1][j] = Panel::RoundedRock;
                        self.mirrors[i][j] = Panel::Space;
                        row_modified = true;
                    }
                }
            }
            modify = row_modified;
        }

        self.clone()
    }

    fn push_west(&mut self) -> Self {
        // skip the first row, we start at 1.
        let mut modify = true;
        let mut row_modified = false;

        while modify {
            row_modified = false;
            for i in 0..self.mirrors.len() {
                let clone = &self.clone();
                let row = &clone.mirrors[i];

                for j in 1..row.len() {
                    if row[j] == Panel::RoundedRock && clone.mirrors[i][j - 1] == Panel::Space {
                        self.mirrors[i][j - 1] = Panel::RoundedRock;
                        self.mirrors[i][j] = Panel::Space;
                        row_modified = true;
                    }
                }
            }
            modify = row_modified;
        }

        self.clone()
    }

    fn push_south(&mut self) -> Self {
        // skip the first row, we start at 1.
        let mut modify = true;
        let mut row_modified = false;

        while modify {
            row_modified = false;
            for i in 0..self.mirrors.len() - 1 {
                let clone = &self.clone();
                let row = &clone.mirrors[i];

                // check each Panel.
                for j in 0..row.len() {
                    if row[j] == Panel::RoundedRock && clone.mirrors[i + 1][j] == Panel::Space {
                        self.mirrors[i + 1][j] = Panel::RoundedRock;
                        self.mirrors[i][j] = Panel::Space;
                        row_modified = true;
                    }
                }
            }
            modify = row_modified;
        }

        self.clone()
    }

    fn push_east(&mut self) -> Self {
        // skip the first row, we start at 1.
        let mut modify = true;
        let mut row_modified = false;

        while modify {
            row_modified = false;
            for i in 0..self.mirrors.len() {
                let clone = &self.clone();
                let row = &clone.mirrors[i];

                // check each Panel.
                for j in 0..row.len() - 1 {
                    if row[j] == Panel::RoundedRock && clone.mirrors[i][j + 1] == Panel::Space {
                        self.mirrors[i][j + 1] = Panel::RoundedRock;
                        self.mirrors[i][j] = Panel::Space;
                        row_modified = true;
                    }
                }
            }
            modify = row_modified;
        }

        self.clone()
    }

    fn cycle(&mut self) -> Self {
        self.push_north().push_west().push_south().push_east()
    }

    fn count_weight(&self) -> usize {
        self.mirrors
            .iter()
            .rev()
            .enumerate()
            .map(|(i, r)| r.iter().filter(|&&p| p == Panel::RoundedRock).count() * (i + 1))
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Panel {
    RoundedRock,
    CubeRock,
    Space,
}

impl From<Vec<String>> for Dish {
    fn from(value: Vec<String>) -> Self {
        let mirrors = value
            .iter()
            .map(|c| c.chars().map_into().collect_vec())
            .collect_vec();
        Self { mirrors }
    }
}

impl From<char> for Panel {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::RoundedRock,
            '#' => Self::CubeRock,
            _ => Self::Space,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_14_part_one() {
        let input = [
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ]
        .into_iter()
        .map(String::from)
        .collect_vec();

        assert_eq!(part_one(input.into()), 136);
    }

    #[test]
    fn test_day_14_part_two() {
        let input = [
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ]
        .into_iter()
        .map(String::from)
        .collect_vec();

        assert_eq!(part_two(input.into()), 64);
    }
}
