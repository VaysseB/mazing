use std::iter;
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

    pub fn iter(&self) -> Iterator<T> {
        let crumbs = self.crumbs();
        Iterator{ grid: self, crumbs }
    }

    pub fn crumbs(&self) -> Crumbs {
        Crumbs{ 
            i: 0, 
            columns: self.columns,
            lines: self.lines,
            max: self.columns * self.lines
        }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        y < self.lines && x < self.columns
    }

    fn localize(&self, x: usize, y: usize) -> usize {
        y * self.columns + x
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        if !self.contains(x, y) {
            None
        } else {
            let index = self.localize(x, y);
            Some(&self.cells[index])
        }
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if !self.contains(x, y) {
            None
        } else {
            let index = self.localize(x, y);
            Some(&mut self.cells[index])
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> Option<Pos<T>> {
        if self.contains(x, y) {
            Some(Pos { column: x, line: y, grid: self })
        } else {
            None
        }
    }

    pub fn cell_mut(&mut self, x: usize, y: usize) -> Option<PosMut<T>> {
        if self.contains(x, y) {
            Some(PosMut { column: x, line: y, grid: self })
        } else {
            None
        }
    }
}


impl<T> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Grid{{{}x{}}}", self.columns, self.lines)
    }
}


// ----------------------------------------------------------------------------


pub trait Within<T> {
    fn grid<'a>(&'a self) -> &'a Grid<T>;
    fn grid_mut<'a>(&'a mut self) -> &'a mut Grid<T>;
}


impl<T> Within<T> for Grid<T> {
    fn grid<'a>(&'a self) -> &'a Grid<T> {
        self
    }
    
    fn grid_mut<'a>(&'a mut self) -> &'a mut Grid<T> {
        self
    }
}


// ----------------------------------------------------------------------------


#[derive(Debug)]
pub struct Address {
    pub column: usize,
    pub line: usize
}


impl Address {
    pub fn from<'a, T>(&'a self, wgrid: &'a Within<T>) -> Option<Pos<T>> {
        wgrid.grid().cell(self.column, self.line)
    }
    
    pub fn from_mut<'a, T>(&'a self, wgrid: &'a mut Within<T>) -> Option<PosMut<T>> {
        wgrid.grid_mut().cell_mut(self.column, self.line)
    }
}


// ----------------------------------------------------------------------------


#[derive(Debug)]
pub struct Pos<'a, T> where T: 'a {
    pub column: usize,
    pub line: usize,
    pub grid: &'a Grid<T>
}


#[derive(Debug)]
pub struct PosMut<'a, T> where T: 'a {
    pub column: usize,
    pub line: usize,
    pub grid: &'a mut Grid<T>
}


// ----------------------------------------------------------------------------


pub struct Crumbs {
    pub i: usize,
    columns: usize,
    lines: usize,
    max: usize
}


impl Debug for Crumbs {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let x = self.i / self.columns;
        let y = (self.i - x) % self.lines;
        write!(f, "Crumbs{{{} => {},{}}}", self.i, x, y)
    }
}


impl iter::Iterator for Crumbs {
    type Item = Address;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.max {
            None
        } else {
            let y = self.i / self.columns;
            let x = self.i - y * self.columns;

            self.i += 1;

            Some(Address{ column: x, line: y })
        }
    }
}


// ----------------------------------------------------------------------------


pub struct Iterator<'a, T> where T: 'a {
    grid: &'a Grid<T>,
    crumbs: Crumbs
}


impl<'a, T> Debug for self::Iterator<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Iterator{{{:?}}}", self.crumbs)
    }
}


impl<'a, T> iter::Iterator for self::Iterator<'a, T> {
    type Item = Pos<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.crumbs.next()
            .map(|pos| Pos { 
                column: pos.column, 
                line: pos.line, 
                grid: self.grid 
            })
    }
}
