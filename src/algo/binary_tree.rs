use rand;
use rand::Rng;

use maze::{OrthoMaze, OrthoLoc};
use grid::{Loc, Way, Localisable, Border};


pub struct Execution {
    pub maze: OrthoMaze,
    history: Vec<String>
}


impl Execution {
    pub fn new(maze: OrthoMaze) -> Execution {
        Execution {
            maze,
            history: Vec::new()
        }
    }


    pub fn carve(&mut self, loc: OrthoLoc, gateway: Way, why: &str) -> Result<(), String> {
        self.maze.carve(&loc, &gateway)?;
        self.history.push(why.to_owned());
        println!("At {}:{}, {}", loc.column(), loc.line(), why);
        Ok(())
    }
}


pub struct BinaryTree();


impl BinaryTree {
    pub fn carve(&self, exec: &mut Execution) -> Result<(), String> {
        let mut rng = rand::thread_rng();
        
        for loc in exec.maze.zwalk() {
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

        let locgen = exec.maze.loc_generator();
        
        // right border should be carved down and left
        let access_tr = exec.maze.gates_at(
            &locgen.create_from_coordinates(NB_COLUMNS - 1, 0));
        assert!(!access_tr.can_move(&Way::Up));
        assert!(access_tr.can_move(&Way::Down));
        assert!(!access_tr.can_move(&Way::Right));
        
        let access_r = exec.maze.gates_at(
            &locgen.create_from_coordinates(NB_COLUMNS - 1, 1));
        assert!(access_r.can_move(&Way::Up));
        assert!(access_r.can_move(&Way::Down));
        assert!(!access_r.can_move(&Way::Right));
        
        // down border should be carved left and right
        let access_dl = exec.maze.gates_at(
            &locgen.create_from_coordinates(0, NB_LINES - 1));
        assert!(!access_dl.can_move(&Way::Down));
        assert!(!access_dl.can_move(&Way::Left));
        assert!(access_dl.can_move(&Way::Right));
        
        let access_d = exec.maze.gates_at(
            &locgen.create_from_coordinates(1, NB_LINES - 1));
        assert!(!access_d.can_move(&Way::Down));
        assert!(access_d.can_move(&Way::Left));
        assert!(access_d.can_move(&Way::Right));
    }
}
