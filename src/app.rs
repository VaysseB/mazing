extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use std::rc::Rc;
use std::cell::RefCell;

use piston::input::{keyboard, RenderArgs, UpdateArgs, Button, Key};
use opengl_graphics::{GlGraphics};

use super::settings::{DEBUG_GATE, DEBUG_ALGO};
use super::maze::{OrthoMaze, WithinOrthoMaze};
use super::maze_render::{MazeRenderer, StaticMazeRenderer};
use super::highmap::OrthoHighMap;
use super::algo;
use super::task;


#[derive(Clone)]
enum Algo {
    BinaryTree,
    SideWinder
}


impl Algo {
    fn name(&self) -> &'static str {
        match *self {
            Algo::BinaryTree => "BinaryTree",
            Algo::SideWinder => "SideWinder"
        }
    }

    fn create(&self, maze: &WithinOrthoMaze)
        -> Box<task::Task<algo::base::Args>> {
            match *self {
                Algo::BinaryTree => Box::new(algo::carving::BinaryTree::new(maze)),
                Algo::SideWinder => Box::new(algo::carving::SideWinder::new(maze))
            }
        }
}


// ----------------------------------------------------------------------------


type Second = f64;


enum Speed {
    VerySlow,
    Normal
}


impl Speed {
    fn period(&self) -> Second {
        match *self {
            Speed::VerySlow => 0.4,
            Speed::Normal => 0.01
        }
    }
    
    fn batch(&self) -> usize {
        match *self {
            Speed::VerySlow => 1,
            Speed::Normal => 5
        }
    }
}


// ----------------------------------------------------------------------------


struct Execution {
    tasks: task::Executor<algo::base::Args>,
    active: bool,
    speed: Speed,
    waited_time: Second
}


impl Execution {
    fn new(speed: Speed) -> Execution {
        Execution {
            tasks: task::Executor::new(),
            active: false,
            speed,
            waited_time: 0.0
        }
    }

    fn reset(&mut self) {
        self.tasks.clear();
        self.active = false;
        self.waited_time = 0.0;
    }

    fn change_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }
}


// ----------------------------------------------------------------------------


pub struct App {
    gl: GlGraphics,
    mr: StaticMazeRenderer,
    last_carve_algo: Option<Algo>,
    next_carve_algo: Option<Algo>,
    maze: Rc<RefCell<OrthoMaze>>,
    highmap: Rc<RefCell<OrthoHighMap>>,
    exec: Execution
}


impl App {
    pub fn new(gl: GlGraphics) -> App {
        let (w, h) = if DEBUG_GATE || DEBUG_ALGO { (6, 4) } else { (60, 40) };
        let speed = if DEBUG_ALGO { Speed::VerySlow} else { Speed::Normal };

        let maze = Rc::new(RefCell::new(OrthoMaze::new(w, h)));
        let highmap = Rc::new(RefCell::new(OrthoHighMap::new(w, h)));
        App {
            gl,
            mr: StaticMazeRenderer::new(),
            maze,
            highmap,
            last_carve_algo: None,
            next_carve_algo: None,
            exec: Execution::new(speed)
        }
    }

    fn reset_algo(&mut self, type_: Algo) {
        println!("[app] Reset algo with {}", type_.name());

        self.exec.reset();

        let maze = self.maze.borrow();
        let algo = type_.create(&*maze);

        self.last_carve_algo = Some(type_);
        self.exec.tasks.stack(algo);

        let depth_walker = algo::seeding::DijkstraWalk::new(&*maze);
        self.exec.tasks.stack(Box::new(depth_walker));
    }

    fn select_algo(&mut self, type_: Algo) {
        println!("[app] Next algo is {}", type_.name());
        if self.last_carve_algo.is_none() {
            self.reset_algo(type_.clone());
        }

        self.next_carve_algo = Some(type_);
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

        let algo = self.next_carve_algo.clone().or(self.last_carve_algo.clone());
        if let Some(type_) = algo {
            self.reset_algo(type_);
        }

        self.next_carve_algo = self.last_carve_algo.clone();
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let cx = args.width as f64 * 0.5;
        let cy = args.height as f64 * 0.5;

        let maze = self.maze.clone();
        let highmap = self.highmap.clone();

        let gl = &mut self.gl;
        let mr = &mut self.mr;

        gl.draw(args.viewport(), |mut c, gl| {
            clear(color::WHITE, gl);

            c.transform = c.transform.trans(cx, cy);

            mr.render(maze, highmap, &c, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.exec.active {
            self.commit_one_step(args.dt);
        }
    }

    fn commit_one_step(&mut self, dt: Second) {
        self.exec.waited_time += dt;

        if self.exec.waited_time >= self.exec.speed.period() {
            for _ in 0..self.exec.speed.batch() {
                let maze = self.maze.clone();
                let highmap = self.highmap.clone();
                let args = algo::base::Args { maze, highmap };
                self.exec.tasks.run_step(args);
            }
    
            self.exec.waited_time %= self.exec.speed.period();
        }
    }

    fn commit_one_task(&mut self) {
        let maze = self.maze.clone();
        let highmap = self.highmap.clone();
        let args = algo::base::Args { maze, highmap };
        self.exec.tasks.run_task(args);
    }

    fn commit_all(&mut self) {
        let maze = self.maze.clone();
        let highmap = self.highmap.clone();
        let args = algo::base::Args { maze, highmap };
        self.exec.tasks.run(args);
    }

    pub fn button_pressed(
        &mut self,
        args: &Button,
        modkeys: &keyboard::ModifierKey)
    {
        match *args {
            Button::Keyboard(key) if key == Key::Space => {
                self.exec.active = !self.exec.active;

                if self.exec.active {
                    println!("[app] Resume algo");
                    self.exec.waited_time = self.exec.speed.period();
                } else {
                    println!("[app] Pause execution");
                    self.exec.waited_time = 0.0;
                }
            },
            Button::Keyboard(key) if key == Key::Return => {
                let scoped_exec = modkeys.contains(keyboard::ModifierKey::CTRL);
                if scoped_exec { self.commit_one_task(); }
                else { self.commit_all(); }
            },
            Button::Keyboard(key) if key == Key::D1 => {
                self.select_algo(Algo::BinaryTree);
            },
            Button::Keyboard(key) if key == Key::D2 => {
                self.select_algo(Algo::SideWinder);
            },
            Button::Keyboard(key) if key == Key::G => {
                self.mr.toggle_gate();
            },
            Button::Keyboard(key) if key == Key::H => {
                self.mr.toggle_highmap();
            },
            Button::Keyboard(key) if key == Key::Backspace => {
                self.reset_maze();
            },
            Button::Keyboard(key) if key == Key::PageUp => {
                self.exec.change_speed(Speed::VerySlow);
            },
            Button::Keyboard(key) if key == Key::PageDown => {
                self.exec.change_speed(Speed::Normal);
            },
            _ => ()
        }
    }
}
