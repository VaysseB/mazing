use super::{Grid, Localisable, Loc, Way};


pub struct FreeWalk<'m, T: 'm> {
    column: usize,
    line: usize,
    grid: &'m Grid<T>
}


impl<'m, T: 'm> FreeWalk<'m, T> {
    pub fn new<'z>(grid: &'z Grid<T>) -> FreeWalk<'z, T> {
        FreeWalk {
            column: 0,
            line: 0,
            grid
        }
    }
    
    
    pub fn column(&self) -> usize {
        self.column
    }
    
    
    pub fn line(&self) -> usize {
        self.line
    }


    pub fn can_move(&self, direction: Way) -> bool {
        let floor = self.grid.lines() - 1;
        let wall = self.grid.columns() - 1;
        (direction == Way::Up && self.line > 0) 
            || (direction == Way::Down && self.line < floor)
            || (direction == Way::Left && self.column > 0)
            || (direction == Way::Right && self.column < wall)
    }

    
    pub fn step_to(&mut self, direction: Way) {
        // Right now, no bound checking (see `can_move`)
        // See later if a `try_step_to -> bool' (with bound check)
        // is useful
        match direction {
            Way::Up => self.line -= 1,
            Way::Down => self.line += 1,
            Way::Left => self.column -= 1,
            Way::Right => self.column += 1
        }
    }
}


impl<'m, T: 'm> Localisable<'m, T> for FreeWalk<'m, T> {
    fn to_loc(&self) -> Loc<'m, T> {
        Loc::from_coordinates(self.column, self.line, self.grid)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    
    const NB_COLUMNS : usize = 4;
    const NB_LINES   : usize = 5;


    fn assert_pos<T>(walker: &FreeWalk<T>, pos: &[usize; 2]) {
        assert_eq!(walker.column(), pos[0]);
        assert_eq!(walker.line(), pos[1]);
    }

    
    #[test]
    fn freewalking_at_origin_can_move_down() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let walker = FreeWalk::new(&grid);
        assert!(walker.can_move(Way::Down));
    }

    
    #[test]
    fn freewalking_at_origin_can_move_right() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let walker = FreeWalk::new(&grid);
        assert!(walker.can_move(Way::Right));
    }

    
    #[test]
    fn freewalking_at_origin_can_not_move_up() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let walker = FreeWalk::new(&grid);
        assert!(!walker.can_move(Way::Up));
    }

    
    #[test]
    fn freewalking_at_origin_can_not_move_left() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let walker = FreeWalk::new(&grid);
        assert!(!walker.can_move(Way::Left));
    }

    
    #[test]
    fn freewalking_moves_to_right() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let mut walker = FreeWalk::new(&grid);
        assert_pos(&walker, &[0, 0]);
        walker.step_to(Way::Right);
        assert_pos(&walker, &[1, 0]);
    }

    
    #[test]
    fn freewalking_moves_to_down() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let mut walker = FreeWalk::new(&grid);
        assert_pos(&walker, &[0, 0]);
        walker.step_to(Way::Down);
        assert_pos(&walker, &[0, 1]);
    }

    
    #[test]
    fn freewalking_moves_to_up() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let mut walker = FreeWalk::new(&grid);
        walker.step_to(Way::Down);
        assert_pos(&walker, &[0, 1]);
        walker.step_to(Way::Up);
        assert_pos(&walker, &[0, 0]);
    }

    
    #[test]
    fn freewalking_moves_to_left() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let mut walker = FreeWalk::new(&grid);
        walker.step_to(Way::Right);
        assert_pos(&walker, &[1, 0]);
        walker.step_to(Way::Left);
        assert_pos(&walker, &[0, 0]);
    }
    
    
    #[test]
    fn walker_is_localizable() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let walker = FreeWalk::new(&grid);
        let _loc = walker.to_loc();
    }
}
