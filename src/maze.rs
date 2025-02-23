
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Point(pub usize, pub usize);

pub struct Maze {
    pub grid: Vec<Vec<i32>>,
}

impl Maze {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            grid: vec![vec![1; cols]; rows], // Default: all open paths (1s)
        }
    }

    pub fn get(&self, point: Point) -> Option<i32> {
        self.grid.get(point.0).and_then(|row| row.get(point.1)).copied()
    }

    pub fn set(&mut self, point: Point, value: i32) {
        if point.0 < self.grid.len() && point.1 < self.grid[0].len() {
            self.grid[point.0][point.1] = value;
        }
    }

    pub fn to_string(&self) -> String {
        self.grid
            .iter()
            .flat_map(|row| row.iter().map(|&cell| if cell == 1 { '.' } else { '#' }))
            .collect()
    }

    pub fn clone(&self) -> Self {
        Self {
            grid: self.grid.clone(),
        }
    }
}