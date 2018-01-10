use maze::{OrthoMaze, OrthoLoc};
use grid::Way;


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

