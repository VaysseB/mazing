extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use std::rc::Rc;
use std::cell::RefCell;

use piston::input::{RenderArgs, UpdateArgs, Button, Key};
use opengl_graphics::{GlGraphics};

use super::maze::{OrthoMaze};
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
    
    pub fn create(&self) -> Box<task::Task<algo::base::Args>> {
        match *self {
            Algo::BinaryTree => Box::new(algo::carving::BinaryTree::new()),
            Algo::SideWinder => Box::new(algo::carving::SideWinder::new())
        }
    }
}



struct Execution {
    tasks: task::Executor<algo::base::Args>,
    active: bool,
    idle_period: f64, // in second
    waited_time: f64 // second
}


impl Execution {
    pub fn new(idle_period: f64) -> Execution {
        Execution {
            tasks: task::Executor::new(),
            active: false,
            idle_period, 
            waited_time: 0.0
        }
    }

    pub fn reset(&mut self) {
        self.tasks.clear();
        self.active = false;
        self.waited_time = 0.0;
    }
}


pub struct App {
    gl: GlGraphics,
    last_used_algo: Option<Algo>,
    maze: Rc<RefCell<OrthoMaze>>,
    exec: Execution
}


impl App {
    pub fn new(gl: GlGraphics) -> App {
        let maze = Rc::new(RefCell::new(OrthoMaze::new(6, 4)));
        App {
            gl,
            maze,
            last_used_algo: None,
            exec: Execution::new(0.4)
        }
    }

    fn use_algo(&mut self, type_: Algo) {
        println!("[app] Add task {}", type_.name());

        let algo = type_.create();
        self.last_used_algo = Some(type_);
        
        self.exec.tasks.stack(algo);
    }

    fn reset_maze(&mut self) {
        println!("[app] Reset maze");
        self.exec.reset();
        
        let (w, h);
        {
            let maze = self.maze.borrow();
            w = maze.columns();
            h = maze.lines();
        }
        
        self.maze = Rc::new(RefCell::new(OrthoMaze::new(w, h)));

        if let Some(ref type_) = self.last_used_algo {
            let algo = type_.create();
            self.exec.tasks.stack(algo);
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let cx = args.width as f64 * 0.5;
        let cy = args.height as f64 * 0.5;

        let maze = self.maze.clone();
        let mut mr = StaticMazeRenderer::new();

        self.gl.draw(args.viewport(), |mut c, gl| {
            clear(color::WHITE, gl);

            c.transform = c.transform.trans(cx, cy);

            mr.render(maze, &c, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.exec.active {
            self.commit_one(args.dt);
        }
    }

    fn commit_one(&mut self, dt: f64) {
        self.exec.waited_time += dt;

        if self.exec.waited_time >= self.exec.idle_period {
            let maze = self.maze.clone();
            let args = algo::base::Args { maze };
            self.exec.tasks.run_step(args);
            self.exec.waited_time %= self.exec.idle_period;
        }
    }

    fn commit_all(&mut self) {
        let maze = self.maze.clone();
        let args = algo::base::Args { maze };
        self.exec.tasks.run(args);
    }

    pub fn button_pressed(&mut self, args: &Button) {
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
                self.use_algo(Algo::BinaryTree);
            },
            Button::Keyboard(key) if key == Key::D2 => {
                self.use_algo(Algo::SideWinder);
            },
            Button::Keyboard(key) if key == Key::Backspace => {
                self.reset_maze();
            },
            _ => ()
        }
    }
}
