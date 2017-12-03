use std::fmt::{Debug, Formatter, Error};


bitflags! {
    struct GateWay: u32 {
        const HORI = 0b001;
        const VERT = 0b010;
    }
}


impl Default for GateWay {
    fn default() -> GateWay {
        GateWay{ bits: 3 }
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
        for _ in 0..count {
            gates.push(GateWay::default());
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
 
    fn continuity(
        &self,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize)
        -> Option<GateWay>
    {
        let diff_x = start_x as i32 - end_x as i32;
        let adj_x = diff_x == 1 || diff_x == -1;
        
        let diff_y = start_y as i32 - end_y as i32;
        let adj_y = diff_y == 1 || diff_y == -1;

        if diff_x == 0 && adj_y {
            Some(GateWay::VERT) 
        } else if diff_y == 0 && adj_x {
            Some(GateWay::HORI)
        } else {
            None
        }
    }

    pub fn carve(&mut self,
                 start_x: usize,
                 start_y: usize,
                 end_x: usize,
                 end_y: usize)
    {
        if self.is_out(start_x, start_y) {
            return;
        } else if self.is_out(end_x, end_y) {
            return;
        }

        match self.continuity(start_x, start_y, end_x, end_y) {
            Some(GateWay::VERT) if start_y < end_y => {
                let index = self.localize(start_x, start_y);
                if let Some(ref mut gate) = self.gates.get_mut(index) {
                    gate.remove(GateWay::VERT);
                }
            }
            Some(GateWay::VERT) if end_y < start_y => {
                let index = self.localize(end_x, end_y);
                if let Some(ref mut gate) = self.gates.get_mut(index) {
                    gate.remove(GateWay::VERT);
                }
            }
            Some(GateWay::HORI) if start_x < end_x => {
                let index = self.localize(start_x, start_y);
                if let Some(ref mut gate) = self.gates.get_mut(index) {
                    gate.remove(GateWay::HORI);
                }
            }
            Some(GateWay::HORI) if end_x < start_x => {
                let index = self.localize(end_x, end_y);
                if let Some(ref mut gate) = self.gates.get_mut(index) {
                    gate.remove(GateWay::HORI);
                }
            }
            _ => ( println!("Failed to carve") )
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
