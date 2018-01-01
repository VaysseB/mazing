use std::iter::Iterator;

use super::{Loc, Localisable, LocGenerator};


pub struct ZWalk<T> {
    i: usize,
    limit: usize,
    locgen: LocGenerator<T>
}


impl<T> ZWalk<T> {
    pub fn new(locgen: LocGenerator<T>) -> ZWalk<T> {
        let limit = locgen.columns() * locgen.lines();
        ZWalk {
            i: 0,
            limit,
            locgen
        }
    }
}


impl<T> Iterator for ZWalk<T> {
    type Item = Loc<T>;

    fn next<'z>(&'z mut self) -> Option<Self::Item> {
        if self.i >= self.limit {
            None
        } else {
            let loc = self.locgen.create_from_storage_pos(self.i);
            self.i += 1;
            Some(loc)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;

    use super::*;
    use super::super::Grid;

    
    const NB_COLUMNS : usize = 4;
    const NB_LINES   : usize = 5;

    
    #[test]
    fn z_walk_is_possible() {
        let init_value = 1;
        let grid = Grid::new_from_copy(NB_COLUMNS, NB_LINES, &init_value);
        let grid = Rc::new(RefCell::new(grid));
        let lc = LocGenerator::new(&grid);
        let mut zwalk = ZWalk::new(lc);
        assert!(zwalk.next().is_some());
    }
    
    
    #[test]
    fn z_walk_ends() {
        let init_value = 1;
        let grid = Grid::new_from_copy(0, 0, &init_value);
        let grid = Rc::new(RefCell::new(grid));
        let lc = LocGenerator::new(&grid);
        let mut zwalk = ZWalk::new(lc);
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
        let grid = Rc::new(RefCell::new(grid));
        let lc = LocGenerator::new(&grid);
        for pos in ZWalk::new(lc) {
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
        let grid = Rc::new(RefCell::new(grid));
        let lc = LocGenerator::new(&grid);
        let mut zwalk = ZWalk::new(lc);
        let _loc : Loc<_> = zwalk.next().expect("position exists");
    }
}
