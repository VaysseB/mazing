//use super::super::depth::OrthoHighMap;
use super::super::task::{Task, Status};
use algo::base::{Args};
use super::super::grid::Within;


pub struct DjisktraWalk {
    action: String
}


impl DjisktraWalk {
    pub fn new() -> DjisktraWalk {
        DjisktraWalk { action: String::new() }
    }
    
    fn log_action(&mut self, _msg: &str) {
        //self.action = format!("At {}, {}", self.pos.to_str(), msg);
    }
}


impl Task<Args> for DjisktraWalk {
    fn name(&self) -> &'static str {
        "DjisktraWalk"
    }

    fn action<'t>(&'t self) -> Option<&'t String> {
        Some(&self.action)
    }

    fn execute_one(&mut self, args: &mut Args) -> Status {
        let mut highmap = args.highmap.borrow_mut();

        let mut max = 0;
        for (i, pos) in highmap.grid().crumbs().enumerate() {
            let pos = pos.from_mut(&mut *highmap);
            pos.expect("position exists").set_depth(i);
            max = i;
        }

        highmap.highest = max;

        Status::Done
    }
}
