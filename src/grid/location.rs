use super::Grid;
use super::pathwalk;
use super::freewalk;


pub struct Loc<'m, T: 'm> {
    spos: usize,
    grid: &'m Grid<T>
}


impl<'m, T: 'm> Loc<'m, T> {
    pub fn from_storage_pos(spos: usize, grid: &Grid<T>) -> Loc<T> {
        Loc { spos, grid }
    }
    
    
    pub fn from_coordinates(column: usize, line: usize, grid: &Grid<T>) -> Loc<T> {
        let spos = column + line * grid.columns();
        Loc { spos, grid }
    }


    pub fn storage_pos(&self) -> usize {
        self.spos
    }


    pub fn column(&self) -> usize {
        self.spos % self.grid.columns()
    }
    
    
    pub fn line(&self) -> usize {
        self.spos / self.grid.columns()
    }

    
    pub fn value<'z>(&'z self) -> &'z T {
        self.grid.at_loc(self)
    }

    
    pub fn maybe_value<'z>(&'z self) -> Option<&'z T> {
        self.grid.try_at_loc(self)
    }
}


pub trait Localisable<'m, T: 'm> {
    fn to_loc(&'m self) -> Loc<'m, T>;
}
