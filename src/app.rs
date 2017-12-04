extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::input::{RenderArgs, UpdateArgs, Button, Key};
use opengl_graphics::{GlGraphics};

use super::maze::Maze;
use super::maze_render::{MazeRenderer, StaticMazeRenderer};
use super::algo_base::{Algo, AlgoStatus};
use super::carving;


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
            KnownAlgo::BinaryTree => Box::new(carving::BinaryTree::new()),
            KnownAlgo::SideWinder => Box::new(carving::SideWinder::new())
        }
    }
}



pub struct Execution {
    algo_type: KnownAlgo,
    algo: Box<Algo>,
    active: bool,
    last_status: Option<AlgoStatus>,
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
            idle_period: 0.4,
            exec: None
        }
    }

    fn set_algo(&mut self, algo_type: KnownAlgo) {
        println!("[app] Set algorithm {}", algo_type.name());

        let algo = algo_type.create();

        self.exec = Some(Execution {
            algo_type,
            algo,
            active: false,
            last_status: None,
            idle_time: 0.0
        });
    }

    fn reset_maze(&mut self) {
        println!("[app] Reset maze");
        let algo_type = match self.exec {
            Some(ref exec) => Some(exec.algo_type.clone()),
            None => None
        };
        self.exec = None;

        let (w, h) = (self.maze.columns(), self.maze.lines());
        self.maze = Maze::new(w, h);

        algo_type.map(|t| self.set_algo(t));
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
        self.commit_one(args.dt);
    }

    fn commit_one(&mut self, dt: f64) {
        if let Some(ref mut exec) = self.exec {
            if Self::can_commit(exec) && exec.active {
                exec.idle_time += dt;

                if exec.idle_time >= self.idle_period {
                    let status = exec.algo.carve_one(&mut self.maze);
                    exec.last_status = Some(status);
                    exec.idle_time = exec.idle_time % self.idle_period;
                }
            }
        }
    }

    fn can_commit(exec: &Execution) -> bool {
        match exec.last_status {
            None => true,
            Some(AlgoStatus::Continuing) => true,
            Some(_) => false
        }
    }

    fn commit_all(&mut self) {
        if let Some(ref mut exec) = self.exec {
            while Self::can_commit(exec) {
                let status = exec.algo.carve_one(&mut self.maze);
                exec.last_status = Some(status);
            }
        }
    }

    pub fn button_pressed(&mut self, args: &Button) {
        match *args {
            Button::Keyboard(key) if key == Key::Space => {
                if let Some(ref mut exec) = self.exec {
                    exec.active = !exec.active;

                    if exec.active {
                        println!("[app] Resume algo");
                        exec.idle_time = self.idle_period * 0.5;
                    } else {
                        println!("[app] Pause execution");
                        exec.idle_time = 0.0;
                    }
                }
            },
            Button::Keyboard(key) if key == Key::Return => {
                self.commit_all();
            },
            Button::Keyboard(key) if key == Key::D1 => {
                self.set_algo(KnownAlgo::BinaryTree);
            },
            Button::Keyboard(key) if key == Key::D2 => {
                self.set_algo(KnownAlgo::SideWinder);
            },
            Button::Keyboard(key) if key == Key::Backspace => {
                self.reset_maze();
            },
            _ => ()
        }
    }
}
