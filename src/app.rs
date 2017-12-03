extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::input::{RenderArgs, UpdateArgs, Button, Key};
use opengl_graphics::{ GlGraphics };

use super::maze::Maze;
use super::maze_render::{MazeRenderer, StaticMazeRenderer};
use super::carving;


pub struct Execution {
    algo: carving::BinaryTree,
    active: bool,
    idle_time: f64 // second
}


pub struct App {
    gl: GlGraphics,
    maze: Maze,
    idle_period: f64, // in second
    exec: Option<Execution>
}


impl App {
    pub fn new(gl: GlGraphics) -> App {
        App {
            gl,
            maze: Maze::new(6, 4),
            idle_period: 1.0,
            exec: None
        }
    }

    fn set_algo(&mut self) {
        println!("Set algorithm");

        self.exec = Some(Execution {
            algo: carving::BinaryTree::new(),
            active: false,
            idle_time: 0.0
        });
    }

    fn reset_maze(&mut self) {
        println!("Reset maze");
        self.exec = None;

        let (w, h) = (self.maze.columns(), self.maze.lines());
        self.maze = Maze::new(w, h);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let cx = args.width as f64 * 0.5;
        let cy = args.height as f64 * 0.5;

        let mut mr = StaticMazeRenderer::new();
        let maze = &self.maze;

        self.gl.draw(args.viewport(), |mut c, gl| {
            clear(color::WHITE, gl);

            c.transform = c.transform.trans(cx, cy);

            mr.render(maze, &c, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if let Some(ref mut exec) = self.exec {
            if exec.active {
                exec.idle_time += args.dt;

                if exec.idle_time >= self.idle_period {
                    exec.algo.carve_one(&mut self.maze);
                    exec.idle_time = exec.idle_time % self.idle_period;
                }
            }
        }
    }

    pub fn button_pressed(&mut self, args: &Button) {
        match *args {
            Button::Keyboard(key) if key == Key::Space => {
                if let Some(ref mut exec) = self.exec {
                    exec.active = !exec.active;

                    if exec.active {
                        println!("Resume algo");
                        exec.idle_time = self.idle_period * 0.5;
                    } else {
                        println!("Pause execution");
                        exec.idle_time = 0.0;
                    }
                }
            },
            Button::Keyboard(key) if key == Key::D1 => {
                self.set_algo();
            },
            Button::Keyboard(key) if key == Key::Backspace => {
                self.reset_maze();
            },
            _ => ()
        }
    }
}
