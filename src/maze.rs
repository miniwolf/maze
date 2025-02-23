
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Point(pub usize, pub usize);

pub struct Maze {
    grid: Vec<i32>,
    pub rows: usize,
    pub cols: usize,
}

impl Maze {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            grid: vec![1; rows * cols], // All open paths (1s)
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
}