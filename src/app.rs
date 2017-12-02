extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::input::{RenderArgs, UpdateArgs};
use opengl_graphics::{ GlGraphics };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64   // Rotation for the square.
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App { gl, rotation: 0.0 }
    }


    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(color::WHITE, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(color::BLACK, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}
