extern crate rand;

use super::maze::Maze;


pub struct BinaryTree {
    x: usize,
    y: usize
}


impl BinaryTree {
    pub fn new() -> BinaryTree {
        BinaryTree { 
            x: 0, 
            y: 0,
        }
    }

    fn is_done(&self, maze: &Maze) -> bool {
        self.y == maze.lines() - 1 && self.x == maze.columns() - 1
    }

    fn carve_right(&self, maze: &mut Maze) {
        maze.carve(self.x, self.y, self.x + 1, self.y);
    }

    fn carve_down(&self, maze: &mut Maze) {
        maze.carve(self.x, self.y, self.x, self.y + 1);
    }

    fn is_last_column(&self, maze: &Maze) -> bool {
        self.x + 1 == maze.columns()
    }

    fn is_last_row(&self, maze: &Maze) -> bool {
        self.y + 1 == maze.lines()
    }

    fn next(&mut self, maze: &Maze) {
        self.x += 1;

        if self.x >= maze.columns() {
            self.x = 0;
            self.y += 1;
        }
    }

    pub fn carve_one(&mut self, maze: &mut Maze) {
        println!("Carve one!");

        if self.is_done(maze) {
            return;
        } else if self.is_last_row(maze) {
            self.carve_right(maze);
        } else if self.is_last_column(maze) {
            self.carve_down(maze);
        } else {
            use carving::rand::Rng;

            let vert = rand::thread_rng().next_f32() < 0.5;
            if vert {
                self.carve_down(maze);
            } else {
                self.carve_right(maze);
            }
        }

        self.next(maze);
    }
}
