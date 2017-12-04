#[derive(Debug, PartialEq)]
pub enum AlgoStatus {
    Done,
    Continuing
}


pub struct Logger {
    pub name: &'static str,
}


impl Logger {
    pub fn info(&self, algo: &Algo, msg: &str) {
        println!("[{}] At {}  {}",
                 self.name,
                 Self::format_pos(algo.curr_pos()),
                 msg);
    }

    pub fn format_pos(pos: &Walker) -> String {
        format!("{}:{}", pos.x(), pos.y())
    }
}


pub struct Walker {
    x: usize,
    y: usize
}


use super::maze::Maze;


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

    pub fn carve_right(&self, maze: &mut Maze) {
        maze.carve(self.x, self.y, self.x + 1, self.y);
    }

    pub fn carve_down(&self, maze: &mut Maze) {
        maze.carve(self.x, self.y, self.x, self.y + 1);
    }

    pub fn mark_active(&self, maze: &mut Maze) {
        maze.mark_active(self.x, self.y)
    }

    pub fn unmark_active(&self, maze: &mut Maze) {
        maze.unmark_active(self.x, self.y)
    }

    pub fn mark_current(&self, maze: &mut Maze) {
        maze.mark_current(self.x, self.y)
    }

    pub fn unmark_current(&self, maze: &mut Maze) {
        maze.unmark_current(self.x, self.y)
    }

    pub fn is_on_right_border(&self, maze: &Maze) -> bool {
        self.x + 1 == maze.columns()
    }

    pub fn is_on_down_border(&self, maze: &Maze) -> bool {
        self.y + 1 == maze.lines()
    }

    pub fn move_x(&self, x: usize) -> Walker {
        Walker { x, y: self.y }
    }

    pub fn walk_right_then_down(&mut self, maze: &mut Maze) {
        self.unmark_current(maze);

        self.x += 1;

        if self.x >= maze.columns() {
            self.x = 0;
            self.y += 1;
        }
        
        if !self.is_done_walking_right_then_down(maze) {
            self.mark_current(maze);
        }
    }
    
    pub fn is_done_walking_right_then_down(&self, maze: &Maze) -> bool {
        self.y >= maze.lines() || self.x >= maze.columns()
    }
}


pub trait Algo {
    fn curr_pos(&self) -> &Walker;

    fn carve_one(&mut self, maze: &mut Maze) -> AlgoStatus;
}
