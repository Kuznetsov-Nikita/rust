#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![T::default(); rows * cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    fn index_from_coords(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[self.index_from_coords(row, col)]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        let index = self.index_from_coords(row, col);
        self.grid[index] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];

        if row != 0 && col != 0 {
            neighbours.push((row - 1, col - 1));
        }
        if row != 0 {
            neighbours.push((row - 1, col));
        }
        if row != 0 && col != self.cols - 1 {
            neighbours.push((row - 1, col + 1));
        }
        if col != 0 {
            neighbours.push((row, col - 1));
        }
        if col != self.cols - 1 {
            neighbours.push((row, col + 1));
        }
        if row != self.cols - 1 && col != 0 {
            neighbours.push((row + 1, col - 1));
        }
        if row != self.rows - 1 {
            neighbours.push((row + 1, col));
        }
        if row != self.rows - 1 && col != self.cols - 1 {
            neighbours.push((row + 1, col + 1));
        }

        neighbours
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    fn alives_count(&self, row: usize, col: usize) -> usize {
        let neighbours = self.grid.neighbours(row, col);

        let mut alives_count = 0;

        for neigbour in neighbours {
            if self.grid.get(neigbour.0, neigbour.1) == &Cell::Alive {
                alives_count += 1;
            }
        }

        alives_count
    }

    pub fn step(&mut self) {
        let mut new_grid = self.grid.clone();

        for row in 0..self.grid.rows {
            for col in 0..self.grid.cols {
                let alives_count = self.alives_count(row, col);

                match (self.grid.get(row, col), alives_count) {
                    (Cell::Alive, 2..=3) => new_grid.set(Cell::Alive, row, col),
                    (Cell::Alive, ..) => new_grid.set(Cell::Dead, row, col),
                    (Cell::Dead, 3) => new_grid.set(Cell::Alive, row, col),
                    _ => new_grid.set(Cell::Dead, row, col),
                }
            }
        }

        self.grid = new_grid
    }
}
