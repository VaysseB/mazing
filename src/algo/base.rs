use std::rc::Rc;
use std::cell::RefCell;

use super::super::grid::Address;
use super::super::maze::{OrthoMaze, WithinOrthoMaze};
use super::super::highmap::OrthoHighMap;


pub struct Args {
    pub maze: Rc<RefCell<OrthoMaze>>,
    pub highmap: Rc<RefCell<OrthoHighMap>>
}


impl Address {
    pub fn to_str(&self) -> String {
        format!("{}:{}", self.column, self.line)
    }
    
    pub fn mark_active(&self, maze: &mut WithinOrthoMaze) {
        self.from_mut(maze).map(|ref mut cell| cell.mark_active());
    }

    pub fn unmark_active(&self, maze: &mut WithinOrthoMaze) {
        self.from_mut(maze).map(|ref mut cell| cell.unmark_active());
    }

    pub fn mark_current(&self, maze: &mut WithinOrthoMaze) {
        self.from_mut(maze).map(|ref mut cell| cell.mark_current());
    }

    pub fn unmark_current(&self, maze: &mut WithinOrthoMaze) {
        self.from_mut(maze).map(|ref mut cell| cell.unmark_current());
    }

    pub fn mark_visit(&self, maze: &mut WithinOrthoMaze) {
        self.from_mut(maze).map(|ref mut cell| cell.mark_visit());
    }

    pub fn unmark_visit(&self, maze: &mut WithinOrthoMaze) {
        self.from_mut(maze).map(|ref mut cell| cell.unmark_visit());
    }

    pub fn is_visited(&self, maze: &WithinOrthoMaze) -> bool {
        self.from(maze).map(|ref cell| cell.is_visited()).unwrap_or(false)
    }

    pub fn is_on_right_border(&self, maze: &WithinOrthoMaze) -> bool {
        self.column + 1 == maze.grid().columns()
    }

    pub fn is_on_down_border(&self, maze: &WithinOrthoMaze) -> bool {
        self.line + 1 == maze.grid().lines()
    }

    pub fn move_column(&self, x: usize) -> Address {
        Address { 
            column: x,
            line: self.line
        }
    }

    pub fn walk_right_then_down(&mut self, maze: &mut WithinOrthoMaze) {
        self.unmark_current(maze);

        self.column += 1;

        if self.column >= maze.grid().columns() {
            self.column = 0;
            self.line += 1;
        }
        
        if !self.is_done_walking_right_then_down(maze) {
            self.mark_current(maze);
        }
    }
    
    pub fn is_done_walking_right_then_down(&self, maze: &WithinOrthoMaze) -> bool {
        self.line >= maze.grid().lines() || self.column >= maze.grid().columns()
    }
}
