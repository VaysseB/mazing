bitflags! {
    struct GateWay: u32 {
        const HORI = 0b001;
        const VERT = 0b010;
    }
}


pub struct Maze {
    pub width: u32,
    pub height: u32,
    gates: Vec<GateWay>
}

impl Maze {
    pub fn new(width: u32, height: u32) -> Maze {
        let count = (width * height) as usize;
        let mut gates = Vec::with_capacity(count);
        for _ in 0..(count) {
            gates.push(GateWay{ bits: 0 });
        }
        Maze{ width, height, gates }
    }

    pub fn toggle(&mut self) {
        for gate in self.gates.iter_mut() {
            *gate = !*gate;
        }
    }
}
