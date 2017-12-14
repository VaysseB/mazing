use super::grid::{Grid, Within, Pos, PosMut};


#[derive(Debug)]
pub struct CellStatus {
    height: Option<usize>
}


impl Default for CellStatus {
    fn default() -> CellStatus {
        CellStatus{ 
            height: None
        }
    }
}


//-----------------------------------------------------------------------------


pub struct OrthoHighMap { 
    grid: Grid<CellStatus>,
    pub highest: usize
}


impl OrthoHighMap {
    pub fn new(w: usize, h: usize) -> OrthoHighMap {
        OrthoHighMap {
            grid: Grid::new(w, h),
            highest: 0
        }
    }
}


impl Within<CellStatus> for OrthoHighMap {
    fn grid<'a>(&'a self) -> &'a Grid<CellStatus> {
        &self.grid
    }
    
    fn grid_mut<'a>(&'a mut self) -> &'a mut Grid<CellStatus> {
        &mut self.grid
    }
}


pub type WithinOrthoHighMap = Within<CellStatus>;


//-----------------------------------------------------------------------------


impl<'a> Pos<'a, CellStatus> {
    pub fn height(&self) -> Option<usize> {
        self.grid.at(self.column, self.line)
            .map(|cell| cell.height)
            .unwrap_or(None)
    }
}
    
impl<'a> PosMut<'a, CellStatus> {
    pub fn set_depth(&mut self, height: usize) {
        if let Some(ref mut seed) = self.grid.at_mut(self.column, self.line) {
            seed.height = Some(height);
        }
    }
}
