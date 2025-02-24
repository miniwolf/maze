use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Point(pub usize, pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Maze {
    grid: [i32; 16],
    pub rows: usize,
    pub cols: usize,
}

impl Maze {
    pub fn new(rows: usize, cols: usize, default_value: i32) -> Self {
        assert!(rows * cols == 16, "Maze must be exactly 15x19.");
        Self {
            grid: [default_value; 16], // Fill with default value (e.g., 1 for open path)
            rows,
            cols,
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.0 < self.rows && point.1 < self.cols
    }

    pub fn index(&self, point: Point) -> Option<usize> {
        if point.0 < self.rows && point.1 < self.cols {
            Some(point.0 * self.cols + point.1)
        } else {
            None
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            grid: self.grid,
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, i32)> {
        self.grid.iter().enumerate().map(move |(i, &value)| {
            let row = i / self.cols;
            let col = i % self.cols;
            (row, col, value)
        })
    }
}

// Implement the indexing operator `maze[point]` for **reading**
impl Index<Point> for Maze {
    type Output = i32;

    fn index(&self, point: Point) -> &Self::Output {
        let idx = self.index(point).expect("Index out of bounds");
        &self.grid[idx]
    }
}

// Implement the indexing operator `maze[point] = value` for **modifying**
impl IndexMut<Point> for Maze {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let idx = self.index(point).expect("Index out of bounds");
        &mut self.grid[idx]
    }
}

impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in self.grid {
            f.write_str(match cell { 1 => ".", _ => "#" })?;
        }
        Ok(())
    }
}
