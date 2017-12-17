extern crate rand;

use std::borrow::Cow;
use self::rand::Rng;

use super::super::grid::{Within, Address};
use super::super::maze::{OrthoMaze, WithinOrthoMaze};
use super::super::task::{Task, Status};
use algo::base::Args;


impl Address {
    fn carve_right(&self, maze: &mut OrthoMaze) {
        maze.carve(self.column, self.line, self.column + 1, self.line);
    }

    fn carve_down(&self, maze: &mut OrthoMaze) {
        maze.carve(self.column, self.line, self.column, self.line + 1);
    }

    fn carve_to(&self, addr_to: &Address, maze: &mut OrthoMaze) {
        maze.carve(self.column, self.line, addr_to.column, addr_to.line);
    }
}


// -----------------------------------------------------------------------------


pub struct BinaryTree {
    location: Address,
    action: String
}


impl BinaryTree {
    pub fn new(maze: &WithinOrthoMaze) -> BinaryTree {
        let location = maze.grid().crumbs().next().expect("first position exists");
        BinaryTree {
            location,
            action: String::new()
        }
    }

    fn log_action(&mut self, msg: &str) {
        self.action = format!("At {}, {}", self.location.to_str(), msg);
    }
}


impl Task<Args> for BinaryTree {
    fn name(&self) -> &'static str {
        "BinaryTree"
    }

    fn action<'t>(&'t self) -> Option<Cow<'t, str>> {
        Some(Cow::Borrowed(&self.action))
    }

    fn execute_one(&mut self, args: &mut Args) -> Status {
        let mut maze = args.maze.borrow_mut();

        if self.location.is_done_walking_right_then_down(&*maze) {
            return Status::Done;
        }
        else if self.location.is_on_down_border(&*maze) {
            self.log_action("Forced carve right");
            self.location.carve_right(&mut *maze);
        }
        else if self.location.is_on_right_border(&*maze) {
            self.log_action("Forced carve down");
            self.location.carve_down(&mut *maze);
        } else {
            let vert = rand::thread_rng().next_f32() < 0.5;
            if vert {
                self.log_action("Carve down");
                self.location.carve_down(&mut *maze);
            } else {
                self.log_action("Carve right");
                self.location.carve_right(&mut *maze);
            }
        }

        self.location.walk_right_then_down(&mut *maze);
        Status::Continuing
    }
}


// -----------------------------------------------------------------------------


pub struct SideWinder {
    pub location: Address,
    start_x: usize,
    action: String
}


impl SideWinder {
    pub fn new(maze: &WithinOrthoMaze) -> SideWinder {
        let location = maze.grid().crumbs().next().expect("first position exists");
        let start_x = location.column;
        SideWinder { location, action: String::new(), start_x }
    }

    fn log_action(&mut self, msg: &str) {
        self.action = format!("At {}, {}", self.location.to_str(), msg);
    }

    fn close_group(&mut self, maze: &mut OrthoMaze) {
        for column in self.start_x..self.location.column {
            let location = self.location.move_column(column);
            location.unmark_active(maze);
        }

        let door = rand::thread_rng().gen_range(
            self.start_x, self.location.column + 1);

        let location = self.location.move_column(door);

        self.log_action(&format!("Close group, carve down at {}", location.to_str()));

        location.carve_down(maze);
    }

    fn continue_group(&mut self, maze: &mut OrthoMaze) {
        self.location.mark_active(maze);
        self.log_action("Continue group, carve right");
        self.location.carve_right(maze);
    }
}


impl Task<Args> for SideWinder {
    fn name(&self) -> &'static str {
        "SideWinder"
    }

    fn action<'t>(&'t self) -> Option<Cow<'t, str>> {
        Some(Cow::Borrowed(&self.action))
    }

    fn execute_one(&mut self, args: &mut Args) -> Status {
        let mut maze = args.maze.borrow_mut();

        let mut update_start = false;

        if self.location.is_done_walking_right_then_down(&*maze) {
            return Status::Done;
        }
        else if self.location.is_on_right_border(&*maze) {
            self.close_group(&mut *maze);
            update_start = true;
        }
        else if self.location.is_on_down_border(&*maze) {
            self.continue_group(&mut *maze);
        }
        else {
            use self::rand::Rng;

            let build_group = rand::thread_rng().next_f32() < 0.5;
            if build_group {
                self.continue_group(&mut *maze);
            } else {
                self.close_group(&mut *maze);
                update_start = true;
            }
        }

        self.location.walk_right_then_down(&mut *maze);

        if update_start {
            self.start_x = self.location.column;
        }

        if self.location.is_done_walking_right_then_down(&*maze) {
            Status::Done
        } else {
            Status::Continuing
        }
    }
}


// -----------------------------------------------------------------------------


pub struct AldousBroder {
    pub location: Address,
    walk: Vec<Address>,
    restart_walk: bool,
    action: String
}


impl AldousBroder {
    pub fn new(maze: &WithinOrthoMaze) -> AldousBroder {
        let unused_location = maze.grid().crumbs().next().expect("first position exists");
        AldousBroder {
            location: unused_location,
            walk: Vec::new(),
            restart_walk: true,
            action: String::new()
        }
    }

    fn log_action(&mut self, msg: &str) {
        self.action = format!("At {}, {}", self.location.to_str(), msg);
    }

    fn clear_walk(&mut self, maze: &mut OrthoMaze) {
        for addr in self.walk.iter() {
            addr.unmark_active(&mut *maze);
        }
        self.walk.clear();
    }

    fn clear_all(&mut self, maze: &mut OrthoMaze) {
        self.location.unmark_current(maze);
        self.clear_walk(&mut *maze);
        self.clear_visit(&mut *maze);
    }

    fn clear_visit(&self, maze: &mut OrthoMaze) {
        for addr in maze.grid().crumbs() {
            addr.unmark_visit(maze);
        }
    }

    fn walk_to(&mut self, addr: Address, maze: &mut OrthoMaze) {
        self.location.unmark_current(&mut *maze);

        self.location.mark_visit(&mut *maze);

        self.location.mark_active(&mut *maze);
        self.walk.push(self.location.clone());

        addr.mark_current(maze);
        self.location = addr;
    }

    fn pick_next(&mut self, maze: &OrthoMaze) -> Result<Address, &'static str> {
        let pos = self.location.from(&*maze)
            .expect("current position exists in maze");

        let candidates = pos.neighbours();
        let maybe_selected = rand::thread_rng().choose(&candidates);

        match maybe_selected {
            None => Err("impossible situation - no neighbours"),
            Some(value) => Ok(value.into())
        }
    }
}


impl Task<Args> for AldousBroder {
    fn name(&self) -> &'static str {
        "AldousBroder"
    }

    fn action<'t>(&'t self) -> Option<Cow<'t, str>> {
        Some(Cow::Borrowed(&self.action))
    }

    fn execute_one(&mut self, args: &mut Args) -> Status {
        let mut maze = args.maze.borrow_mut();

        if self.restart_walk {
            self.clear_walk(&mut *maze);
            self.restart_walk = false;
        }

        let next_addr = match self.pick_next(&*maze) {
            Err(msg) => {
                self.log_action(msg);
                return Status::Aborted(msg.to_owned());
            }
            Ok(value) => value
        };

        let must_carve = !next_addr.is_visited(&*maze);
        if must_carve {
            self.log_action(&format!("carve to {}", next_addr.to_str()));
            self.location.carve_to(&next_addr, &mut maze);
        } else {
            self.log_action(&format!(
                    "no carving because {} is already visited",
                    next_addr.to_str()));
        }

        self.restart_walk = !must_carve || self.walk.contains(&next_addr);

        self.walk_to(next_addr, &mut *maze);

        if maze.is_visitation_complete() {
            self.log_action("random walk ends, maze is complete");
            self.clear_all(&mut *maze);
            Status::Done
        } else {
            Status::Continuing
        }
    }
}


// -----------------------------------------------------------------------------


pub struct Wilson {
    pub location: Address,
    started: bool,
    walk: Vec<Address>,
    unlocalise_action: String
}


impl Wilson {
    pub fn new(maze: &WithinOrthoMaze) -> Wilson {
        let location = maze.grid().crumbs().next().expect("first position exists");
        Wilson {
            location,
            started: false,
            walk: Vec::new(),
            unlocalise_action: String::new()
        }
    }

    fn log_action(&mut self, msg: &str) {
        self.unlocalise_action = msg.to_owned();
    }

    fn clear_all(&mut self, maze: &mut OrthoMaze) {
        self.location.unmark_current(maze);
        self.clear_visit(&mut *maze);
    }

    fn clear_visit(&self, maze: &mut OrthoMaze) {
        for addr in maze.grid().crumbs() {
            addr.unmark_visit(maze);
        }
    }

    fn walk_to(&mut self, addr: Address, maze: &mut OrthoMaze) {
        self.location.unmark_current(&mut *maze);

        self.location.mark_active(&mut *maze);
        self.walk.push(self.location.clone());

        addr.mark_current(maze);
        self.location = addr;
    }

    fn commit_walk(&mut self, addr: Address, maze: &mut OrthoMaze) {
        self.walk.push(self.location.clone());
        self.walk.push(addr);

        for pair in self.walk.windows(2) {
            let source = &pair[0];
            let dest = &pair[1];
            source.unmark_active(maze);
            source.mark_visit(maze);
            source.carve_to(&dest, maze);
        }

        self.walk.clear();
    }

    fn pick_next(&mut self, maze: &OrthoMaze)
        -> Result<Address, &'static str> {
            let pos = self.location.from(&*maze)
                .expect("current position exists in maze");

            let candidates = pos.neighbours();
            let maybe_selected = rand::thread_rng().choose(&candidates);

            match maybe_selected {
                None => Err("impossible situation - no neighbours"),
                Some(value) => Ok(value.into())
            }
        }

    fn pick_rand_unvisited(&mut self, maze: &OrthoMaze)
        -> Result<Address, &'static str> {
            maze.grid().anywhere_rand_match(|ref pos| !pos.is_visited())
                .map(|pos| pos.into())
                .ok_or("impossible situation - no more unvisited cell")
        }

    fn rewind_to(&mut self, addr: Address, maze: &mut OrthoMaze) {
        let res;
        {
            let walk = &self.walk;
            let mut iter = walk.into_iter();
            res = iter.by_ref()
                .take_while(|ref x| ***x != addr)
                .cloned()
                .collect();

            addr.unmark_active(maze);
            for looped in iter {
                looped.unmark_active(maze);
            }
        }

        self.walk = res;

        self.location.unmark_current(&mut *maze);
        addr.mark_current(&mut *maze);
        self.location = addr;
    }

    fn relocate_rand(&mut self, maze: &mut OrthoMaze) -> Status {
        self.location.unmark_current(&mut *maze);

        self.location = match self.pick_rand_unvisited(&*maze) {
            Err(msg) => {
                self.log_action(msg);
                return Status::Aborted(msg.to_owned());
            }
            Ok(value) => value
        };

        self.location.mark_current(&mut *maze);
        Status::Continuing
    }
}


impl Task<Args> for Wilson {
    fn name(&self) -> &'static str {
        "Wilson"
    }

    fn action<'t>(&'t self) -> Option<Cow<'t, str>> {
        let msg = format!("At {}, {}", self.location.to_str(), self.unlocalise_action);
        Some(Cow::Owned(msg))
    }

    fn execute_one(&mut self, args: &mut Args) -> Status {
        let mut maze = args.maze.borrow_mut();

        if !self.started {
            self.started = true;
            self.location.mark_visit(&mut *maze);
            self.log_action("initialised");
            return self.relocate_rand(&mut *maze);
        }

        let next_addr = match self.pick_next(&*maze) {
            Err(msg) => {
                self.log_action(msg);
                return Status::Aborted(msg.to_owned());
            }
            Ok(value) => value
        };

        let next_is_visited = next_addr.is_visited(&*maze);
        if next_is_visited {
            self.commit_walk(next_addr, &mut *maze);

            if maze.is_visitation_complete() {
                self.log_action("current walk ended, maze is complete");
                self.clear_all(&mut *maze);
                Status::Done
            } else {
                self.log_action("current walk ended, carve it, and select new unvisited cell");
                self.relocate_rand(&mut *maze)
            }
        }
        else {
            let is_in_current_walk = self.walk.contains(&next_addr);

            if is_in_current_walk {
                self.log_action(
                    &format!("loop detected at {}, rewind",
                             next_addr.to_str()));
                self.rewind_to(next_addr, &mut *maze)
            }
            else {
                self.log_action("walk continue");
                self.walk_to(next_addr, &mut *maze)
            }

            Status::Continuing
        }
    }
}
