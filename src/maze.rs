use super::grid::{Grid, GridCell};


enum GateWay {
    HORIZONTAL,
    VERTICAL
}


pub struct CellStatus {
    open_gate_hori: bool,
    open_gate_vert: bool,
    active: bool,
    current: bool,
    path_depth: Option<usize>
}


impl Default for CellStatus {
    fn default() -> CellStatus {
        CellStatus{ 
            open_gate_hori: false,
            open_gate_vert: false,
            active: false,
            current: false,
            path_depth: None
        }
    }
}


pub type Maze = Grid<CellStatus>;


impl Maze {
    pub fn is_active(&self, x: usize, y: usize) -> bool {
        self.cell(x, y)
            .map(|ref cell| cell.active)
            .unwrap_or(false)
    }

    pub fn is_current(&self, x: usize, y: usize) -> bool {
        self.cell(x, y)
            .map(|ref cell| cell.current)
            .unwrap_or(false)
    }

    pub fn can_move_down(&self, x: usize, y: usize) -> bool {
        self.cell(x, y)
            .map(|ref cell| cell.open_gate_vert)
            .unwrap_or(false)
    }

    pub fn can_move_right(&self, x: usize, y: usize) -> bool {
        self.cell(x, y)
            .map(|ref cell| cell.open_gate_hori)
            .unwrap_or(false)
    }

    pub fn mark_active(&mut self, x: usize, y: usize) {
        self.cell_mut(x, y)
            .map(|cell| cell.active = true);
    }
 
    pub fn unmark_active(&mut self, x: usize, y: usize) {
        self.cell_mut(x, y)
            .map(|cell| cell.active = false);
    }
 
    pub fn mark_current(&mut self, x: usize, y: usize) {
        self.cell_mut(x, y)
            .map(|cell| cell.current = true);
    }
 
    pub fn unmark_current(&mut self, x: usize, y: usize) {
        self.cell_mut(x, y)
            .map(|cell| cell.current = false);
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
            Some(GateWay::VERTICAL) 
        } else if diff_y == 0 && adj_x {
            Some(GateWay::HORIZONTAL)
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
            Some(GateWay::VERTICAL) if start_y < end_y => {
                if let Some(ref mut cell) = 
                    self.cell_mut(start_x, start_y) {
                        cell.open_gate_vert = true;
                }
            }
            Some(GateWay::VERTICAL) if end_y < start_y => {
                if let Some(ref mut cell) = 
                    self.cell_mut(end_x, end_y) {
                        cell.open_gate_vert = true;
                }
            }
            Some(GateWay::HORIZONTAL) if start_x < end_x => {
                if let Some(ref mut cell) = 
                    self.cell_mut(start_x, start_y) {
                        cell.open_gate_hori = true;
                }
            }
            Some(GateWay::HORIZONTAL) if end_x < start_x => {
                if let Some(ref mut cell) = 
                    self.cell_mut(end_x, end_y) {
                        cell.open_gate_hori = true;
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
