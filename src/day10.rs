use grid::Grid;
use itertools::Itertools;

use crate::utils::LookAround;

pub fn solve(input: String) {
    let data = parse(input.clone());
    println!("Day 10, part one: {}", part_one(data));
}

fn part_one(data: Grid<Piece>) -> usize {
    // find animal.
    let (animal_row_col, _) = data
        .indexed_iter()
        .find(|&(_, p)| matches!(*p, Piece::Animal))
        .expect("need an animal");

    let mut previous_xy = (animal_row_col.1, animal_row_col.0);
    let animal = GridPiece {
        grid: &data,
        x: previous_xy.0,
        y: previous_xy.1,
        piece: Piece::Animal,
    };

    // Now, for either end of the animal, find the connecting pieces until one of the connections
    // is the animal back again.
    let animal_connections = animal.connections();
    let current_piece = animal_connections.first().unwrap();

    let mut current_xy = (current_piece.x, current_piece.y, current_piece.piece);
    let mut route = vec![current_xy];

    loop {
        let current = GridPiece {
            grid: &data,
            x: current_xy.0,
            y: current_xy.1,
            piece: current_xy.2,
        };
        let next_pieces = current.connections();
        // remove the current piece from the connection.
        let next_piece = *next_pieces
            .iter()
            .filter(|&p| (p.x, p.y) != (previous_xy.0, previous_xy.1))
            .collect_vec()
            .first()
            .unwrap();
        previous_xy = (current_xy.0, current_xy.1);
        // check if we are at out starting point.
        if (next_piece.x, next_piece.y) == (animal.x, animal.y) {
            break;
        }
        current_xy = (next_piece.x, next_piece.y, next_piece.piece);
        route.push(current_xy);
    }
    (route.len() + 1) / 2
}

fn parse(input: String) -> Grid<Piece> {
    let cols = input.lines().collect_vec();
    let cols = cols.first().expect("valid grid").chars().count();

    let chars = input
        .chars()
        .filter(|&c| c != '\n')
        .map_into()
        .collect_vec();

    Grid::from_vec(chars, cols)
}

//rows is y and columns is x
#[derive(Debug)]
struct GridPiece<'a> {
    grid: &'a Grid<Piece>,
    x: usize,
    y: usize,
    piece: Piece,
}

impl GridPiece<'_> {
    fn connections(&self) -> Vec<GridPiece> {
        match self.piece {
            Piece::Ground => Vec::new(),
            Piece::Animal => {
                // first get all of its neighbors.
                let mut matches = Vec::new();
                let neighbors = self
                    .grid
                    .get_xy_neighbors(self.y, self.x)
                    .iter()
                    .filter(|&&(_, _, p)| p.is_some())
                    //rows is y and columns is x
                    .map(|&(y, x, p)| GridPiece {
                        grid: self.grid,
                        x,
                        y,
                        piece: *p.unwrap(),
                    })
                    .collect_vec();

                for n in neighbors {
                    match n.piece {
                        Piece::VerticalPipe => {
                            if ((self.x, self.y + 1) == (n.x, n.y))
                                || (self.y > 0 && (self.x, self.y.saturating_sub(1)) == (n.x, n.y))
                            {
                                matches.push(n);
                            }
                        }
                        Piece::HorizontalPipe => {
                            if ((self.x + 1, self.y) == (n.x, n.y))
                                || (self.x > 0 && (self.x.saturating_sub(1), self.y) == (n.x, n.y))
                            {
                                matches.push(n);
                            }
                        }
                        Piece::NorthEastPipe => {
                            if ((self.x, self.y + 1) == (n.x, n.y))
                                || (self.x > 0 && (self.x.saturating_sub(1), self.y) == (n.x, n.y))
                            {
                                matches.push(n)
                            }
                        }
                        Piece::NorthWestPipe => {
                            if ((self.x, self.y + 1) == (n.x, n.y))
                                || ((self.x + 1, self.y) == (n.x, n.y))
                            {
                                matches.push(n)
                            }
                        }
                        Piece::SouthEastPipe => {
                            if (self.y > 0 && (self.x, self.y.saturating_sub(1)) == (n.x, n.y))
                                || (self.x > 0 && (self.x.saturating_sub(1), self.y) == (n.x, n.y))
                            {
                                matches.push(n)
                            }
                        }
                        Piece::SouthWestPipe => {
                            if (self.y > 0 && (self.x, self.y.saturating_sub(1)) == (n.x, n.y))
                                || ((self.x + 1, self.y) == (n.x, n.y))
                            {
                                matches.push(n)
                            }
                        }
                        _ => (),
                        //We are not interested in animal or ground.
                    }
                }

                matches
            }
            Piece::VerticalPipe => {
                let mut matches = Vec::new();
                if self.y > 0 {
                    let &piece = self.grid.get(self.y.saturating_sub(1), self.x).unwrap();
                    matches.push(GridPiece {
                        grid: self.grid,
                        x: self.x,
                        y: self.y.saturating_sub(1),
                        piece,
                    });
                }
                let &piece = self.grid.get(self.y + 1, self.x).unwrap();
                matches.push(GridPiece {
                    grid: self.grid,
                    x: self.x,
                    y: self.y + 1,
                    piece,
                });

                matches
            }
            Piece::HorizontalPipe => {
                let mut matches = Vec::new();
                if self.x > 0 {
                    let &piece = self.grid.get(self.y, self.x.saturating_sub(1)).unwrap();
                    matches.push(GridPiece {
                        grid: self.grid,
                        x: self.x.saturating_sub(1),
                        y: self.y,
                        piece,
                    });
                }

                if let Some(&piece) = self.grid.get(self.y, self.x + 1) {
                    matches.push(GridPiece {
                        grid: self.grid,
                        x: self.x + 1,
                        y: self.y,
                        piece,
                    })
                }
                matches
            }
            Piece::NorthEastPipe => {
                let mut matches = Vec::new();
                if self.y > 0 {
                    let &piece = self.grid.get(self.y.saturating_sub(1), self.x).unwrap();
                    matches.push(GridPiece {
                        grid: self.grid,
                        x: self.x,
                        y: self.y.saturating_sub(1),
                        piece,
                    });
                }
                if let Some(&piece) = self.grid.get(self.y, self.x + 1) {
                    matches.push(GridPiece {
                        grid: self.grid,
                        x: self.x + 1,
                        y: self.y,
                        piece,
                    })
                }
                matches
            }
            Piece::NorthWestPipe => {
                let mut matches = Vec::new();
                if self.y > 0 {
                    let &piece = self.grid.get(self.y.saturating_sub(1), self.x).unwrap();
                    matches.push(GridPiece {
                        grid: self.grid,
                        x: self.x,
                        y: self.y.saturating_sub(1),
                        piece,
                    });
                }
                if self.x > 0 {
                    let &piece = self.grid.get(self.y, self.x.saturating_sub(1)).unwrap();
                    matches.push(GridPiece {
                        grid: self.grid,
                        x: self.x.saturating_sub(1),
                        y: self.y,
                        piece,
                    });
                }
                matches
            }
            Piece::SouthEastPipe => {
                let mut matches = Vec::new();
                if let Some(&piece) = self.grid.get(self.y + 1, self.x) {
                    matches.push(GridPiece {
                        grid: self.grid,
                        x: self.x,
                        y: self.y + 1,
                        piece,
                    })
                }
                if let Some(&piece) = self.grid.get(self.y, self.x + 1) {
                    matches.push(GridPiece {
                        grid: self.grid,
                        x: self.x + 1,
                        y: self.y,
                        piece,
                    })
                }
                matches
            }
            Piece::SouthWestPipe => {
                let mut matches = Vec::new();
                if let Some(&piece) = self.grid.get(self.y + 1, self.x) {
                    matches.push(GridPiece {
                        grid: self.grid,
                        x: self.x,
                        y: self.y + 1,
                        piece,
                    })
                }
                if self.x > 0 {
                    let &piece = self.grid.get(self.y, self.x.saturating_sub(1)).unwrap();
                    matches.push(GridPiece {
                        grid: self.grid,
                        x: self.x.saturating_sub(1),
                        y: self.y,
                        piece,
                    });
                }
                matches
            } // _ => Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Piece {
    Animal,
    Ground,
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthEastPipe,
    SouthWestPipe,
}

impl From<char> for Piece {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Animal,
            '.' => Self::Ground,
            '|' => Self::VerticalPipe,
            '-' => Self::HorizontalPipe,
            'L' => Self::NorthEastPipe,
            'J' => Self::NorthWestPipe,
            'F' => Self::SouthEastPipe,
            '7' => Self::SouthWestPipe,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_10_part_one() {
        let input = ".....
.S-7.
.|.|.
.L-J.
....."
            .to_string();
        let data = parse(input);

        let input2 = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            .to_string();
        let data2 = parse(input2);
        assert_eq!(part_one(data), 4);
        assert_eq!(part_one(data2), 8);
    }
}
