use std;
use std::collections::VecDeque;
use super::super::grid::Address;
use super::super::maze::WithinOrthoMaze;
use super::super::task::{Task, Status};
use algo::base::Args;


pub struct DijkstraWalk {
    in_sight: VecDeque<Address>,
    action: String
}


impl DijkstraWalk {
    pub fn new(maze: &WithinOrthoMaze) -> DijkstraWalk {
        let start = maze.grid().crumbs().next().expect("first position exists");
        let mut in_sight = VecDeque::new();
        in_sight.push_back(start);
        DijkstraWalk {
            in_sight,
            action: String::new()
        }
    }

    fn log_action(&mut self, current: &Address, msg: String) {
        self.action = format!("At {}, {}", current.to_str(), msg);
    }
}


impl Task<Args> for DijkstraWalk {
    fn name(&self) -> &'static str {
        "DijkstraWalk"
    }

    fn action<'t>(&'t self) -> Option<&'t String> {
        Some(&self.action)
    }

    fn execute_one(&mut self, args: &mut Args) -> Status {
        let maze = args.maze.borrow();
        let mut highmap = args.highmap.borrow_mut();

        if let Some(ref address) = self.in_sight.pop_front() {
            let pos = address.from(&*maze).expect("address in maze exists");
            let neighbours = pos.reachable_neighbours();

            let mut surroudings = Vec::with_capacity(4);
            for neighbour in neighbours {
                let haddress : Address = neighbour.into();
                let hpos = haddress.from(&*highmap)
                    .expect("address in highmap exists");
                

                if let Some(height) = hpos.height() {
                    surroudings.push(height);
                } else if !self.in_sight.contains(&haddress) {
                    self.in_sight.push_back(haddress);
                }
            }

            let height = surroudings.iter().min()
                .map(|x| x + 1)
                .unwrap_or(0);

            self.log_action(&address, format!("distance is {}", height));

            {
                let mut hpos = address.from_mut(&mut *highmap)
                    .expect("position in highmap exists");
                hpos.set_depth(height);
            }

            highmap.highest = std::cmp::max(height, highmap.highest);
        }

        if self.in_sight.is_empty() { Status::Done }
        else { Status::Continuing }
    }
}
