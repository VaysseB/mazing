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
    fn info(&self, algo: &Algo, msg: &str) {
        println!("[{}] At {}  {}",
                 self.name,
                 Self::format_pos(algo.curr_pos()),
                 msg);
    }

    fn format_pos(pos: &Walker) -> String {
        format!("{}:{}", pos.x(), pos.y())
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

    fn move_x(&self, x: usize) -> Walker {
        Walker { x, y: self.y }
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


pub trait Algo {
    fn curr_pos(&self) -> &Walker;

    fn carve_one(&mut self, maze: &mut Maze) -> CarveStatus;
}


#[derive(Clone)]
pub enum KnownAlgo {
    BinaryTree,
    SideWinder
}


impl KnownAlgo {
    pub fn name(&self) -> &'static str {
        match *self {
            KnownAlgo::BinaryTree => "BinaryTree",
            KnownAlgo::SideWinder => "SideWinder"
        }
    }
    
    pub fn create(&self) -> Box<Algo> {
        match *self {
            KnownAlgo::BinaryTree => Box::new(BinaryTree::new()),
            KnownAlgo::SideWinder => Box::new(SideWinder::new())
        }
    }
}


pub struct BinaryTree {
    pos: Walker,
    log: Logger
}


impl BinaryTree {
    pub fn new() -> BinaryTree {
        let pos = Walker::new();
        let log = Logger { name: "BinaryTree" };
        BinaryTree { pos, log }
    }
}


impl Algo for BinaryTree {
    fn curr_pos(&self) -> &Walker {
        &self.pos
    }

    fn carve_one(&mut self, maze: &mut Maze) -> CarveStatus {
        if self.pos.is_done_walking_right_then_down(maze) {
            self.log.info(self, "Done");
            return CarveStatus::Done;
        } 
        else if self.pos.is_on_down_border(maze) {
            self.log.info(self, "Forced carve right");
            self.pos.carve_right(maze);
        } 
        else if self.pos.is_on_right_border(maze) {
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


pub struct SideWinder {
    pub pos: Walker,
    start_x: usize,
    log: Logger
}


impl SideWinder {
    pub fn new() -> SideWinder {
        let pos = Walker::new();
        let log = Logger { name: "SideWinder" };
        let start_x = pos.x();
        SideWinder { pos, log, start_x }
    }
    
    fn close_group(&mut self, maze: &mut Maze) {
        use carving::rand::Rng;
        
        let door = rand::thread_rng().gen_range(
            self.start_x, self.pos.x() + 1);

        let pos = self.pos.move_x(door);
        pos.carve_down(maze);
        
        self.log.info(self, &format!(
                "Close group, carve down at {}",
                Logger::format_pos(&pos)));
    }
}


impl Algo for SideWinder {
    fn curr_pos(&self) -> &Walker {
        &self.pos
    }

    fn carve_one(&mut self, maze: &mut Maze) -> CarveStatus {
        let mut update_start = false;

        if self.pos.is_done_walking_right_then_down(maze) {
            self.log.info(self, "Done");
            return CarveStatus::Done;
        } 
        else if self.pos.is_on_down_border(maze) {
            self.log.info(self, "Forced carve right");
            self.pos.carve_right(maze);
        } 
        else if self.pos.is_on_right_border(maze) {
            self.close_group(maze);
            update_start = true;
        } 
        else {
            use carving::rand::Rng;

            let build_group = rand::thread_rng().next_f32() < 0.5;
            if build_group {
                self.log.info(self, "Randomly continue group, carve right");
                self.pos.carve_right(maze);
            } else {
                self.close_group(maze);
                update_start = true;
            }
        }

        self.pos.walk_right_then_down(maze);
        
        if update_start {
            self.start_x = self.pos.x();
        }

        CarveStatus::Continuing
    }
}
