extern crate rand;

use super::maze::Maze;

#[derive(Debug, PartialEq)]
pub enum CarveStatus {
    Done,
    Continuing
}


struct Logger {
    name: &'static str,
}


impl Logger {
    fn info<T>(&self, algo: &BinaryTree, msg: T) 
        where T: Into<&'static str>
    {
        println!("[{}] At {}:{}  {}",
                 self.name,
                 algo.pos.x(),
                 algo.pos.y(),
                 msg.into());
    }
}


pub struct Walker {
    x: usize,
    y: usize
}


impl Walker {
    pub fn new() -> Walker {
        Walker { 
            x: 0, 
            y: 0,
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    fn carve_right(&self, maze: &mut Maze) {
        maze.carve(self.x, self.y, self.x + 1, self.y);
    }

    fn carve_down(&self, maze: &mut Maze) {
        maze.carve(self.x, self.y, self.x, self.y + 1);
    }

    fn is_on_right_border(&self, maze: &Maze) -> bool {
        self.x + 1 == maze.columns()
    }

    fn is_on_down_border(&self, maze: &Maze) -> bool {
        self.y + 1 == maze.lines()
    }

    fn walk_right_then_down(&mut self, maze: &Maze) {
        self.x += 1;

        if self.x >= maze.columns() {
            self.x = 0;
            self.y += 1;
        }
    }
    
    fn is_done_walking_right_then_down(&self, maze: &Maze) -> bool {
        self.y == maze.lines() - 1 && self.x == maze.columns() - 1
    }
}


pub struct BinaryTree {
    pub pos: Walker,
    log: Logger
}


impl BinaryTree {
    pub fn new() -> BinaryTree {
        let pos = Walker::new();
        let log = Logger { name: "BinaryTree" };
        BinaryTree { pos, log }
    }

    pub fn carve_one(&mut self, maze: &mut Maze) -> CarveStatus {
        if self.pos.is_done_walking_right_then_down(maze) {
            self.log.info(self, "Done");
            return CarveStatus::Done;
        } else if self.pos.is_on_down_border(maze) {
            self.log.info(self, "Forced carve right");
            self.pos.carve_right(maze);
        } else if self.pos.is_on_right_border(maze) {
            self.log.info(self, "Forced carve down");
            self.pos.carve_down(maze);
        } else {
            use carving::rand::Rng;

            let vert = rand::thread_rng().next_f32() < 0.5;
            if vert {
                self.log.info(self, "Forced carve down");
                self.pos.carve_down(maze);
            } else {
                self.log.info(self, "Forced carve right");
                self.pos.carve_right(maze);
            }
        }

        self.pos.walk_right_then_down(maze);
        CarveStatus::Continuing
    }
}
