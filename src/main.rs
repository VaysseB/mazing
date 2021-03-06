#[macro_use]
extern crate lazy_static;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::{keyboard, RenderEvent, UpdateEvent, PressEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };


pub mod app;
pub mod settings;
pub mod task;
pub mod grid;
pub mod maze;
pub mod maze_render;
pub mod highmap;
pub mod algo;


use settings::{DEBUG_GATE, DEBUG_ALGO, DEBUG_MAIN};


fn main() {
    let opengl = OpenGL::V3_2;

    let win_size;
    
    if DEBUG_GATE || DEBUG_ALGO {
        win_size = [500, 450];
    } else {
        win_size = [700, 550];
    }

    let win_settings = WindowSettings::new("mazing", win_size)
        .opengl(opengl)
        .srgb(false)
        .decorated(true)
        .exit_on_esc(true);

    let mut window: GlutinWindow = win_settings.build()
        .expect("fail to build window");

    let mut app = app::App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    let mut modkeys = keyboard::ModifierKey::NO_MODIFIER;
    while let Some(e) = events.next(&mut window) {
        if DEBUG_MAIN {
            println!("[main] start-loop, pre-render");
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if DEBUG_MAIN {
            println!("[main] post-render, pre-update");
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        modkeys.event(&e);

        if DEBUG_MAIN {
            println!("[main] post-update, pre-button-pressed");
        }
        if let Some(b) = e.press_args() {
            app.button_pressed(&b, &modkeys);
        }
        
        if DEBUG_MAIN {
            println!("[main] post-button-pressed, end-loop");
        }
    }
}
