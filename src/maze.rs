use super::grid::{Grid, GridCell};


bitflags! {
    pub struct CellStatus: u32 {
        const GATE_HORI = 0b00000001;
        const GATE_VERT = 0b00000010;
        const ACTIVE    = 0b00000100;
        const CURRENT   = 0b00001000;
    }
}


impl Default for CellStatus {
    fn default() -> CellStatus {
        CellStatus{ bits: 3 }
    }
}


pub type Maze = Grid<CellStatus>;


impl Maze {
    pub fn is_active(&self, x: usize, y: usize) -> bool {
        self.cell(x, y)
            .map(|&cell| cell.contains(CellStatus::ACTIVE))
            .unwrap_or(false)
    }

    pub fn is_current(&self, x: usize, y: usize) -> bool {
        self.cell(x, y)
            .map(|&cell| cell.contains(CellStatus::CURRENT))
            .unwrap_or(false)
    }

    pub fn can_move_down(&self, x: usize, y: usize) -> bool {
        self.cell(x, y)
            .map(|&cell| !cell.contains(CellStatus::GATE_VERT))
            .unwrap_or(false)
    }

    pub fn can_move_right(&self, x: usize, y: usize) -> bool {
        self.cell(x, y)
            .map(|&cell| !cell.contains(CellStatus::GATE_HORI))
            .unwrap_or(false)
    }

    pub fn mark_active(&mut self, x: usize, y: usize) {
        self.cell_mut(x, y)
            .map(|cell| cell.insert(CellStatus::ACTIVE));
    }
 
    pub fn unmark_active(&mut self, x: usize, y: usize) {
        self.cell_mut(x, y)
            .map(|cell| cell.remove(CellStatus::ACTIVE));
    }
 
    pub fn mark_current(&mut self, x: usize, y: usize) {
        self.cell_mut(x, y)
            .map(|cell| cell.insert(CellStatus::CURRENT));
    }
 
    pub fn unmark_current(&mut self, x: usize, y: usize) {
        self.cell_mut(x, y)
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
        if !self.contains(start_x, start_y) {
            return;
        } else if !self.contains(end_x, end_y) {
            return;
        }

        match self.continuity(start_x, start_y, end_x, end_y) {
            Some(CellStatus::GATE_VERT) if start_y < end_y => {
                if let Some(ref mut cell) = 
                    self.cell_mut(start_x, start_y) {
                    cell.remove(CellStatus::GATE_VERT);
                }
            }
            Some(CellStatus::GATE_VERT) if end_y < start_y => {
                if let Some(ref mut cell) = 
                    self.cell_mut(end_x, end_y) {
                    cell.remove(CellStatus::GATE_VERT);
                }
            }
            Some(CellStatus::GATE_HORI) if start_x < end_x => {
                if let Some(ref mut cell) = 
                    self.cell_mut(start_x, start_y) {
                    cell.remove(CellStatus::GATE_HORI);
                }
            }
            Some(CellStatus::GATE_HORI) if end_x < start_x => {
                if let Some(ref mut cell) = 
                    self.cell_mut(end_x, end_y) {
                    cell.remove(CellStatus::GATE_HORI);
                }
            }
            _ => ( println!("Failed to carve") )
        }
    } 
}


impl<'a> GridCell<'a, CellStatus> {
    pub fn can_move_down(&self) -> bool {
        return self.grid.can_move_down(self.column, self.line)
    }
    
    pub fn can_move_right(&self) -> bool {
        return self.grid.can_move_right(self.column, self.line)
    }
    
    pub fn is_active(&self) -> bool {
        return self.grid.is_active(self.column, self.line)
    }
    
    pub fn is_current(&self) -> bool {
        return self.grid.is_current(self.column, self.line)
    }
}
