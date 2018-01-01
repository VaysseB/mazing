use std::rc::{Rc, Weak};
use std::cell::RefCell;

use super::{Grid, Border};
use super::pathwalk;
use super::freewalk;


pub struct LocGenerator<T> {
    grid: Weak<RefCell<Grid<T>>>
}


impl<T> LocGenerator<T> {
    pub fn new(grid: &Rc<RefCell<Grid<T>>>) -> LocGenerator<T> {
        let grid = Rc::downgrade(&grid);
        LocGenerator { grid }
    }
    
    
    pub fn create_from_storage_pos(&self, spos: usize) -> Loc<T> {
        // TODO define behavior if grid is dropped
        let grid_rc = self.grid.upgrade().expect("grid still exists");
        let line = spos / grid_rc.borrow().columns();
        let column = spos - line * grid_rc.borrow().columns();
        let grid = self.grid.clone();
        Loc { spos, column, line, grid }
    }
    
    
    pub fn create_from_coordinates(&self, column: usize, line: usize) -> Loc<T> {
        // TODO define behavior if grid is dropped
        let grid_rc = self.grid.upgrade().expect("grid still exists");
        let spos = column + line * grid_rc.borrow().columns();
        let grid = self.grid.clone();
        Loc { spos, column, line, grid }
    }


    pub fn columns(&self) -> usize {
        let column;
        {
            // TODO define behavior if grid is dropped
            let grid_rc = self.grid.upgrade().expect("grid still exists");
            column = grid_rc.borrow().columns();
        }
        column
    }


    pub fn lines(&self) -> usize {
        let line;
        {
            // TODO define behavior if grid is dropped
            let grid_rc = self.grid.upgrade().expect("grid still exists");
            line = grid_rc.borrow().lines();
        }
        line
    }
}


pub struct Loc<T> {
    spos: usize,
    column: usize,
    line: usize,
    grid: Weak<RefCell<Grid<T>>>
}


impl<T> Loc<T> {
    pub fn storage_pos(&self) -> usize {
        self.spos
    }


    pub fn column(&self) -> usize {
        self.column
    }
    
    
    pub fn line(&self) -> usize {
        self.line
    }


    pub fn coordinates(&self) -> (usize, usize) {
        (self.column, self.line)
    }

    
    pub fn is_close_to(&self, border: Border) -> bool {
        // TODO define behavior if grid is dropped
        let grid_rc = self.grid.upgrade().expect("grid still exists");
        match border {
            Border::Top => self.line == 0,
            Border::Down => self.line + 1 >= grid_rc.borrow().lines(),
            Border::Left => self.column == 0,
            Border::Right => self.column + 1 >= grid_rc.borrow().columns(),
        }
    }
}


pub trait Localisable<T> {
    fn to_loc(&self) -> Loc<T>;
}


#[cfg(test)]
mod tests {
    use super::*;
    use maze::OrthoMaze;


    const NB_COLUMNS : usize = 4;
    const NB_LINES   : usize = 5;


    #[test]
    fn location_on_left_side_knows_it() {
        let maze = OrthoMaze::new(NB_COLUMNS, NB_LINES);
        let mut pos = maze.freewalk();
        pos.spawn_at(0, 0);
        let loc = pos.to_loc();
        assert_eq!(loc.column(), 0);
        assert_eq!(loc.is_close_to(Border::Right), false);
        assert_eq!(loc.is_close_to(Border::Left), true);
    }


    #[test]
    fn location_on_right_side_knows_it() {
        let maze = OrthoMaze::new(NB_COLUMNS, NB_LINES);
        let mut pos = maze.freewalk();
        pos.spawn_at(NB_COLUMNS - 1, 0);
        let loc = pos.to_loc();
        assert_eq!(loc.column(), NB_COLUMNS - 1);
        assert_eq!(loc.is_close_to(Border::Left), false);
        assert_eq!(loc.is_close_to(Border::Right), true);
    }


    #[test]
    fn location_on_top_side_knows_it() {
        let maze = OrthoMaze::new(NB_COLUMNS, NB_LINES);
        let mut pos = maze.freewalk();
        pos.spawn_at(0, 0);
        let loc = pos.to_loc();
        assert_eq!(loc.line(), 0);
        assert_eq!(loc.is_close_to(Border::Down), false);
        assert_eq!(loc.is_close_to(Border::Top), true);
    }


    #[test]
    fn location_on_down_side_knows_it() {
        let maze = OrthoMaze::new(NB_COLUMNS, NB_LINES);
        let mut pos = maze.freewalk();
        pos.spawn_at(0, NB_LINES - 1);
        let loc = pos.to_loc();
        assert_eq!(loc.line(), NB_LINES - 1);
        assert_eq!(loc.is_close_to(Border::Top), false);
        assert_eq!(loc.is_close_to(Border::Down), true);
    }
}
