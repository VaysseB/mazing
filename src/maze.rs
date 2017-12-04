use super::grid::{Grid, GridCell, GridCellMut, GridIterator};


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


pub struct Maze { 
    grid: Grid<CellStatus>,
    max_path_depth: usize
}


impl Maze {
    pub fn new(w: usize, h: usize) -> Maze {
        Maze {
            grid: Grid::new(w, h),
            max_path_depth: 0
        }
    }

    pub fn iter(&self) -> GridIterator<CellStatus> {
        self.grid.iter()
    }

    pub fn columns(&self) -> usize {
        self.grid.columns()
    }

    pub fn lines(&self) -> usize {
        self.grid.lines()
    }

    pub fn at(&self, x: usize, y: usize) -> Option<GridCell<CellStatus>> {
        self.grid.at(x, y)
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> Option<GridCellMut<CellStatus>> {
        self.grid.at_mut(x, y)
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
        if !self.grid.contains(start_x, start_y) {
            return;
        } else if !self.grid.contains(end_x, end_y) {
            return;
        }

        match self.continuity(start_x, start_y, end_x, end_y) {
            Some(GateWay::VERTICAL) if start_y < end_y => {
                if let Some(ref mut cell) = 
                    self.grid.cell_mut(start_x, start_y) {
                        cell.open_gate_vert = true;
                }
            }
            Some(GateWay::VERTICAL) if end_y < start_y => {
                if let Some(ref mut cell) = 
                    self.grid.cell_mut(end_x, end_y) {
                        cell.open_gate_vert = true;
                }
            }
            Some(GateWay::HORIZONTAL) if start_x < end_x => {
                if let Some(ref mut cell) = 
                    self.grid.cell_mut(start_x, start_y) {
                        cell.open_gate_hori = true;
                }
            }
            Some(GateWay::HORIZONTAL) if end_x < start_x => {
                if let Some(ref mut cell) = 
                    self.grid.cell_mut(end_x, end_y) {
                        cell.open_gate_hori = true;
                }
            }
            _ => ( println!("Failed to carve") )
        }
    } 
}


impl<'a> GridCell<'a, CellStatus> {
    pub fn can_move_down(&self) -> bool {
        self.grid.cell(self.column, self.line)
            .map(|ref cell| cell.open_gate_vert)
            .unwrap_or(false)
    }
    
    pub fn can_move_right(&self) -> bool {
        self.grid.cell(self.column, self.line)
            .map(|ref cell| cell.open_gate_hori)
            .unwrap_or(false)
    }
    
    pub fn is_active(&self) -> bool {
        self.grid.cell(self.column, self.line)
            .map(|ref cell| cell.active)
            .unwrap_or(false)
    }
    
    pub fn is_current(&self) -> bool {
        self.grid.cell(self.column, self.line)
            .map(|ref cell| cell.current)
            .unwrap_or(false)
    }
}
    
impl<'a> GridCellMut<'a, CellStatus> {
    pub fn mark_current(&mut self) {
        self.grid.cell_mut(self.column, self.line)
            .map(|ref mut cell| cell.current = true);
    }
    
    pub fn unmark_current(&mut self) {
        self.grid.cell_mut(self.column, self.line)
            .map(|ref mut cell| cell.current = false);
    }
    
    pub fn mark_active(&mut self) {
        self.grid.cell_mut(self.column, self.line)
            .map(|ref mut cell| cell.active = true);
    }
    
    pub fn unmark_active(&mut self) {
        self.grid.cell_mut(self.column, self.line)
            .map(|ref mut cell| cell.active = false);
    }
}
