use super::{Loc, LocGenerator};


pub struct Grid<T> {
    columns: usize,
    lines: usize,
    cells: Vec<T>
}


impl<T: Default> Grid<T> {
    pub fn new(columns: usize, lines: usize) -> Grid<T> {
        let count = (columns * lines) as usize;
        let mut cells = Vec::with_capacity(count);
        for _ in 0..count {
            cells.push(Default::default());
        }
        Grid{ columns, lines, cells }
    }
}


impl<T: Clone> Grid<T> {
    pub fn new_from_copy(columns: usize, lines: usize, model: &T) -> Grid<T> {
        let count = (columns * lines) as usize;
        let mut cells = Vec::with_capacity(count);
        for _ in 0..count {
            cells.push(model.clone());
        }
        Grid{ columns, lines, cells }
    }
}


impl<T> Grid<T> {
    pub fn new_by<F>(columns: usize, lines: usize, create: F) -> Grid<T> 
        where F: Fn() -> T
    {
        let count = (columns * lines) as usize;
        let mut cells = Vec::with_capacity(count);
        for _ in 0..count {
            cells.push(create());
        }
        Grid{ columns, lines, cells }
    }
    
    pub fn new_init_with<F>(columns: usize, lines: usize, create: &mut F) -> Grid<T> 
        where F: FnMut() -> T
    {
        let count = (columns * lines) as usize;
        let mut cells = Vec::with_capacity(count);
        for _ in 0..count {
            cells.push(create());
        }
        Grid{ columns, lines, cells }
    }
}


impl<T> Grid<T> {
    pub fn columns(&self) -> usize {
        self.columns
    }

    
    pub fn lines(&self) -> usize {
        self.lines
    }


    fn _storage_pos(&self, column: usize, line: usize) -> Option<usize> {
        if column < self.columns && line < self.lines {
            Some(column + self.columns * line)
        } else {
            None
        }
    }

    pub fn at<'m>(&'m self, column: usize, line: usize) -> &'m T {
        let spos = self._storage_pos(column, line)
            .expect("coordinates exists");
        &self.cells[spos]
    }

    
    pub fn at_loc<'m>(&'m self, loc: &Loc<T>) -> &'m T {
        let spos = loc.storage_pos();
        &self.cells[spos]
    }
    
    
    pub fn try_at<'m>(&'m self, column: usize, line: usize) -> Option<&'m T> {
        self._storage_pos(column, line)
            .map_or(None, |spos| self.cells.get(spos))
    }


    pub fn try_at_loc<'m>(&'m self, loc: &Loc<T>) -> Option<&'m T> {
        let spos = loc.storage_pos();
        self.cells.get(spos)
    }


    pub fn at_mut<'m>(&'m mut self, column: usize, line: usize) -> &'m mut T {
        let spos = self._storage_pos(column, line)
            .expect("coordinates exists");
        &mut self.cells[spos]
    }

    
    pub fn at_loc_mut<'m>(&'m mut self, loc: &Loc<T>) -> &'m mut T {
        let spos = loc.storage_pos();
        &mut self.cells[spos]
    }
    
    
    pub fn try_at_mut<'m>(&'m mut self, column: usize, line: usize) -> Option<&'m mut T> {
        self._storage_pos(column, line)
            .map_or(None, move |spos| self.cells.get_mut(spos))
    }


    pub fn try_at_loc_mut<'m>(&'m mut self, loc: &Loc<T>) -> Option<&'m mut T> {
        let spos = loc.storage_pos();
        self.cells.get_mut(spos)
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;

    use super::*;

   
    const NB_COLUMNS : usize = 4;
    const NB_LINES   : usize = 5;
   
   
    // == BUILD ==


    #[test]
    fn build_grid_with_default_value() {
        #[derive(PartialEq, Debug)]
        struct Dummy(i32, char);

        impl Default for Dummy {
            fn default() -> Dummy {
                Dummy(77, 'M')
            }
        }

        let def_value = Dummy::default();
        let grid : Grid<Dummy> = Grid::new(NB_COLUMNS, NB_LINES);
        
        assert_eq!(grid.columns(), NB_COLUMNS);
        assert_eq!(grid.lines(), NB_LINES);
        assert_eq!(grid.at(0, 0), &def_value);
    }
   
    
    #[test]
    fn build_grid_with_initial_value() {
        #[derive(Clone, PartialEq, Debug)]
        struct Dummy(i32, char);

        let init_value = Dummy(42, '*');
        let grid : Grid<Dummy> = Grid::new_from_copy(
            NB_COLUMNS, NB_LINES, &init_value);
        
        assert_eq!(grid.columns(), NB_COLUMNS);
        assert_eq!(grid.lines(), NB_LINES);
        assert_eq!(grid.at(0, 0), &init_value);
    }
   
    
    #[test]
    fn build_grid_with_non_mutable_function() {
        let func_const = || 112;
        let value = 112;

        let grid : Grid<usize> = Grid::new_by(
            NB_COLUMNS, NB_LINES, func_const);
        
        assert_eq!(grid.columns(), NB_COLUMNS);
        assert_eq!(grid.lines(), NB_LINES);
        assert_eq!(grid.at(0, 0), &value);
        assert_eq!(grid.at(NB_COLUMNS - 1, NB_LINES - 1), &value);
    }
   
    
    #[test]
    fn build_grid_with_mutable_function() {
        let inital = 0;
        let mut counter = inital;
        let mut func = || { 
            let v = counter;
            counter += 1;
            v 
        };

        let grid : Grid<usize> = Grid::new_init_with(
            NB_COLUMNS, NB_LINES, &mut func);
        let last = func() - 1;
        
        assert_eq!(grid.columns(), NB_COLUMNS);
        assert_eq!(grid.lines(), NB_LINES);
        assert_eq!(grid.at(0, 0), &inital);
        assert_eq!(grid.at(NB_COLUMNS - 1, NB_LINES - 1), &last);
    }


    // == read-only direct access ==


    #[test]
    #[should_panic]
    fn panic_if_direct_access_value_outside_column_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        grid.at(NB_COLUMNS, 0);
    }

    
    #[test]
    #[should_panic]
    fn panic_if_direct_access_value_outside_line_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        grid.at(0, NB_LINES);
    }

    
    #[test]
    fn access_with_location_within_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        let grid_rc = Rc::new(RefCell::new(grid));
        let locgen = LocGenerator::new(&grid_rc);
        let loc = locgen.create_from_coordinates(0, 0);
        assert_eq!(grid_rc.borrow().at_loc(&loc), &0);
    }

    
    #[test]
    #[should_panic]
    fn panic_if_access_with_location_outside_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        let grid_rc = Rc::new(RefCell::new(grid));
        let locgen = LocGenerator::new(&grid_rc);
        let loc = locgen.create_from_coordinates(NB_COLUMNS, NB_LINES);
        assert_eq!(grid_rc.borrow().at_loc(&loc), &0);
    }
   

    // == read-only optional access ==
   

    #[test]
    fn try_access_value_inside_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        assert_eq!(grid.try_at(0, 0), Some(&0));
    }

    
    #[test]
    fn try_access_value_beyond_column_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        assert_eq!(grid.try_at(NB_COLUMNS, 0), None);
    }

    
    #[test]
    fn try_access_value_beyond_line_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        assert_eq!(grid.try_at(0, NB_LINES), None);
    }

    
    #[test]
    fn try_access_with_location_within_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        let grid_rc = Rc::new(RefCell::new(grid));
        let locgen = LocGenerator::new(&grid_rc);
        let loc = locgen.create_from_coordinates(0, 0);
        assert_eq!(grid_rc.borrow().try_at_loc(&loc), Some(&0));
    }
    
    
    #[test]
    fn try_access_with_location_outside_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        let grid_rc = Rc::new(RefCell::new(grid));
        let locgen = LocGenerator::new(&grid_rc);
        let loc = locgen.create_from_coordinates(NB_COLUMNS, NB_LINES);
        assert_eq!(grid_rc.borrow().try_at_loc(&loc), None);
    }


    // == writable by direct access ==
    
   
    #[test]
    fn mutate_value_within_range() {
        let mut grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        assert_eq!(grid.at_mut(0, 0), &0);
        *grid.at_mut(0, 0) = 12;
        assert_eq!(grid.at_mut(0, 0), &12);
    }
   

    #[test]
    #[should_panic]
    fn panic_if_mutate_value_outside_column_range() {
        let mut grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        *grid.at_mut(NB_COLUMNS, 0) = 1;
    }

    
    #[test]
    #[should_panic]
    fn panic_if_mutate_value_outside_line_range() {
        let mut grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        *grid.at_mut(0, NB_LINES) = 1;
    }

    
    #[test]
    fn mutate_with_location_within_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        let grid_rc = Rc::new(RefCell::new(grid));
        let locgen = LocGenerator::new(&grid_rc);
        let loc = locgen.create_from_coordinates(0, 0);
        assert_eq!(grid_rc.borrow().at_loc(&loc), &0);
        *grid_rc.borrow_mut().at_loc_mut(&loc) = 1;
        assert_eq!(grid_rc.borrow().at_loc(&loc), &1);
    }

    
    #[test]
    #[should_panic]
    fn panic_if_mutate_with_location_outside_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        let grid_rc = Rc::new(RefCell::new(grid));
        let locgen = LocGenerator::new(&grid_rc);
        let loc = locgen.create_from_coordinates(NB_COLUMNS, NB_LINES);
        *grid_rc.borrow_mut().at_loc_mut(&loc) = 1;
    }

    
    // == writable by optional access ==
    
   
    #[test]
    fn try_mutate_value_inside_range() {
        let mut grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        assert_eq!(grid.try_at(0, 0), Some(&0));
        grid.try_at_mut(0, 0).map(|var| *var = 1);
        assert_eq!(grid.try_at(0, 0), Some(&1));
    }

    
    #[test]
    fn try_mutate_value_beyond_column_range() {
        let mut grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        assert_eq!(grid.try_at_mut(NB_COLUMNS, 0), None);
    }

    
    #[test]
    fn try_mutate_value_beyond_line_range() {
        let mut grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        assert_eq!(grid.try_at_mut(0, NB_LINES), None);
    }

    
    #[test]
    fn try_mutate_with_location_within_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        let grid_rc = Rc::new(RefCell::new(grid));
        let locgen = LocGenerator::new(&grid_rc);
        let loc = locgen.create_from_coordinates(0, 0);
        assert_eq!(grid_rc.borrow().try_at_loc(&loc), Some(&0));
        grid_rc.borrow_mut().try_at_loc_mut(&loc).map(|var| *var = 1);
        assert_eq!(grid_rc.borrow().try_at_loc(&loc), Some(&1));
    }
    
    
    #[test]
    fn try_mutate_with_location_outside_range() {
        let grid : Grid<usize> = Grid::new(NB_COLUMNS, NB_LINES);
        let grid_rc = Rc::new(RefCell::new(grid));
        let locgen = LocGenerator::new(&grid_rc);
        let loc = locgen.create_from_coordinates(NB_COLUMNS, NB_LINES);
        assert_eq!(grid_rc.borrow().try_at_loc(&loc), None);
        grid_rc.borrow_mut().try_at_loc_mut(&loc).map(|var| *var = 1);
    }
}
