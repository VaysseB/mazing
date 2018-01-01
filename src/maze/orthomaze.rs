use std::rc::Rc;
use std::cell::RefCell;

use grid::{Grid, Way, Border, Loc, LocGenerator, ZWalk, OrthoFreeWalk};


pub type OrthoLoc = Loc<MazeCell>;


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


bitflags! {
    pub struct Gates: usize {
        const TOP   = 0b00001;
        const DOWN  = 0b00010;
        const LEFT  = 0b00100;
        const RIGHT = 0b01000;
    }
}


impl<'a> Into<Gates> for &'a Way {
    fn into(self) -> Gates {
        match *self {
            Way::Up => Gates::TOP,
            Way::Down => Gates::DOWN,
            Way::Left => Gates::LEFT,
            Way::Right => Gates::RIGHT,
        }
    }
}

   
impl Gates {
    pub fn can_move(&self, gateway: &Way) -> bool {
        self.contains(gateway.into())
    }


    pub fn can_move_all(&self, gateways: &[Way]) -> bool {
        gateways.iter().all(|gtw| self.contains(gtw.into()))
    }
}


//-----------------------------------------------------------------------------


pub struct OrthoMaze {
    pub grid: Rc<RefCell<Grid<MazeCell>>>,
    _current: Option<usize>,
    _group: Vec<usize>
}


impl OrthoMaze {
    pub fn new(w: usize, h: usize) -> OrthoMaze {
        let grid = Rc::new(RefCell::new(Grid::new(w, h)));
        OrthoMaze {
            grid,
            _current: None,
            _group: Vec::new()
        }
    }

    
    pub fn loc_generator(&self) -> LocGenerator<MazeCell> {
        LocGenerator::new(&self.grid)
    }
    

    pub fn zwalk(&self) -> ZWalk<MazeCell> {
        ZWalk::new(self.loc_generator())
    }


    pub fn freewalk(&self) -> OrthoFreeWalk<MazeCell> {
        OrthoFreeWalk::new(self.loc_generator())
    }


    pub fn carve(&mut self, loc: &Loc<MazeCell>, gateway: &Way) 
        -> Result<(), String>
    {
        let res = match gateway {
            &Way::Down if loc.line() + 1 <= self.grid.borrow().lines() => 
                self.grid.borrow_mut()
                .try_at_loc_mut(&loc)
                .map(|ocell| ocell.down_gate_open = true),
            &Way::Right if loc.column() + 1 <= self.grid.borrow().columns() => 
                self.grid.borrow_mut()
                .try_at_loc_mut(&loc)
                .map(|ocell| ocell.right_gate_open = true),
            &Way::Up if loc.line() >= 1 => 
                self.grid.borrow_mut()
                .try_at_mut(loc.column(), loc.line() - 1)
                .map(|ocell| ocell.down_gate_open = true),
            &Way::Left if loc.column() >= 1 => 
                self.grid.borrow_mut()
                .try_at_mut(loc.column() - 1, loc.line())
                .map(|ocell| ocell.right_gate_open = true),
            _ => None
        };
        res.ok_or("invalid carving".to_owned())
    }


    pub fn gates_at(&self, loc: &Loc<MazeCell>) -> Gates {
        let mut gates = Gates{ bits: 0 };
        
        self.grid.borrow().try_at_loc(loc).map(|ocell|
            if ocell.down_gate_open {
                gates.insert(Gates::DOWN);
            } else if ocell.right_gate_open {
                gates.insert(Gates::RIGHT);
            });
        
        if !loc.is_close_to(Border::Top) {
            self.grid.borrow().try_at(loc.column(), loc.line() - 1)
                .map(|ocell| if ocell.down_gate_open {
                    gates.insert(Gates::TOP);
                });
        }
        
        if !loc.is_close_to(Border::Left) {
            self.grid.borrow().try_at(loc.column() - 1, loc.line())
                .map(|ocell| if ocell.right_gate_open {
                    gates.insert(Gates::LEFT);
                });
        }
        
        gates
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
