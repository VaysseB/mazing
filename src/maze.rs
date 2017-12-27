use super::grid::{Grid, ZWalk, FreeWalk};


pub struct MazeCell {
    down_gate_open: bool,
    right_gate_open: bool,
    visited: bool,
    height: Option<bool>
}


impl Default for MazeCell {
    fn default() -> MazeCell {
        MazeCell {
            down_gate_open: false,
            right_gate_open: false,
            visited: false,
            height: None
        }
    }
}


//-----------------------------------------------------------------------------


pub struct OrthoMaze {
    grid: Grid<MazeCell>,
    current: Option<usize>,
    group: Vec<usize>
}


impl OrthoMaze {
    pub fn new(w: usize, h: usize) -> OrthoMaze {
        OrthoMaze { 
            grid: Grid::new(w, h),
            current: None,
            group: Vec::new()
        }
    }

    pub fn zwalk<'m>(&'m self) -> ZWalk<'m, MazeCell> {
        ZWalk::new(&self.grid)
    }

    pub fn freewalk<'m>(&'m self) -> FreeWalk<'m, MazeCell> {
        FreeWalk::new(&self.grid)
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
