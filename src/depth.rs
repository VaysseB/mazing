use super::grid::{Grid, Within, Pos, PosMut};


pub struct CellStatus {
    depth: Option<usize>
}


impl Default for CellStatus {
    fn default() -> CellStatus {
        CellStatus{ 
            depth: None
        }
    }
}


//-----------------------------------------------------------------------------


pub struct OrthoDepthMap { 
    grid: Grid<CellStatus>,
    max_path_depth: usize
}


impl OrthoDepthMap {
    pub fn new(w: usize, h: usize) -> OrthoDepthMap {
        OrthoDepthMap {
            grid: Grid::new(w, h),
            max_path_depth: 0
        }
    }
}


impl Within<CellStatus> for OrthoDepthMap {
    fn grid<'a>(&'a self) -> &'a Grid<CellStatus> {
        &self.grid
    }
    
    fn grid_mut<'a>(&'a mut self) -> &'a mut Grid<CellStatus> {
        &mut self.grid
    }
}


//-----------------------------------------------------------------------------


impl<'a> Pos<'a, CellStatus> {
    pub fn depth(&self) -> Option<usize> {
        self.grid.at(self.column, self.line)
            .map(|cell| cell.depth)
            .unwrap_or(None)
    }
}
    
impl<'a> PosMut<'a, CellStatus> {
    pub fn set_depth(&mut self, depth: usize) {
        if let Some(ref mut seed) = self.grid.at_mut(self.column, self.line) {
            seed.depth = Some(depth);
        }
    }
}
