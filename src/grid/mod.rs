#[derive(Debug, PartialEq, Clone)]
pub enum Way {
    Up,
    Down,
    Left,
    Right
}


#[derive(Debug, PartialEq, Clone)]
pub enum Border {
    Top,
    Down,
    Left,
    Right
}


mod base;
pub use self::base::Grid;

mod pathwalk;
pub use self::pathwalk::ZWalk;

mod freewalk;
pub use self::freewalk::OrthoFreeWalk;

mod location;
pub use self::location::{Loc, Localisable, LocGenerator};
