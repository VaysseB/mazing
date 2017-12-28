use grid::{Grid, Way, Loc, ZWalk, OrthoFreeWalk};


pub struct MazeCell {
    down_gate_open: bool,
    right_gate_open: bool,
    _visited: bool,
    _height: Option<bool>
}


impl Default for MazeCell {
    fn default() -> MazeCell {
        MazeCell {
            down_gate_open: false,
            right_gate_open: false,
            _visited: false,
            _height: None
        }
    }
}


//-----------------------------------------------------------------------------


pub type OrthoLoc = Loc;


//-----------------------------------------------------------------------------


pub struct OrthoMaze {
    pub grid: Grid<MazeCell>,
    _current: Option<usize>,
    _group: Vec<usize>
}


impl OrthoMaze {
    pub fn new(w: usize, h: usize) -> OrthoMaze {
        OrthoMaze {
            grid: Grid::new(w, h),
            _current: None,
            _group: Vec::new()
        }
    }


    pub fn zwalk(&self) -> ZWalk {
        ZWalk::new(self.grid.loc_generator())
    }


    pub fn freewalk(&self) -> OrthoFreeWalk {
        OrthoFreeWalk::new(self.grid.loc_generator())
    }


    pub fn carve(&mut self, loc: &OrthoLoc, gateway: &Way) 
        -> Result<(), String>
    {
        let res = match gateway {
            &Way::Down if loc.line() + 1 <= self.grid.lines() => 
                self.grid
                .try_at_loc_mut(&loc)
                .map(|ocell| ocell.down_gate_open = true),
            &Way::Right if loc.column() + 1 <= self.grid.columns() => 
                self.grid
                .try_at_loc_mut(&loc)
                .map(|ocell| ocell.right_gate_open = true),
            &Way::Up if loc.line() >= 1 => 
                self.grid
                .try_at_mut(loc.column(), loc.line() - 1)
                .map(|ocell| ocell.down_gate_open = true),
            &Way::Left if loc.column() >= 1 => 
                self.grid
                .try_at_mut(loc.column() - 1, loc.line())
                .map(|ocell| ocell.right_gate_open = true),
            _ => None
        };
        res.ok_or("invalid carving".to_owned())
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    const NB_COLUMNS : usize = 4;
    const NB_LINES   : usize = 5;


    #[test]
    fn maze_can_be_build() {
        let _maze = OrthoMaze::new(NB_COLUMNS, NB_LINES);
    }


    #[test]
    fn maze_can_be_walked_in_z_way() {
        let maze = OrthoMaze::new(NB_COLUMNS, NB_LINES);
        let _zwalk = maze.zwalk();
    }


    #[test]
    fn maze_can_be_walked_freely() {
        let maze = OrthoMaze::new(NB_COLUMNS, NB_LINES);
        let _freewalk = maze.freewalk();
    }
}
