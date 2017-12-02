extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::input::{RenderArgs, UpdateArgs, Button, Key};
use opengl_graphics::{ GlGraphics };

use super::maze::Maze;
use super::maze_render::{MazeRenderer, StaticMazeRenderer};

pub struct App {
    gl: GlGraphics,
    maze: Maze,
    idle_period: f64, // second
    run_algo: bool,
    algo_idle: f64 // second
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App { 
            gl, 
            maze: Maze::new(6, 4),
            idle_period: 1.0,
            run_algo: false,
            algo_idle: 0.0
        }
    }

    fn reset_maze(&mut self) {
        println!("Reset maze");
        let (w, h) = (self.maze.columns(), self.maze.lines());
        self.maze = Maze::new(w, h);
        self.run_algo = false;
        self.algo_idle= 0.0;
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
        if self.run_algo {
            self.algo_idle += args.dt;

            if self.algo_idle >= self.idle_period {
                self.commit_one();
                self.algo_idle = self.algo_idle % self.idle_period;
            }
        }
    }

    pub fn button_pressed(&mut self, args: &Button) {
        match *args {
            Button::Keyboard(key) if key == Key::Space => {
                self.run_algo = !self.run_algo;

                if self.run_algo {
                    println!("Resume algo");
                    self.commit_one();
                } else {
                    println!("Pause execution");
                    self.algo_idle = 0.0;
                }
            },
            Button::Keyboard(key) if key == Key::Backspace => {
                self.reset_maze();
            },
            _ => ()
        }
    }

    fn commit_one(&mut self) {
        println!("Compute!");
    }
}
