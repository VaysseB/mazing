use grid::Way;
use maze::OrthoMaze;


pub enum SymbolSet {
    LightWeight,
    Dashed
}


// -----------------------------------------------------------------------------


struct LineBuilderFactory {
    opened_hori_gate: String,
    closed_hori_gate: String,
    opened_vert_gate: String,
    closed_vert_gate: String,
    join: String,
    vert: String
}


impl LineBuilderFactory {
    fn new(
        hori_close: char,
        hori_open: char,
        vert_close: char,
        vert_open: char,
        join: char,
        space: char,
          ) -> LineBuilderFactory {
        LineBuilderFactory {
            opened_hori_gate: format!("{}{}{}", join, hori_open, hori_open),
            closed_hori_gate: format!("{}{}{}", join, hori_close, hori_close),
            opened_vert_gate: format!("{}{}{}", vert_open, space, space),
            closed_vert_gate: format!("{}{}{}", vert_close, space, space),
            join: format!("{}", join),
            vert: format!("{}", vert_close),
        }
    }

    
    fn horizontal(&self, length: usize) -> LineBuilder {
        LineBuilder::new(length, 
                         self.opened_hori_gate.clone(), 
                         self.closed_hori_gate.clone(), 
                         self.join.clone())
    }

    
    fn vertical(&self, length: usize) -> LineBuilder {
        LineBuilder::new(length, 
                         self.opened_vert_gate.clone(), 
                         self.closed_vert_gate.clone(), 
                         self.vert.clone())
    }


    fn horizontal_border(&self, length: usize) -> String {
        let line = self.closed_hori_gate.repeat(length);
        format!("{}{}", line, self.join)
    }
}


// ----------------------------------------------------------------------------


struct LineBuilder {
    line: String,
    nb_cells: usize,
    start_index: usize,
    gate_opened_last_time: Option<bool>,
    opened_gate: String,
    closed_gate: String,
    end_gate: String
}


impl LineBuilder {
    fn new(
        count: usize,
        opened_gate: String,
        closed_gate: String,
        end_gate: String
        ) -> LineBuilder {
        let len_gate = opened_gate.len();
        let len_end_gate = end_gate.len();
        LineBuilder {
            line: String::with_capacity(count * len_gate + len_end_gate),
            nb_cells: count,
            start_index: 0,
            gate_opened_last_time: None,
            opened_gate,
            closed_gate,
            end_gate
        }
    }


    fn update(&mut self, index: usize, gate_is_opened: bool) {
        if self.gate_opened_last_time.is_none() {
            self.start_index = index;
            self.gate_opened_last_time = Some(gate_is_opened);
        } else if self.gate_opened_last_time != Some(gate_is_opened) {
            let pattern =
                if self.gate_opened_last_time.unwrap() { &self.opened_gate }
                else { &self.closed_gate };

            let times = index - self.start_index;
            for _ in 0..times {
                self.line.push_str(pattern);
            }

            self.start_index = index;
            self.gate_opened_last_time = Some(gate_is_opened);
        }
    }


    fn done(mut self) -> String {
        let pattern =
            if self.gate_opened_last_time.unwrap() { &self.opened_gate }
            else { &self.closed_gate };

        let times = self.nb_cells - self.start_index;
        for _ in 0..times {
            self.line.push_str(pattern);
        }

        self.line.push_str(&self.end_gate);
        self.line
    }
}


// ----------------------------------------------------------------------------


pub struct PlainAscii {
    symbol_set: SymbolSet
}


impl PlainAscii {
    pub fn new(symbol_set: SymbolSet) -> PlainAscii {
        PlainAscii {
            symbol_set
        }
    }


    fn factory(&self) -> LineBuilderFactory {
        match self.symbol_set {
            SymbolSet::LightWeight => LineBuilderFactory::new('-', ' ', '|', ' ', '+', ' '),
            SymbolSet::Dashed => LineBuilderFactory::new('=', '-', '‖', '¦', '#', ' '),
        }
    }


    pub fn draw(&self, maze: &OrthoMaze) -> Result<Vec<String>, ()> {
        let (nb_columns, nb_lines) = {
            let grid_guard = maze.grid.lock()
                .expect("nobody panics holding mutex");
            grid_guard.dim()
        };

        let locgen = maze.loc_generator();
        let mut result = Vec::new();
        
        let line_builder = self.factory();
        result.push(line_builder.horizontal_border(nb_columns));

        for y in 0..nb_lines {
            let mut hori_line = line_builder.horizontal(nb_columns);
            let mut vert_line = line_builder.vertical(nb_columns);

            for x in 0..nb_columns {
                let loc = locgen.create_from_coordinates(x, y);
                let gates = maze.gates_at(&loc);

                hori_line.update(x, gates.can_move(&Way::Down));
                vert_line.update(x, gates.can_move(&Way::Left));
            }

            result.push(vert_line.done());
            if y + 1 < nb_lines { result.push(hori_line.done()); }
            else { result.push(line_builder.horizontal_border(nb_columns)) }
        }

        Ok(result)
    }
}

