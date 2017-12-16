use super::grid::{Grid, Within, Pos, PosMut};


enum GateWay {
    HORIZONTAL,
    VERTICAL
}

pub enum Visitation {
    None,
    Partial(usize),
    Complete
}


//-----------------------------------------------------------------------------


pub struct CellStatus {
    open_gate_hori: bool,
    open_gate_vert: bool,
    active: bool,
    current: bool,
    visited: bool
}


impl Default for CellStatus {
    fn default() -> CellStatus {
        CellStatus{
            open_gate_hori: false,
            open_gate_vert: false,
            active: false,
            current: false,
            visited: false
        }
    }
}


//-----------------------------------------------------------------------------


pub struct OrthoMaze {
    grid: Grid<CellStatus>
}


impl OrthoMaze {
    pub fn new(w: usize, h: usize) -> OrthoMaze {
        OrthoMaze { grid: Grid::new(w, h) }
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
                    self.grid.at_mut(start_x, start_y) {
                        cell.open_gate_vert = true;
                    }
            }
            Some(GateWay::VERTICAL) if end_y < start_y => {
                if let Some(ref mut cell) =
                    self.grid.at_mut(end_x, end_y) {
                        cell.open_gate_vert = true;
                    }
            }
            Some(GateWay::HORIZONTAL) if start_x < end_x => {
                if let Some(ref mut cell) =
                    self.grid.at_mut(start_x, start_y) {
                        cell.open_gate_hori = true;
                    }
            }
            Some(GateWay::HORIZONTAL) if end_x < start_x => {
                if let Some(ref mut cell) =
                    self.grid.at_mut(end_x, end_y) {
                        cell.open_gate_hori = true;
                    }
            }
            _ => ( println!("Failed to carve between {}:{} and {}:{}",
                            start_x, start_y, end_x, end_y) )
        }
    }

    #[allow(dead_code)]
    pub fn visitation(&self) -> Visitation {
        let mut visit = 0;
        
        for cell in self.grid.iter() {
            if cell.is_visited() {
                visit += 1;
            }
        }

        if visit == 0 {
            Visitation::None
        } else if visit == self.grid.cell_count() {
            Visitation::Complete
        } else {
            Visitation::Partial(visit)
        }
    }

    pub fn is_visitation_complete(&self) -> bool {
        for cell in self.grid.iter() {
            if !cell.is_visited() {
                return false;
            }
        }

        true
    }
}


pub type WithinOrthoMaze = Within<CellStatus>;


impl Within<CellStatus> for OrthoMaze {
    fn grid<'a>(&'a self) -> &'a Grid<CellStatus> {
        &self.grid
    }

    fn grid_mut<'a>(&'a mut self) -> &'a mut Grid<CellStatus> {
        &mut self.grid
    }
}


//-----------------------------------------------------------------------------


impl<'a> Pos<'a, CellStatus> {
    pub fn reachable_neighbours<'b>(&'b self) -> Vec<Pos<'b, CellStatus>> {
        self.neighbours()
            .into_iter()
            .filter(|ref pos| self.can_move_to(&pos))
            .collect()
    }

    pub fn can_move_to(&self, pos: &Self) -> bool {
        if self.column == pos.column && self.line.wrapping_sub(1) == pos.line {
            // `self` is bellow `pos`
            pos.can_move_down()
        } else if self.column == pos.column && self.line == pos.line.wrapping_sub(1) {
            // `self` is above `pos`
            self.can_move_down()
        } else if self.line == pos.line && self.column.wrapping_sub(1) == pos.column {
            // `self` is at the right of `pos`
            pos.can_move_right()
        } else if self.line == pos.line && self.column == pos.column.wrapping_sub(1) {
            // `self` is at the left of `pos`
            self.can_move_right()
        } else {
            false
        }
    }

    pub fn can_move_down(&self) -> bool {
        self.grid.at(self.column, self.line)
            .map(|ref cell| cell.open_gate_vert)
            .unwrap_or(false)
    }

    pub fn can_move_right(&self) -> bool {
        self.grid.at(self.column, self.line)
            .map(|ref cell| cell.open_gate_hori)
            .unwrap_or(false)
    }

    pub fn is_active(&self) -> bool {
        self.grid.at(self.column, self.line)
            .map(|ref cell| cell.active)
            .unwrap_or(false)
    }

    pub fn is_current(&self) -> bool {
        self.grid.at(self.column, self.line)
            .map(|ref cell| cell.current)
            .unwrap_or(false)
    }

    pub fn is_visited(&self) -> bool {
        self.grid.at(self.column, self.line)
            .map(|ref cell| cell.visited)
            .unwrap_or(false)
    }
}


impl<'a> PosMut<'a, CellStatus> {
    pub fn mark_current(&mut self) {
        self.grid.at_mut(self.column, self.line)
            .map(|ref mut cell| cell.current = true);
    }

    pub fn unmark_current(&mut self) {
        self.grid.at_mut(self.column, self.line)
            .map(|ref mut cell| cell.current = false);
    }

    pub fn mark_active(&mut self) {
        self.grid.at_mut(self.column, self.line)
            .map(|ref mut cell| cell.active = true);
    }

    pub fn unmark_active(&mut self) {
        self.grid.at_mut(self.column, self.line)
            .map(|ref mut cell| cell.active = false);
    }

    pub fn mark_visit(&mut self) {
        self.grid.at_mut(self.column, self.line)
            .map(|ref mut cell| cell.visited = true);
    }

    pub fn unmark_visit(&mut self) {
        self.grid.at_mut(self.column, self.line)
            .map(|ref mut cell| cell.visited = false);
    }
}
