use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Cell {
    Space,
    Wall,
}
pub use Cell::*;

impl AsRef<str> for Cell {
    fn as_ref(&self) -> &str {
        match self { Space => ".", Wall => "#" }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Point(pub usize, pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Grid<T> {
    data: [T; 16],
    pub rows: usize,
    pub cols: usize,
}

pub type Maze = Grid<Cell>;

// methods that work for any T
impl<T> Grid<T> {
    pub fn in_bounds(&self, point: Point) -> bool {
        point.0 < self.rows && point.1 < self.cols
    }

    pub fn get_index(&self, point: Point) -> Option<usize> {
        if point.0 < self.rows && point.1 < self.cols {
            Some(point.0 * self.cols + point.1)
        } else {
            None
        }
    }
}

// methods that require T to be Copy
impl<T: Copy> Grid<T> {
    pub fn new(rows: usize, cols: usize, default_value: T) -> Self {
        assert!(rows * cols == 16, "Maze must be exactly 15x19.");
        Self {
            data: [default_value; 16], // Fill with default value (e.g., 1 for open path)
            rows,
            cols,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, T)> {
        self.data.iter().enumerate().map(move |(i, &value)| {
            let row = i / self.cols;
            let col = i % self.cols;
            (row, col, value)
        })
    }
}

// Implement the indexing operator `maze[point]` for **reading**
impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        let idx = self.get_index(point).expect("Index out of bounds");
        &self.data[idx]
    }
}

// Implement the indexing operator `maze[point] = value` for **modifying**
impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let idx = self.get_index(point).expect("Index out of bounds");
        &mut self.data[idx]
    }
}

impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in self.data {
            f.write_str(cell.as_ref())?;
        }
        Ok(())
    }
}
