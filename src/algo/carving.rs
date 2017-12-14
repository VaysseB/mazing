extern crate rand;

use super::super::grid::{Address, Within};
use super::super::maze::{OrthoMaze, WithinOrthoMaze};
use super::super::task::{Task, Status};
use algo::base::{Args};


impl Address {
    fn carve_right(&self, maze: &mut OrthoMaze) {
        maze.carve(self.column, self.line, self.column + 1, self.line);
    }

    fn carve_down(&self, maze: &mut OrthoMaze) {
        maze.carve(self.column, self.line, self.column, self.line + 1);
    }
}


pub struct BinaryTree {
    location: Address,
    action: String
}


impl BinaryTree {
    pub fn new(maze: &WithinOrthoMaze) -> BinaryTree {
        let location = maze.grid().crumbs().next().expect("first position exists");
        BinaryTree { 
            location,
            action: String::new()
        }
    }
    
    fn log_action(&mut self, msg: &str) {
        self.action = format!("At {}, {}", self.location.to_str(), msg);
    }
}


impl Task<Args> for BinaryTree {
    fn name(&self) -> &'static str {
        "BinaryTree"
    }

    fn action<'t>(&'t self) -> Option<&'t String> {
        Some(&self.action)
    }

    fn execute_one(&mut self, args: &mut Args) -> Status {
        let mut maze = args.maze.borrow_mut();
        
        if self.location.is_done_walking_right_then_down(&*maze) {
            return Status::Done;
        } 
        else if self.location.is_on_down_border(&*maze) {
            self.log_action("Forced carve right");
            self.location.carve_right(&mut *maze);
        } 
        else if self.location.is_on_right_border(&*maze) {
            self.log_action("Forced carve down");
            self.location.carve_down(&mut *maze);
        } else {
            use self::rand::Rng;

            let vert = rand::thread_rng().next_f32() < 0.5;
            if vert {
                self.log_action("Forced carve down");
                self.location.carve_down(&mut *maze);
            } else {
                self.log_action("Forced carve right");
                self.location.carve_right(&mut *maze);
            }
        }

        self.location.walk_right_then_down(&mut *maze);
        Status::Continuing
    }
}


pub struct SideWinder {
    pub location: Address,
    start_x: usize,
    action: String
}


impl SideWinder {
    pub fn new(maze: &WithinOrthoMaze) -> SideWinder {
        let location = maze.grid().crumbs().next().expect("first position exists");
        let start_x = location.column;
        SideWinder { location, action: String::new(), start_x }
    }

    fn log_action(&mut self, msg: &str) {
        self.action = format!("At {}, {}", self.location.to_str(), msg);
    }
    
    fn close_group(&mut self, maze: &mut OrthoMaze) {
        use self::rand::Rng;

        for column in self.start_x..self.location.column {
            let mut location = self.location.move_column(column);
            location.unmark_active(maze);
        }
        
        let door = rand::thread_rng().gen_range(
            self.start_x, self.location.column + 1);

        let location = self.location.move_column(door);
        
        self.log_action(&format!("Close group, carve down at {}", location.to_str()));
        
        location.carve_down(maze);
    }
    
    fn continue_group(&mut self, maze: &mut OrthoMaze) {
        self.location.mark_active(maze);
        self.log_action("Continue group, carve right");
        self.location.carve_right(maze);
    }
}


impl Task<Args> for SideWinder {
    fn name(&self) -> &'static str {
        "SideWinder"
    }

    fn action<'t>(&'t self) -> Option<&'t String> {
        Some(&self.action)
    }

    fn execute_one(&mut self, args: &mut Args) -> Status {
        let mut maze = args.maze.borrow_mut();
        
        let mut update_start = false;

        if self.location.is_done_walking_right_then_down(&*maze) {
            return Status::Done;
        } 
        else if self.location.is_on_right_border(&*maze) {
            self.close_group(&mut *maze);
            update_start = true;
        } 
        else if self.location.is_on_down_border(&*maze) {
            self.continue_group(&mut *maze);
        } 
        else {
            use self::rand::Rng;

            let build_group = rand::thread_rng().next_f32() < 0.5;
            if build_group {
                self.continue_group(&mut *maze);
            } else {
                self.close_group(&mut *maze);
                update_start = true;
            }
        }

        self.location.walk_right_then_down(&mut *maze);
        
        if update_start {
            self.start_x = self.location.column;
        }

        if self.location.is_done_walking_right_then_down(&*maze) {
            Status::Done
        } else {
            Status::Continuing
        }
    }
}
