
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Point(pub usize, pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Maze {
    grid: [i32; 285],
    pub rows: usize,
    pub cols: usize,
}

impl Maze {

    pub fn new(rows: usize, cols: usize, default_value: i32) -> Self {
        assert!(rows * cols == 285, "Maze must be exactly 15x19.");
        Self {
            grid: [default_value; 285], // Fill with default value (e.g., 1 for open path)
            rows,
            cols,
        }
    }

    pub fn index(&self, point: Point) -> Option<usize> {
        if point.0 < self.rows && point.1 < self.cols {
            Some(point.0 * self.cols + point.1)
        } else {
            None
        }
    }

    pub fn get(&self, point: Point) -> Option<i32> {
        self.index(point).map(|idx| self.grid[idx])
    }

    pub fn set(&mut self, point: Point, value: i32) {
        if let Some(idx) = self.index(point) {
            self.grid[idx] = value;
        }
    }

    pub fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|&cell| if cell == 1 { '.' } else { '#' })
            .collect()
    }

    pub fn clone(&self) -> Self {
        Self {
            grid: self.grid.clone(),
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, i32)> + '_ {
        self.grid.iter().enumerate().map(move |(i, &value)| {
            let row = i / self.cols;
            let col = i % self.cols;
            (row, col, value)
        })
    }
}