use std::fmt::{Debug, Formatter, Error};


pub struct Grid<T> {
    columns: usize,
    lines: usize,
    cells: Vec<T>
}


impl<T> Grid<T> where T: Default {
    pub fn new(columns: usize, lines: usize) -> Grid<T> {
        let count = (columns * lines) as usize;
        let mut cells = Vec::with_capacity(count);
        for _ in 0..count {
            cells.push(T::default());
        }
        Grid{ columns, lines, cells }
    }
}


impl<T> Grid<T> {
    pub fn columns(&self) -> usize {
        return self.columns
    }

    pub fn lines(&self) -> usize {
        return self.lines
    }

    pub fn iter(&self) -> GridIterator<T> {
        let max = self.columns * self.lines;
        GridIterator{ grid: self, i: 0, max }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        y >= self.lines || x >= self.columns
    }

    fn localize(&self, x: usize, y: usize) -> usize {
        y * self.columns + x
    }

    pub fn cell(&self, x: usize, y: usize) -> Option<&T> {
        if self.contains(x, y) {
            None
        } else {
            let index = self.localize(x, y);
            Some(&self.cells[index])
        }
    }

    pub fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if self.contains(x, y) {
            None
        } else {
            let index = self.localize(x, y);
            Some(&mut self.cells[index])
        }
    }
}


impl<T> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Grid{{{}x{}}}", self.columns, self.lines)
    }
}


#[derive(Debug)]
pub struct GridCell<'a, T> where T: 'a {
    pub column: usize,
    pub line: usize,
    pub grid: &'a Grid<T>
}


pub struct GridIterator<'a, T> where T: 'a {
    grid: &'a Grid<T>,
    i: usize,
    max: usize
}


impl<'a, T> Debug for GridIterator<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let x = self.i / self.grid.columns();
        let y = (self.i - x) % self.grid.lines();
        write!(f, "GridIterator{{{} => {},{}}}", self.i, x, y)
    }
}


impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = GridCell<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.max {
            None
        } else {
            let y = self.i / self.grid.columns();
            let x = self.i - y * self.grid.columns();

            self.i += 1;

            Some(GridCell{ column: x, line: y, grid: self.grid })
        }
    }
}
