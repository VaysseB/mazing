#![allow(unused_imports)]

mod base;
pub use self::base::Grid;

mod pathwalk;
pub use self::pathwalk::Walk;

mod freewalk;
pub use self::freewalk::{Walker, Way};

mod location;
pub use self::location::{Loc, Localisable};
