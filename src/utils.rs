use grid::Grid;

pub trait LookAround<T> {
    fn get_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize, Option<&T>)>;
    fn get_xy_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize, Option<&T>)>;
}

impl<T> LookAround<T> for Grid<T> {
    fn get_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize, Option<&T>)> {
        let neighbors = [
            (row.wrapping_sub(1), col),
            (row.wrapping_add(1), col),
            (row, col.wrapping_sub(1)),
            (row, col.wrapping_add(1)),
            (row.wrapping_sub(1), col.wrapping_sub(1)),
            (row.wrapping_sub(1), col.wrapping_add(1)),
            (row.wrapping_add(1), col.wrapping_sub(1)),
            (row.wrapping_add(1), col.wrapping_add(1)),
        ];

        neighbors
            .iter()
            .map(|(r, c)| (*r, *c, self.get(*r, *c)))
            .collect()
    }

    /// get_xy_neighbors returns the elements that are next to self without diagonals.
    fn get_xy_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize, Option<&T>)> {
        let neighbors = [
            (row.wrapping_sub(1), col),
            (row.wrapping_add(1), col),
            (row, col.wrapping_sub(1)),
            (row, col.wrapping_add(1)),
        ];

        neighbors
            .iter()
            .map(|(r, c)| (*r, *c, self.get(*r, *c)))
            .collect()
    }
}
