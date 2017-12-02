extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::input::{RenderArgs, UpdateArgs, Button, Key};
use opengl_graphics::{ GlGraphics };

pub struct App {
    gl: GlGraphics,
    pos: [i32; 2]
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App { gl, pos: [0, 0] }
    }


    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const ML : f64 = 4.0;

        let square = rectangle::square(0.0, 0.0, ML * 2f64);
        let (x, y) = (
            (args.width / 2) as f64,
            (args.height / 2) as f64);
        let (dx, dy) = (
            ML * self.pos[0] as f64,
            ML * self.pos[1] as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(color::WHITE, gl);

            let transform = c.transform
                .trans(x, y)
                .trans(-ML, -ML)
                .trans(dx, dy);

            rectangle(color::BLACK, square, transform, gl);
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
    }

    pub fn button_pressed(&mut self, args: &Button) {
        match *args {
            Button::Keyboard(key) if key == Key::Left => {
                self.pos[0] -= 1;
            },
            Button::Keyboard(key) if key == Key::Right => {
                self.pos[0] += 1;
            },
            Button::Keyboard(key) if key == Key::Up => {
                self.pos[1] -= 1;
            },
            Button::Keyboard(key) if key == Key::Down => {
                self.pos[1] += 1;
            },
            _ => ()
        }
    }
}
