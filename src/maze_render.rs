use std::rc::Rc;
use std::cell::RefCell;

use graphics::{color, Context, line, rectangle};
use graphics::types::{Color, ColorComponent};
use opengl_graphics::{GlGraphics};

use super::settings::{DEBUG_GATE, DEBUG_ALGO};
use super::grid::{Pos, Within};
use super::maze;
use super::highmap;
use super::maze::OrthoMaze;
use super::highmap::OrthoHighMap;


lazy_static! {
    static ref ROOT_COLORS : [Color; 5] = [
        color::hex("1B5E20"), // green
        color::hex("0D47A1"), // blue
        color::hex("E65100"), // orange
        color::hex("b71c1c"), // red
        color::hex("004D40") // turquoise
    ];
}


const DIST_PER_COLOR : usize = 80;


// ----------------------------------------------------------------------------


pub trait MazeRenderer {
    fn render(
        &mut self,
        maze: Rc<RefCell<OrthoMaze>>,
        highmap: Rc<RefCell<OrthoHighMap>>,
        context: &Context,
        gl: &mut GlGraphics);

    fn toggle_gate(&mut self);
    
    fn toggle_highmap(&mut self);

    fn distance_per_color(&mut self) -> usize;
    fn set_distance_per_color(&mut self, dist: usize);
}


// ----------------------------------------------------------------------------


pub struct StaticMazeRenderer {
    cell_size: f64,
    line_thickness: f64,
    hori_line: Color,
    vert_line: Color,
    visible_gates: bool,
    visible_highmap: bool,
    dist_per_color: usize
}


impl StaticMazeRenderer {
    pub fn new() -> StaticMazeRenderer {
        let (hori_line, vert_line);
        if DEBUG_GATE {
            hori_line = color::hex("FF0000A0");
            vert_line = color::hex("00FF00A0");
        } else {
            hori_line = color::BLACK;
            vert_line = color::BLACK;
        }

        let (cell_size, line_thickness);
        if DEBUG_ALGO {
            cell_size = 70.0;
            line_thickness = 8.0;
        } else {
            cell_size = 10.0;
            line_thickness = 1.0;
        }
        
        StaticMazeRenderer {
            cell_size,
            line_thickness,
            hori_line,
            vert_line,
            visible_gates: true,
            visible_highmap: true,
            dist_per_color: DIST_PER_COLOR
        }
    }

    fn current_status_color<'a>(
        &'a self,
        pos: Pos<'a, maze::CellStatus>)
        -> Option<Color> {
            if pos.is_current() {
                Some(color::hex("FF5722"))
            } else if pos.is_active() {
                Some(color::hex("FFC107"))
            } else if pos.is_visited() {
                Some(color::hex("9E9E9E"))
            } else {
                None
            }
        }

    
    fn height_color<'a>(
        &'a self,
        pos: Pos<'a, highmap::CellStatus>,
        _highest: usize)
        -> Option<Color> {
            pos.height().map(|v| self.altitude_color(v))
        }

    
    fn altitude_color(&self, altitude: usize) -> Color {
        use graphics::Colored;
        
        let color_index = (altitude / self.dist_per_color) % ROOT_COLORS.len();

        let size = self.dist_per_color as f64;
        let advance = (altitude as f64 % size) / size;
        
        let tint = 0.4 + 1.2 * advance;
        ROOT_COLORS[color_index].tint(tint as ColorComponent)
    }


    fn frame_box(&self, maze: Rc<RefCell<OrthoMaze>>)
        -> (f64, f64, f64, f64) {
            let maze = maze.borrow();

            let space = self.cell_size + self.line_thickness;

            let width = maze.grid().columns() as f64 * space + self.line_thickness;
            let height = maze.grid().lines() as f64 * space + self.line_thickness;

            let origin_x = (width - self.line_thickness) * 0.5;
            let origin_y = (height - self.line_thickness) * 0.5;

            (-origin_x, -origin_y, width, height)
        }


    fn draw_partial_frame_centered(
        &mut self,
        maze: Rc<RefCell<OrthoMaze>>,
        context: &Context,
        gl: &mut GlGraphics)
    {
        let (origin_x, origin_y, width, height) = self.frame_box(maze);
        let hlt = self.line_thickness * 0.5;

        // top
        line(self.hori_line, hlt, [
             origin_x - hlt,
             origin_y,
             origin_x + width - hlt,
             origin_y
        ], context.transform, gl);

        // left
        line(self.vert_line, hlt, [
             origin_x,
             origin_y - hlt,
             origin_x,
             origin_y + height - hlt
        ], context.transform, gl);

        if !self.visible_gates {
            // bottom
            line(self.hori_line, hlt, [
                 origin_x - hlt,
                 origin_y + height - self.line_thickness,
                 origin_x + width - hlt,
                 origin_y + height - self.line_thickness
            ], context.transform, gl);

            // right
            line(self.vert_line, hlt, [
                 origin_x + width - self.line_thickness,
                 origin_y - hlt,
                 origin_x + width - self.line_thickness,
                 origin_y + height - hlt
            ], context.transform, gl);
        }
    }


    fn draw_gates_centered(
        &mut self,
        maze: Rc<RefCell<OrthoMaze>>,
        context: &Context,
        gl: &mut GlGraphics)
    {
        use grid::Within;

        let (origin_x, origin_y, _, _) = self.frame_box(maze.clone());

        let hlt = self.line_thickness * 0.5;
        let space = self.line_thickness + self.cell_size;

        let maze = maze.borrow();
        for pos in maze.grid().iter() {
            let x = pos.column;
            let y = pos.line;

            let corner_x = origin_x + x as f64 * space;
            let corner_y = origin_y + y as f64 * space;

            if !pos.can_move_down() {
                line(self.hori_line, hlt, [
                     corner_x - hlt,
                     corner_y + space,
                     corner_x + space + hlt,
                     corner_y + space
                ], context.transform, gl);
            }

            if !pos.can_move_right() {
                line(self.vert_line, hlt, [
                     corner_x + space,
                     corner_y - hlt,
                     corner_x + space,
                     corner_y + space + hlt
                ], context.transform, gl);
            }
        }
    }

    fn draw_cells_centered(
        &mut self,
        maze: Rc<RefCell<OrthoMaze>>,
        highmap: Rc<RefCell<OrthoHighMap>>,
        context: &Context,
        gl: &mut GlGraphics)
    {
        let (origin_x, origin_y, _, _) = self.frame_box(maze.clone());

        let hlt = self.line_thickness * 0.5;
        let space = self.line_thickness + self.cell_size;

        let maze = maze.borrow();
        let highmap = highmap.borrow();
        let highest = highmap.highest;

        for address in maze.grid().crumbs() {
            let pos = address.from(&*maze).expect("position of maze exists");
            let hpos = address.from(&*highmap).expect("position of highmap exists");

            let x = address.column;
            let y = address.line;

            let corner_x = origin_x + x as f64 * space;
            let corner_y = origin_y + y as f64 * space;

            let status_color = self.current_status_color(pos);
            let color = if self.visible_highmap {
                status_color.or_else(|| self.height_color(hpos, highest))
            } else {
                status_color
            };

            if let Some(color) = color {
                rectangle(color, [
                          corner_x - hlt,
                          corner_y - hlt,
                          space,
                          space
                ], context.transform, gl);
            }
        }
    }
}


impl MazeRenderer for StaticMazeRenderer {
    fn render(
        &mut self,
        maze: Rc<RefCell<OrthoMaze>>,
        highmap: Rc<RefCell<OrthoHighMap>>,
        context: &Context,
        gl: &mut GlGraphics)
    {
        self.draw_cells_centered(maze.clone(), highmap.clone(), context, gl);
        
        self.draw_partial_frame_centered(maze.clone(), context, gl);

        if self.visible_gates {
            self.draw_gates_centered(maze.clone(), context, gl);
        }
    }

    fn toggle_gate(&mut self) {
        self.visible_gates = !self.visible_gates;
    }
    
    fn toggle_highmap(&mut self) {
        self.visible_highmap = !self.visible_highmap;
    }
    
    fn distance_per_color(&mut self) -> usize {
        self.dist_per_color
    }
    
    fn set_distance_per_color(&mut self, dist: usize) {
        self.dist_per_color = dist;
    }
}
