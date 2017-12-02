use std::fmt::{Debug, Formatter, Error};


bitflags! {
    struct GateWay: u32 {
        const HORI = 0b001;
        const VERT = 0b010;
    }
}


pub struct Maze {
    columns: usize,
    lines: usize,
    gates: Vec<GateWay>
}


impl Maze {
    pub fn new(columns: usize, lines: usize) -> Maze {
        let count = (columns * lines) as usize;
        let mut gates = Vec::with_capacity(count);
        for _ in 0..(count) {
            gates.push(GateWay{ bits: 3 });
        }
        Maze {columns, lines, gates}
    }

    pub fn columns(&self) -> usize {
        return self.columns
    }

    pub fn lines(&self) -> usize {
        return self.lines
    }

    pub fn iter(&self) -> CellIterator {
        let max = self.columns * self.lines;
        CellIterator{ maze: self, i: 0, max }
    }

    fn is_out(&self, x: usize, y: usize) -> bool {
        y >= self.lines || x >= self.columns
    }

    fn localize(&self, x: usize, y: usize) -> usize {
        y * self.columns + x
    }

    pub fn can_move_down(&self, x: usize, y: usize) -> bool {
        if self.is_out(x, y) {
            false
        } else {
            let index = self.localize(x, y);
            let gate = self.gates[index];
            gate.contains(GateWay::VERT)
        }
    }

    pub fn can_move_right(&self, x: usize, y: usize) -> bool {
        if self.is_out(x, y) {
            false
        } else {
            let index = self.localize(x, y);
            let gate = self.gates[index];
            gate.contains(GateWay::HORI)
        }
    }
}


impl Debug for Maze {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Maze{{{}x{}}}", self.columns, self.lines)
    }
}


#[derive(Debug)]
pub struct CellInfo<'a> {
    pub column: usize,
    pub line: usize,
    pub maze: &'a Maze
}


impl<'a> CellInfo<'a> {
    pub fn can_move_down(&self) -> bool {
        return self.maze.can_move_down(self.column, self.line)
    }
    
    pub fn can_move_right(&self) -> bool {
        return self.maze.can_move_right(self.column, self.line)
    }
}


pub struct CellIterator<'a> {
    maze: &'a Maze,
    i: usize,
    max: usize
}


impl<'a> Debug for CellIterator<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let x = self.i / self.maze.columns();
        let y = (self.i - x) % self.maze.lines();
        write!(f, "CellIterator{{{} => {},{}}}", self.i, x, y)
    }
}


impl<'a> Iterator for CellIterator<'a> {
    type Item = CellInfo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.max {
            None
        } else {
            let y = self.i / self.maze.columns();
            let x = self.i - y * self.maze.columns();

            self.i += 1;

            Some(CellInfo{
                column: x,
                line: y,
                maze: self.maze
            })
        }
    }
}
