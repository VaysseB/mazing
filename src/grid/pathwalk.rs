use std::rc::Rc;
use std::iter::Iterator;


use super::{Grid, Localisable, Loc};


pub struct ZWalk<T> {
    i: usize,
    limit: usize,
    grid: Rc<Grid<T>>
}


impl<T> ZWalk<T> {
    pub fn new(grid: &Rc<Grid<T>>) -> ZWalk<T> {
        let grid = grid.clone();
        let limit = grid.columns() * grid.lines();
        ZWalk {
            i: 0,
            limit,
            grid
        }
    }
}


impl<T> Iterator for ZWalk<T> {
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.limit {
            None
        } else {
            let spos = self.i;
            self.i += 1;
            Some(Pos{ spos, grid: self.grid.clone() })
        }
    }
}


pub struct Pos<T> {
    spos: usize,
    grid: Rc<Grid<T>>
}


impl<T> Pos<T> {
    pub fn column(&self) -> usize {
        self.spos % self.grid.columns()
    }
    
    
    pub fn line(&self) -> usize {
        self.spos / self.grid.columns()
    }
}


impl<T> Localisable<T> for Pos<T> {
    fn to_loc(&mut self) -> Loc<T> {
        Loc::from_storage_pos(self.spos, &mut self.grid)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    
    const NB_COLUMNS : usize = 4;
    const NB_LINES   : usize = 5;

    
    #[test]
    fn z_walk_is_possible() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let grid = Rc::new(grid);
        let mut zwalk = ZWalk::new(&grid);
        assert!(zwalk.next().is_some());
    }
    
    
    #[test]
    fn z_walk_ends() {
        let init_value = 1;
        let grid = Grid::new_from_copy(0, 0, &init_value);
        let grid = Rc::new(grid);
        let mut zwalk = ZWalk::new(&grid);
        assert!(zwalk.next().is_none());
    }
    
    
    #[test]
    fn z_walk_is_completely_valid() {
        use std::collections::VecDeque;

        #[derive(PartialEq, Debug)]
        struct DummyPos(usize, usize);

        let mut expected_path = VecDeque::new();
        for y in 0..NB_LINES {
            for x in 0..NB_COLUMNS {
                expected_path.push_back(DummyPos(x, y));
            }
        }

        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let grid = Rc::new(grid);
        for pos in ZWalk::new(&grid) {
            let dpos = DummyPos(pos.column(), pos.line());
            let expected_pos = expected_path.pop_front()
                .expect("path is not finished");
            assert_eq!(dpos, expected_pos);
        }
    }
    
    
    #[test]
    fn walk_is_localizable() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let grid = Rc::new(grid);
        let mut zwalk = ZWalk::new(&grid);
        let mut pos = zwalk.next().expect("position exists");
        let _loc = pos.to_loc();
    }
}
