extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderEvent, UpdateEvent, PressEvent};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


pub mod app;
pub mod task;
pub mod grid;
pub mod maze;
pub mod maze_render;
pub mod depth;
pub mod algo;


fn main() {
    let opengl = OpenGL::V3_2;

    let win_size = [500, 450];
    let win_settings = WindowSettings::new("mazing", win_size)
        .opengl(opengl)
        .srgb(false)
        .exit_on_esc(true);

    let mut window: Window = win_settings.build()
        .expect("fail to build window");

    let mut app = app::App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(b) = e.press_args() {
            app.button_pressed(&b);
        }
    }
}
