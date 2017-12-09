extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::input::{RenderArgs, UpdateArgs, Button, Key};
use opengl_graphics::{GlGraphics};

use super::maze::Maze;
use super::maze_render::{MazeRenderer, StaticMazeRenderer};
use super::algo;
use super::task;


#[derive(Clone)]
pub enum Algo {
    BinaryTree,
    SideWinder
}


impl Algo {
    pub fn name(&self) -> &'static str {
        match *self {
            Algo::BinaryTree => "BinaryTree",
            Algo::SideWinder => "SideWinder"
        }
    }
    
    pub fn create<'app>(&self) -> Box<task::Task<algo::base::Args<'app>>> {
        match *self {
            Algo::BinaryTree => Box::new(algo::carving::BinaryTree::new()),
            Algo::SideWinder => Box::new(algo::carving::SideWinder::new())
        }
    }
}



pub struct Execution<'app> {
    tasks: task::Executor<algo::base::Args<'app>>,
    active: bool,
    idle_period: f64, // in second
    waited_time: f64 // second
}


pub struct App<'app> {
    gl: GlGraphics,
    maze: Maze,
    exec: Execution<'app>
}


impl<'app> App<'app> {
    pub fn new(gl: GlGraphics) -> App<'app> {
        App {
            gl,
            maze: Maze::new(6, 4),
            exec: Execution{
                tasks: task::Executor::new(),
                active: false,
                idle_period: 0.4,
                waited_time: 0.0
            }
        }
    }

    fn add_task(&mut self, algo_type: Algo) {
        println!("[app] Add task {}", algo_type.name());

        let algo = algo_type.create();
        self.exec.tasks.stack(algo);
    }

    fn reset_maze(&mut self) {
        println!("[app] Reset maze");
        self.exec.tasks.clear();

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

    pub fn update(&'app mut self, args: &UpdateArgs) {
        if self.exec.active {
            self.commit_one(args.dt);
        }
    }

    fn commit_one(&'app mut self, dt: f64) {
        self.exec.waited_time += dt;

        if self.exec.waited_time >= self.exec.idle_period {
            let maze = &mut self.maze;
            let args = algo::base::Args { maze };
            self.exec.tasks.run_step(args);
            self.exec.waited_time %= self.exec.idle_period;
        }
    }

    fn commit_all(&'app mut self) {
        let args = algo::base::Args {
            maze: &mut self.maze
        };
        
        self.exec.tasks.run(args);
    }

    pub fn button_pressed(&'app mut self, args: &Button) {
        match *args {
            Button::Keyboard(key) if key == Key::Space => {
                self.exec.active = !self.exec.active;

                if self.exec.active {
                    println!("[app] Resume algo");
                    self.exec.waited_time = self.exec.idle_period;
                } else {
                    println!("[app] Pause execution");
                    self.exec.waited_time = 0.0;
                }
            },
            Button::Keyboard(key) if key == Key::Return => {
                self.commit_all();
            },
            Button::Keyboard(key) if key == Key::D1 => {
                self.add_task(Algo::BinaryTree);
            },
            Button::Keyboard(key) if key == Key::D2 => {
                self.add_task(Algo::SideWinder);
            },
            Button::Keyboard(key) if key == Key::Backspace => {
                self.reset_maze();
            },
            _ => ()
        }
    }
}
