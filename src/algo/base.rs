use std::rc::Rc;
use std::cell::RefCell;

use super::super::grid::Within;
use super::super::maze::OrthoMaze;
use super::super::depth::OrthoDepthMap;


pub struct Args {
    pub maze: Rc<RefCell<OrthoMaze>>,
    pub depth_map: Rc<RefCell<OrthoDepthMap>>
}


pub struct Walker {
    pub x: usize,
    pub y: usize
}


impl Walker {
    pub fn to_str(&self) -> String {
        format!("{}:{}", self.x, self.y)
    }
    
    pub fn mark_active(&mut self, maze: &mut OrthoMaze) {
        maze.grid_mut().cell_mut(self.x, self.y).map(|ref mut cell| cell.mark_active());
    }

    pub fn unmark_active(&mut self, maze: &mut OrthoMaze) {
        maze.grid_mut().cell_mut(self.x, self.y).map(|ref mut cell| cell.unmark_active());
    }

    pub fn mark_current(&mut self, maze: &mut OrthoMaze) {
        maze.grid_mut().cell_mut(self.x, self.y).map(|ref mut cell| cell.mark_current());
    }

    pub fn unmark_current(&mut self, maze: &mut OrthoMaze) {
        maze.grid_mut().cell_mut(self.x, self.y).map(|ref mut cell| cell.unmark_current());
    }
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

    pub fn is_on_right_border(&self, maze: &OrthoMaze) -> bool {
        self.x + 1 == maze.grid().columns()
    }

    pub fn is_on_down_border(&self, maze: &OrthoMaze) -> bool {
        self.y + 1 == maze.grid().lines()
    }

    pub fn move_x(&self, x: usize) -> Walker {
        Walker { x, y: self.y }
    }

    pub fn walk_right_then_down(&mut self, maze: &mut OrthoMaze) {
        self.unmark_current(maze);

        self.x += 1;

        if self.x >= maze.grid().columns() {
            self.x = 0;
            self.y += 1;
        }
        
        if !self.is_done_walking_right_then_down(maze) {
            self.mark_current(maze);
        }
    }
    
    pub fn is_done_walking_right_then_down(&self, maze: &OrthoMaze) -> bool {
        self.y >= maze.grid().lines() || self.x >= maze.grid().columns()
    }
}
