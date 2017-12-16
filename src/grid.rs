extern crate rand;

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
        self.columns
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn cell_count(&self) -> usize {
        self.lines * self.columns
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

    fn pin(&self, index: usize) -> (usize, usize) {
        let y = index / self.columns;
        let x = index - y * self.columns;
        (x, y)
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

    pub fn center(&self) -> Option<Pos<T>> {
        let x = self.columns / 2;
        let y = self.lines / 2;
        self.cell(x, y)
    }

    pub fn anywhere_rand(&self) -> Option<Pos<T>>
    {
        use self::rand::Rng;

        let index = rand::thread_rng().gen_range(0, self.cell_count());
        let (column, line) = self.pin(index);
        Some(Pos{ column, line, grid: self })
    }

    pub fn anywhere_rand_match<F>(&self, func: F) 
        -> Option<Pos<T>> 
        where F: Fn(&Pos<T>) -> bool
    {
        use self::rand::Rng;

        let index = rand::thread_rng().gen_range(0, self.cell_count());
        let (column, line) = self.pin(index);
        let mut candidate = Pos{ column, line, grid: self };

        let mut security = self.cell_count();
        while security > 0 && !func(&candidate) {
            let index = rand::thread_rng().gen_range(0, self.cell_count());
            let (column, line) = self.pin(index);
            candidate = Pos{ column, line, grid: self };
            security -= 1;
        }

        if security == 0 { None } else { Some(candidate) }
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


#[derive(Debug, PartialEq, Clone)]
pub struct Address {
    pub column: usize,
    pub line: usize
}


impl Address {
    pub fn from<'a, T>(&self, wgrid: &'a Within<T>) -> Option<Pos<'a, T>> {
        wgrid.grid().cell(self.column, self.line)
    }
    
    pub fn from_mut<'a, T>(&self, wgrid: &'a mut Within<T>) -> Option<PosMut<'a, T>> {
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


impl<'a, T> From<Pos<'a, T>> for Address where T: 'a {
    fn from(pos: Pos<'a, T>) -> Address {
        Address {
            column: pos.column,
            line: pos.line
        }
    }
}


impl<'a, T> From<&'a Pos<'a, T>> for Address where T: 'a {
    fn from(pos: &'a Pos<'a, T>) -> Address {
        Address {
            column: pos.column,
            line: pos.line
        }
    }
}


impl<'a, T> Pos<'a, T> where T: 'a {
    pub fn neighbours<'b>(&'b self) -> Vec<Pos<'b, T>> {
        let mut result = Vec::with_capacity(4);

        // top
        if let Some(new_line) = self.line.checked_sub(1) {
            if self.grid.contains(self.column, new_line) {
                result.push(Pos{
                    column: self.column,
                    line: new_line,
                    grid: self.grid
                });
            }
        }

        // left
        if let Some(new_column) = self.column.checked_sub(1) {
            if self.grid.contains(new_column, self.line) {
                result.push(Pos{
                    column: new_column,
                    line: self.line,
                    grid: self.grid
                });
            }
        }

        // bottom
        if let Some(new_line) = self.line.checked_add(1) {
            if self.grid.contains(self.column, new_line) {
                result.push(Pos{
                    column: self.column,
                    line: new_line,
                    grid: self.grid
                });
            }
        }

        // right
        if let Some(new_column) = self.column.checked_add(1) {
            if self.grid.contains(new_column, self.line) {
                result.push(Pos{
                    column: new_column,
                    line: self.line,
                    grid: self.grid
                });
            }
        }

        result
    }
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
