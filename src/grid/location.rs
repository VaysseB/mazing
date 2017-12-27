use std::rc::Rc;


use super::{Grid, Border};
use super::pathwalk;
use super::freewalk;


pub struct Loc<'m, T: 'm> {
    spos: usize,
    grid: &'m mut Rc<Grid<T>>
}


impl<'m, T: 'm> Loc<'m, T> {
    pub fn from_storage_pos(spos: usize, grid: &mut Rc<Grid<T>>) -> Loc<T> {
        Loc { spos, grid }
    }
    
    
    pub fn from_coordinates(column: usize, line: usize, grid: &mut Rc<Grid<T>>) -> Loc<T> {
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


    pub fn coordinates(&self) -> (usize, usize) {
        (self.column(), self.line())
    }

    
    pub fn value<'z>(&'z self) -> &'z T {
        self.grid.at_loc(self)
    }

    
    pub fn maybe_value<'z>(&'z self) -> Option<&'z T> {
        self.grid.try_at_loc(self)
    }

    
    pub fn value_mut<'z>(&'z mut self) -> &'z mut T {
        let spos = self.storage_pos();
        let grid = Rc::get_mut(&mut self.grid)
            .expect("no-one has ownership over maze's grid");
        grid.direct_at_mut(spos)
    }

    
    pub fn maybe_value_mut<'z>(&'z mut self) -> Option<&'z mut T> {
        let spos = self.storage_pos();
        let grid = Rc::get_mut(&mut self.grid)
            .expect("no-one has ownership over maze's grid");
        grid.try_direct_at_mut(spos)
    }

    
    pub fn is_close_to(&self, border: Border) -> bool {
        let (column, line) = self.coordinates();
        match border {
            Border::Top => line == 0,
            Border::Down => line + 1 >= self.grid.lines(),
            Border::Left => column == 0,
            Border::Right => column + 1 >= self.grid.columns(),
        }
    }
}


pub trait Localisable<T> {
    fn to_loc(&mut self) -> Loc<T>;
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
