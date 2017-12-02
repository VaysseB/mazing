extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::input::{RenderArgs, UpdateArgs, Button, Key};
use opengl_graphics::{ GlGraphics };

use super::maze::Maze;
use super::maze_render::{MazeRenderer, StaticMazeRenderer};

pub struct App {
    gl: GlGraphics,
    maze: Maze
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App { gl, maze: Maze::new(6, 4) }
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

    pub fn update(&mut self, _args: &UpdateArgs) {
    }

    pub fn button_pressed(&mut self, args: &Button) {
        //match *args {
            //Button::Keyboard(key) if key == Key::Up => {
                //self.pos[1] -= 1;
            //},
            //Button::Keyboard(key) if key == Key::Down => {
                //self.pos[1] += 1;
            //},
            //_ => ()
        //}
    }
}
