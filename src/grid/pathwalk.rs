use std::iter::Iterator;


use super::{Grid, Localisable, Loc};


pub struct ZWalk<'m, T: 'm> {
    i: usize,
    limit: usize,
    grid: &'m Grid<T>
}


impl<'m, T: 'm> ZWalk<'m, T> {
    pub fn new<'z>(grid: &'z Grid<T>) -> ZWalk<'z, T> {
        let limit = grid.columns() * grid.lines();
        ZWalk {
            i: 0,
            limit,
            grid
        }
    }
}


impl<'m, T: 'm> Iterator for ZWalk<'m, T> {
    type Item = Pos<'m, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.limit {
            None
        } else {
            let spos = self.i;
            self.i += 1;
            Some(Pos{ spos, grid: self.grid })
        }
    }
}


pub struct Pos<'m, T: 'm> {
    spos: usize,
    grid: &'m Grid<T>
}


impl<'m, T: 'm> Pos<'m, T> {
    pub fn column(&self) -> usize {
        self.spos % self.grid.columns()
    }
    
    
    pub fn line(&self) -> usize {
        self.spos / self.grid.columns()
    }
}


impl<'m, T: 'm> Localisable<'m, T> for Pos<'m, T> {
    fn to_loc(&self) -> Loc<'m, T> {
        Loc::from_storage_pos(self.spos, self.grid)
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
        let mut zwalk = ZWalk::new(&grid);
        assert!(zwalk.next().is_some());
    }
    
    
    #[test]
    fn z_walk_ends() {
        let init_value = 1;
        let grid = Grid::new_from_copy(0, 0, &init_value);
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
        let mut zwalk = ZWalk::new(&grid);
        let pos = zwalk.next().expect("position exists");
        let _loc = pos.to_loc();
    }
}
