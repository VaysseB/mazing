use std::fmt::{Debug, Formatter, Error};


bitflags! {
    pub struct CellStatus: u32 {
        const GATE_HORI = 0b00000001;
        const GATE_VERT = 0b00000010;
        const ACTIVE    = 0b00000100;
        const CURRENT   = 0b00001000;
    }
}


impl CellStatus {
    fn new() -> CellStatus {
        CellStatus{ bits: 3 }
    }
}


pub struct Maze {
    columns: usize,
    lines: usize,
    cells: Vec<CellStatus>
}


impl Maze {
    pub fn new(columns: usize, lines: usize) -> Maze {
        let count = (columns * lines) as usize;
        let mut cells = Vec::with_capacity(count);
        for _ in 0..count {
            cells.push(CellStatus::new());
        }
        Maze {columns, lines, cells}
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

    fn _status_at(&self, x: usize, y: usize) -> Option<&CellStatus> {
        if self.is_out(x, y) {
            None
        } else {
            let index = self.localize(x, y);
            Some(&self.cells[index])
        }
    }

    fn _status_at_mut(&mut self, x: usize, y: usize) -> Option<&mut CellStatus> {
        if self.is_out(x, y) {
            None
        } else {
            let index = self.localize(x, y);
            Some(&mut self.cells[index])
        }
    }

    pub fn is_active(&self, x: usize, y: usize) -> bool {
        self._status_at(x, y)
            .map(|&cell| cell.contains(CellStatus::ACTIVE))
            .unwrap_or(false)
    }

    pub fn is_current(&self, x: usize, y: usize) -> bool {
        self._status_at(x, y)
            .map(|&cell| cell.contains(CellStatus::CURRENT))
            .unwrap_or(false)
    }

    pub fn can_move_down(&self, x: usize, y: usize) -> bool {
        self._status_at(x, y)
            .map(|&cell| !cell.contains(CellStatus::GATE_VERT))
            .unwrap_or(false)
    }

    pub fn can_move_right(&self, x: usize, y: usize) -> bool {
        self._status_at(x, y)
            .map(|&cell| !cell.contains(CellStatus::GATE_HORI))
            .unwrap_or(false)
    }

    pub fn mark_active(&mut self, x: usize, y: usize) {
        self._status_at_mut(x, y)
            .map(|cell| cell.insert(CellStatus::ACTIVE));
    }
 
    pub fn unmark_active(&mut self, x: usize, y: usize) {
        self._status_at_mut(x, y)
            .map(|cell| cell.remove(CellStatus::ACTIVE));
    }
 
    pub fn mark_current(&mut self, x: usize, y: usize) {
        self._status_at_mut(x, y)
            .map(|cell| cell.insert(CellStatus::CURRENT));
    }
 
    pub fn unmark_current(&mut self, x: usize, y: usize) {
        self._status_at_mut(x, y)
            .map(|cell| cell.remove(CellStatus::CURRENT));
    }
 
    fn continuity(
        &self,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize)
        -> Option<CellStatus>
    {
        let diff_x = start_x as i32 - end_x as i32;
        let adj_x = diff_x == 1 || diff_x == -1;
        
        let diff_y = start_y as i32 - end_y as i32;
        let adj_y = diff_y == 1 || diff_y == -1;

        if diff_x == 0 && adj_y {
            Some(CellStatus::GATE_VERT) 
        } else if diff_y == 0 && adj_x {
            Some(CellStatus::GATE_HORI)
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
            Some(CellStatus::GATE_VERT) if start_y < end_y => {
                let index = self.localize(start_x, start_y);
                if let Some(ref mut cell) = self.cells.get_mut(index) {
                    cell.remove(CellStatus::GATE_VERT);
                }
            }
            Some(CellStatus::GATE_VERT) if end_y < start_y => {
                let index = self.localize(end_x, end_y);
                if let Some(ref mut cell) = self.cells.get_mut(index) {
                    cell.remove(CellStatus::GATE_VERT);
                }
            }
            Some(CellStatus::GATE_HORI) if start_x < end_x => {
                let index = self.localize(start_x, start_y);
                if let Some(ref mut cell) = self.cells.get_mut(index) {
                    cell.remove(CellStatus::GATE_HORI);
                }
            }
            Some(CellStatus::GATE_HORI) if end_x < start_x => {
                let index = self.localize(end_x, end_y);
                if let Some(ref mut cell) = self.cells.get_mut(index) {
                    cell.remove(CellStatus::GATE_HORI);
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
    
    pub fn is_active(&self) -> bool {
        return self.maze.is_active(self.column, self.line)
    }
    
    pub fn is_current(&self) -> bool {
        return self.maze.is_current(self.column, self.line)
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
