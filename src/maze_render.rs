use graphics::{Context};
use opengl_graphics::{GlGraphics};

use super::maze::Maze;

pub trait MazeRenderer {
    fn render(&mut self, maze: &Maze, content: &Context, gl: &mut GlGraphics);
}

pub struct StaticMazeRenderer {}

impl StaticMazeRenderer {
    pub fn new() -> StaticMazeRenderer {
        StaticMazeRenderer {}
    }
}

impl MazeRenderer for StaticMazeRenderer {
    fn render(&mut self, maze: &Maze, context: &Context, gl: &mut GlGraphics) {
        use graphics::*;
        
        const CELL_W : f64 = 12.0f64;
        const LINE_S : f64 = 1.0f64;
        let frame_x = maze.width as f64 * (CELL_W + LINE_S) + LINE_S;
        let frame_y = maze.height as f64 * (CELL_W + LINE_S) + LINE_S;

        // top
        line(color::BLACK, LINE_S, [
             frame_x * -0.5,
             frame_y * -0.5,
             frame_x *  0.5,
             frame_y * -0.5
        ], context.transform, gl);

        // bottom
        line(color::BLACK, LINE_S, [
             frame_x * -0.5,
             frame_y *  0.5,
             frame_x *  0.5,
             frame_y *  0.5
        ], context.transform, gl);

        // left
        line(color::BLACK, LINE_S, [
             frame_x * -0.5,
             frame_y * -0.5,
             frame_x * -0.5,
             frame_y *  0.5
        ], context.transform, gl);

        // right
        line(color::BLACK, LINE_S, [
             frame_x *  0.5,
             frame_y * -0.5,
             frame_x *  0.5,
             frame_y *  0.5
        ], context.transform, gl);
    }
}
