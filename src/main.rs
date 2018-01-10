use std::thread;
use std::sync::{Arc, Mutex};


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
    let exec = Arc::new(Mutex::new(Execution::new(maze)));
    let th_builder = thread::Builder::new();
    
    let bt = BinaryTree();
    let display = PlainAscii::new(SymbolSet::LightWeight);
    
    {
        let exec = exec.clone();
        let handler = th_builder.name("carving".to_owned())
            .spawn(move || {
                let mut exec_guard = exec.lock().expect("nobody panics holding mutex");
                bt.carve(&mut exec_guard)
            })
            .expect("can create named thread");
        let carving_result : thread::Result<Result<(), String>> = handler.join();
        match carving_result {
            Ok(Ok(_)) => (),
            Ok(Err(msg)) => eprintln!("Fail to carve: {}", msg),
            Err(_) => eprintln!("Fail to create carving thread."),
        }
    }
    
    {
        let exec_guard = exec.lock().expect("nobody panics holding mutex");
        let repr = display.draw(&exec_guard.maze).expect("maze is displayable");
        repr.into_iter().for_each(|line| println!("{}", line));
    }
}
