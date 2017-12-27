use rand;
use rand::Rng;
use std::rc::Rc;

use maze::{OrthoMaze, OrthoLoc};
use grid::{Way, Localisable, Border};


pub struct Execution {
    pub maze: Rc<OrthoMaze>,
    history: Vec<String>
}


impl Execution {
    pub fn new(maze: OrthoMaze) -> Execution {
        Execution {
            maze: Rc::new(maze),
            history: Vec::new()
        }
    }


    pub fn carve(&mut self, loc: OrthoLoc, gateway: Way, why: &str) -> Result<(), String> {
        {
            let maze = Rc::get_mut(&mut self.maze)
                .expect("no-one has ownership of maze");
            maze.carve(&loc, &gateway)?;
        }
        self.history.push(why.to_owned());
        println!("At {}:{}, {}", loc.column(), loc.line(), why);
        Ok(())
    }
}


pub struct BinaryTree();


impl BinaryTree {
    pub fn carve(&self, exec: &mut Execution) -> Result<(), String> {
        let mut rng = rand::thread_rng();
        for mut pos in exec.maze.zwalk() {
            let loc = pos.to_loc();
            if loc.is_close_to(Border::Down) {
                exec.carve(loc, Way::Right, 
                           "bottom border, forced to carve right")?;
            }
            else if loc.is_close_to(Border::Right) {
                exec.carve(loc, Way::Down, 
                           "right border, forced to carve down")?;
            }
            else {
                let is_tail = rng.gen();
                if is_tail {
                    exec.carve(loc, Way::Down, 
                               "randomly choose to carve down")?;
                } else {
                    exec.carve(loc, Way::Right, 
                               "randomly choose to carve right")?;
                }
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    
    const NB_COLUMNS : usize = 4;
    const NB_LINES   : usize = 5;

    
    #[test]
    fn binary_tree_is_executable() {
        let maze = OrthoMaze::new(NB_COLUMNS, NB_LINES);
        let mut exec = Execution::new(maze);
        let bt = BinaryTree();
        assert_eq!(bt.carve(&mut exec), Ok(()));
    }

    
    #[test]
    fn binary_tree_is_valid() {
        let maze = OrthoMaze::new(NB_COLUMNS, NB_LINES);
        let mut exec = Execution::new(maze);
        let bt = BinaryTree();
        assert_eq!(bt.carve(&mut exec), Ok(()));
        // TODO check gates on strategic point to verify everything is ok
    }
}
