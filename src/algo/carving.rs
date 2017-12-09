extern crate rand;

use super::super::maze::Maze;
use super::super::task::{Task, Status};
use algo::base::{Walker, Args};


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


pub struct BinaryTree {
    pos: Walker,
    action: String
}


impl BinaryTree {
    pub fn new() -> BinaryTree {
        let pos = Walker::new();
        BinaryTree { pos, action: String::new() }
    }
    
    fn log_action(&mut self, msg: &str) {
        self.action = format!("At {}, {}", self.pos.to_str(), msg);
    }
}


impl<'a> Task<Args<'a>> for BinaryTree {
    fn name(&self) -> &'static str {
        "BinaryTree"
    }

    fn action<'t>(&'t self) -> Option<&'t String> {
        Some(&self.action)
    }

    fn execute_one(&mut self, args: &mut Args<'a>) -> Status {
        if self.pos.is_done_walking_right_then_down(args.maze) {
            return Status::Done;
        } 
        else if self.pos.is_on_down_border(args.maze) {
            self.log_action("Forced carve right");
            self.pos.carve_right(args.maze);
        } 
        else if self.pos.is_on_right_border(args.maze) {
            self.log_action("Forced carve down");
            self.pos.carve_down(args.maze);
        } else {
            use self::rand::Rng;

            let vert = rand::thread_rng().next_f32() < 0.5;
            if vert {
                self.log_action("Forced carve down");
                self.pos.carve_down(args.maze);
            } else {
                self.log_action("Forced carve right");
                self.pos.carve_right(args.maze);
            }
        }

        self.pos.walk_right_then_down(args.maze);
        Status::Continuing
    }
}


pub struct SideWinder {
    pub pos: Walker,
    start_x: usize,
    action: String
}


impl SideWinder {
    pub fn new() -> SideWinder {
        let pos = Walker::new();
        let start_x = pos.x();
        SideWinder { pos, action: String::new(), start_x }
    }

    fn log_action(&mut self, msg: &str) {
        self.action = format!("At {}, {}", self.pos.to_str(), msg);
    }
    
    fn close_group(&mut self, maze: &mut Maze) {
        use self::rand::Rng;

        for x in self.start_x..self.pos.x() {
            let mut pos = self.pos.move_x(x);
            pos.unmark_active(maze);
        }
        
        let door = rand::thread_rng().gen_range(
            self.start_x, self.pos.x() + 1);

        let pos = self.pos.move_x(door);
        
        self.log_action(&format!("Close group, carve down at {}", pos.to_str()));
        
        pos.carve_down(maze);
    }
    
    fn continue_group(&mut self, maze: &mut Maze) {
        self.pos.mark_active(maze);
        self.log_action("Continue group, carve right");
        self.pos.carve_right(maze);
    }
}


impl<'a> Task<Args<'a>> for SideWinder {
    fn name(&self) -> &'static str {
        "SideWinder"
    }

    fn action<'t>(&'t self) -> Option<&'t String> {
        Some(&self.action)
    }

    fn execute_one(&mut self, args: &mut Args<'a>) -> Status {
        let mut update_start = false;

        if self.pos.is_done_walking_right_then_down(args.maze) {
            return Status::Done;
        } 
        else if self.pos.is_on_right_border(args.maze) {
            self.close_group(args.maze);
            update_start = true;
        } 
        else if self.pos.is_on_down_border(args.maze) {
            self.continue_group(args.maze);
        } 
        else {
            use self::rand::Rng;

            let build_group = rand::thread_rng().next_f32() < 0.5;
            if build_group {
                self.continue_group(args.maze);
            } else {
                self.close_group(args.maze);
                update_start = true;
            }
        }

        self.pos.walk_right_then_down(args.maze);
        
        if update_start {
            self.start_x = self.pos.x();
        }

        if self.pos.is_done_walking_right_then_down(args.maze) {
            Status::Done
        } else {
            Status::Continuing
        }
    }
}
