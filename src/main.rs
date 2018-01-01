extern crate rand;
#[macro_use]
extern crate bitflags;


pub mod grid;
pub mod maze;
pub mod algo;
pub mod display;


fn main() {
    use maze::OrthoMaze;
    use algo::{Execution, BinaryTree};
    use display::{PlainAscii, SymbolSet};

    const NB_COLUMNS : usize = 5;
    const NB_LINES : usize = 4;
    
    let maze = OrthoMaze::new(NB_COLUMNS, NB_LINES);
    let mut exec = Execution::new(maze);
    let bt = BinaryTree();
    assert_eq!(bt.carve(&mut exec), Ok(()));
    let display = PlainAscii::new(SymbolSet::LightWeight);
    let repr = display.draw(&exec.maze).expect("maze is displayable");
    repr.into_iter().for_each(|line| println!("{}", line));
}
