//use super::super::depth::OrthoDepthMap;
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
        let mut depth_map = args.depth_map.borrow_mut();

        for (i, pos) in depth_map.grid().crumbs().enumerate() {
            let pos = pos.from_mut(&mut *depth_map);
            pos.expect("position exists").set_depth(i);
        }

        Status::Done
    }
}
