use graphics::{color, Context, line};
use opengl_graphics::{GlGraphics};

use super::maze::Maze;

pub trait MazeRenderer {
    fn render(&mut self, maze: &Maze, context: &Context, gl: &mut GlGraphics);
}

pub struct StaticMazeRenderer {
    cell_size: f64,
    line_thickness: f64
}

impl StaticMazeRenderer {
    pub fn new() -> StaticMazeRenderer {
        StaticMazeRenderer {
            cell_size: 24.0,
            line_thickness: 1.0
        }
    }

    fn dim(&self, maze: &Maze) -> (f64, f64) {
        let frame_x = maze.columns() as f64 * (self.cell_size + self.line_thickness) + self.line_thickness;
        let frame_y = maze.lines() as f64 * (self.cell_size + self.line_thickness) + self.line_thickness;

        (frame_x, frame_y)
    }

    fn draw_partial_frame_centered(&mut self, maze: &Maze, context: &Context, gl: &mut GlGraphics) {
        let (width, height) = self.dim(maze);

        // top
        line(color::BLACK, self.line_thickness, [
             width * -0.5,
             height * -0.5,
             width *  0.5,
             height * -0.5
        ], context.transform, gl);

        // left
        line(color::BLACK, self.line_thickness, [
             width * -0.5,
             height * -0.5,
             width * -0.5,
             height *  0.5
        ], context.transform, gl);
    }
    
    fn draw_cells_centered(&mut self, maze: &Maze, context: &Context, gl: &mut GlGraphics) {
        let (width, height) = self.dim(maze);
        let corner_x = -0.5 * width + self.line_thickness;
        let corner_y = -0.5 * height + self.line_thickness;

        let space = self.line_thickness + self.cell_size;
        let catchup = space + self.line_thickness;

        for cell in maze.iter() {
            let x = cell.column;
            let y = cell.line;

            let move_down = !cell.can_move_down();
            let move_right = !cell.can_move_right();

            if move_down && move_right {
                continue
            }

            let opp_corner_x = (x + 1) as f64 * space;
            let opp_corner_y = (y + 1) as f64 * space;

            if !move_down {
                line(color::BLACK, self.line_thickness, [
                     corner_x + opp_corner_x - catchup,
                     corner_y + opp_corner_y,
                     corner_x + opp_corner_x,
                     corner_y + opp_corner_y
                ], context.transform, gl);
            }
            
            if !move_right {
                line(color::BLACK, self.line_thickness, [
                     corner_x + opp_corner_x,
                     corner_y + opp_corner_y - catchup,
                     corner_x + opp_corner_x,
                     corner_y + opp_corner_y
                ], context.transform, gl);
            }
        }
    }
}

impl MazeRenderer for StaticMazeRenderer {
    fn render(&mut self, maze: &Maze, context: &Context, gl: &mut GlGraphics) {
        self.draw_partial_frame_centered(maze, context, gl);
        self.draw_cells_centered(maze, context, gl);
    }
}
