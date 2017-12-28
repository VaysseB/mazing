use super::{Grid, Border};
use super::pathwalk;
use super::freewalk;


pub struct LocGenerator {
    pub columns: usize,
    pub lines: usize
}


impl LocGenerator {
    pub fn create_from_storage_pos(&self, spos: usize) -> Loc {
        let line = spos / self.columns;
        let column = spos - line * self.columns;
        Loc { 
            spos, column, line,
            nb_columns: self.columns,
            nb_lines: self.lines,
        }
    }
    
    
    pub fn create_from_coordinates(&self, column: usize, line: usize) -> Loc {
        let spos = column + line * self.columns;
        Loc { 
            spos, column, line,
            nb_columns: self.columns,
            nb_lines: self.lines,
        }
    }


    pub fn columns(&self) -> usize {
        self.columns
    }


    pub fn lines(&self) -> usize {
        self.lines
    }
}


pub struct Loc {
    spos: usize,
    column: usize,
    line: usize,
    nb_columns: usize,
    nb_lines: usize
}


impl Loc {
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
        match border {
            Border::Top => self.line == 0,
            Border::Down => self.line + 1 >= self.nb_lines,
            Border::Left => self.column == 0,
            Border::Right => self.column + 1 >= self.nb_columns,
        }
    }
}


pub trait Localisable {
    fn to_loc(&self) -> Loc;
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
