extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use std::rc::Rc;
use std::cell::RefCell;

use piston::input::{RenderArgs, UpdateArgs, Button, Key};
use opengl_graphics::{GlGraphics};

use super::settings::DEBUG_GATE;
use super::maze::{OrthoMaze, WithinOrthoMaze};
use super::maze_render::{MazeRenderer, StaticMazeRenderer};
use super::highmap::OrthoHighMap;
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
    
    pub fn create(&self, maze: &WithinOrthoMaze) 
        -> Box<task::Task<algo::base::Args>> {
        match *self {
            Algo::BinaryTree => Box::new(algo::carving::BinaryTree::new(maze)),
            Algo::SideWinder => Box::new(algo::carving::SideWinder::new(maze))
        }
    }
}


// ----------------------------------------------------------------------------


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


// ----------------------------------------------------------------------------


pub struct App {
    gl: GlGraphics,
    last_used_algo: Option<Algo>,
    maze: Rc<RefCell<OrthoMaze>>,
    highmap: Rc<RefCell<OrthoHighMap>>,
    exec: Execution
}


impl App {
    pub fn new(gl: GlGraphics) -> App {
        let (w, h);

        if DEBUG_GATE {
            w = 6;
            h = 4;
        } else {
            w = 60;
            h = 40;
        }
        
        let maze = Rc::new(RefCell::new(OrthoMaze::new(w, h)));
        let highmap = Rc::new(RefCell::new(OrthoHighMap::new(w, h)));
        App {
            gl,
            maze,
            highmap,
            last_used_algo: None,
            exec: Execution::new(0.4)
        }
    }

    fn use_algo(&mut self, type_: Algo) {
        println!("[app] Add task {}", type_.name());

        let maze = self.maze.borrow();
        let algo = type_.create(&*maze);
        self.last_used_algo = Some(type_);
        
        self.exec.tasks.stack(algo);

        let depth_walker = algo::seeding::DijkstraWalk::new(&*maze);
        self.exec.tasks.stack(Box::new(depth_walker));
    }

    fn reset_maze(&mut self) {
        println!("[app] Reset maze");
        self.exec.reset();
        
        let (w, h);
        {
            use grid::Within;

            let maze = self.maze.borrow();
            w = maze.grid().columns();
            h = maze.grid().lines();
        }
        
        self.maze = Rc::new(RefCell::new(OrthoMaze::new(w, h)));
        self.highmap = Rc::new(RefCell::new(OrthoHighMap::new(w, h)));

        if let Some(type_) = self.last_used_algo.clone() {
            self.use_algo(type_);
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let cx = args.width as f64 * 0.5;
        let cy = args.height as f64 * 0.5;

        let maze = self.maze.clone();
        let highmap = self.highmap.clone();
        
        let mut mr = StaticMazeRenderer::new();

        self.gl.draw(args.viewport(), |mut c, gl| {
            clear(color::WHITE, gl);

            c.transform = c.transform.trans(cx, cy);

            mr.render(maze, highmap, &c, gl);
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
            let highmap = self.highmap.clone();
            let args = algo::base::Args { maze, highmap };
            self.exec.tasks.run_step(args);
            self.exec.waited_time %= self.exec.idle_period;
        }
    }

    fn commit_all(&mut self) {
        let maze = self.maze.clone();
        let highmap = self.highmap.clone();
        let args = algo::base::Args { maze, highmap };
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
