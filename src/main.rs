use std::thread;
use std::sync::{Arc, Mutex};


extern crate rand;
#[macro_use]
extern crate bitflags;


pub mod grid;
pub mod maze;
pub mod algo;
pub mod display;


use std::thread::JoinHandle;
use std::sync::atomic::{AtomicBool, Ordering};
use algo::{Execution, BinaryTree};
pub struct ThreadExec {
    algo: Option<BinaryTree>,
    exec: Option<Arc<Mutex<Execution>>>,
    done_switch: Arc<AtomicBool>,
    handler: Option<JoinHandle<Result<(), String>>>
}


impl ThreadExec {
    fn new(exec: &Arc<Mutex<Execution>>) -> ThreadExec {
        ThreadExec {
            algo: Some(BinaryTree()),
            exec: Some(exec.clone()),
            done_switch: Arc::new(AtomicBool::new(false)),
            handler: None
        }
    }


    fn start(&mut self) {
        let algo = self.algo.take().expect("there is a algo");
        let exec = self.exec.take().expect("there is an execution");
        let done_switch = self.done_switch.clone();
        
        let builder = thread::Builder::new().name("carving".to_owned());
        self.handler = Some(
            builder.spawn(move || {
                let mut exec_guard = exec.lock()
                    .expect("nobody panics holding mutex");
                let res = algo.carve(&mut exec_guard);

                (*done_switch).store(true, Ordering::Relaxed);

                res
            })
            .expect("can create named thread"));
    }

    
    fn stop(self) {
        match self.handler {
            None => (),
            Some(handler) => 
                match handler.join() {
                    Ok(Ok(_)) => (),
                    Ok(Err(msg)) => eprintln!("Fail to carve: {}", msg),
                    Err(_) => eprintln!("Fail to create carving thread."),
                }
        }
    }


    fn is_done(&self) -> bool {
        (*self.done_switch).load(Ordering::Relaxed)
    }
}


fn main() {
    use std::time::Duration;
    use maze::OrthoMaze;
    use algo::Execution;
    use display::{PlainAscii, SymbolSet};

    const NB_COLUMNS : usize = 5;
    const NB_LINES : usize = 4;
    
    let maze = OrthoMaze::new(NB_COLUMNS, NB_LINES);
    let exec = Arc::new(Mutex::new(Execution::new(maze)));
    
    let display = PlainAscii::new(SymbolSet::LightWeight);
    
    let mut th_exec = ThreadExec::new(&exec);
    th_exec.start();
    
    while !th_exec.is_done() {
        let exec_guard = exec.lock().expect("nobody panics holding mutex");
        let repr = display.draw(&exec_guard.maze).expect("maze is displayable");
        repr.into_iter().for_each(|line| println!("{}", line));
        println!("");
        thread::sleep(Duration::from_millis(20));
    }
    
    println!("Now joining");
    th_exec.stop();
    
}
