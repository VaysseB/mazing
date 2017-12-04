extern crate rand;

use super::super::maze::Maze;
use algo::base::{AlgoStatus, Algo, Walker, Logger};


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

trait CarvingActions {
    fn carve_right(&self, maze: &mut Maze);
    fn carve_down(&self, maze: &mut Maze);
}

impl CarvingActions for Walker {
    fn carve_right(&self, maze: &mut Maze) {
        maze.carve(self.x, self.y, self.x + 1, self.y);
    }

    fn carve_down(&self, maze: &mut Maze) {
        maze.carve(self.x, self.y, self.x, self.y + 1);
    }
}


impl Algo for BinaryTree {
    fn curr_pos(&self) -> &Walker {
        &self.pos
    }

    fn carve_one(&mut self, maze: &mut Maze) -> AlgoStatus {
        if self.pos.is_done_walking_right_then_down(maze) {
            self.log.info(self, "Done");
            return AlgoStatus::Done;
        } 
        else if self.pos.is_on_down_border(maze) {
            self.log.info(self, "Forced carve right");
            self.pos.carve_right(maze);
        } 
        else if self.pos.is_on_right_border(maze) {
            self.log.info(self, "Forced carve down");
            self.pos.carve_down(maze);
        } else {
            use self::rand::Rng;

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
        AlgoStatus::Continuing
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
        use self::rand::Rng;

        for x in self.start_x..self.pos.x() {
            let pos = self.pos.move_x(x);
            pos.unmark_active(maze);
        }
        
        let door = rand::thread_rng().gen_range(
            self.start_x, self.pos.x() + 1);

        let pos = self.pos.move_x(door);
        
        self.log.info(self, &format!(
                "Close group, carve down at {}",
                Logger::format_pos(&pos)));
        
        pos.carve_down(maze);
    }
    
    fn continue_group(&mut self, maze: &mut Maze) {
        self.pos.mark_active(maze);
        
        self.log.info(self, "Continue group, carve right");
        self.pos.carve_right(maze);
    }
}


impl Algo for SideWinder {
    fn curr_pos(&self) -> &Walker {
        &self.pos
    }

    fn carve_one(&mut self, maze: &mut Maze) -> AlgoStatus {
        let mut update_start = false;

        if self.pos.is_done_walking_right_then_down(maze) {
            return AlgoStatus::Done;
        } 
        else if self.pos.is_on_right_border(maze) {
            self.close_group(maze);
            update_start = true;
        } 
        else if self.pos.is_on_down_border(maze) {
            self.continue_group(maze);
        } 
        else {
            use self::rand::Rng;

            let build_group = rand::thread_rng().next_f32() < 0.5;
            if build_group {
                self.continue_group(maze);
            } else {
                self.close_group(maze);
                update_start = true;
            }
        }

        self.pos.walk_right_then_down(maze);
        
        if update_start {
            self.start_x = self.pos.x();
        }

        if self.pos.is_done_walking_right_then_down(maze) {
            self.log.info(self, "Done");
            AlgoStatus::Done
        } else {
            AlgoStatus::Continuing
        }
    }
}
