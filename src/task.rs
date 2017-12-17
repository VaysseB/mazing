use std::collections::VecDeque;
use std::borrow::Cow;

use super::settings::DEBUG_ALGO;


#[derive(Debug, PartialEq)]
pub enum Status {
    Done,
    Continuing,
    Aborted(String)
}


pub trait Task<T> {
    fn name(&self) -> &'static str;
    fn execute_one(&mut self, args: &mut T) -> Status;

    // Message to use before execution
    fn context<'a>(&'a self) -> Option<Cow<'a, str>> { None }
    
    // Message to use after execution
    fn action<'a>(&'a self) -> Option<Cow<'a, str>> { None }
}


pub struct Executor<T> {
    stack: VecDeque<Box<Task<T>>>
}


impl<T> Executor<T> {
    pub fn new() -> Executor<T> {
        Executor{
            stack: VecDeque::new()
        }
    }

    pub fn stack(&mut self, task: Box<Task<T>>) {
        self.stack.push_back(task);
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }
    
    
    fn do_exec(&mut self, mut args: &mut T) -> Status {
        let maybe_status = self.execute_task(&mut args);
        if let Some(status) = maybe_status {
            match status {
                Status::Done => { self.stack.pop_front(); }
                Status::Aborted(_) => { self.stack.pop_front(); }
                Status::Continuing => ()
            }
            status
        } else {
            Status::Done
        }
    }


    pub fn run(&mut self, mut args: T) {
        'all: while !self.stack.is_empty() {
            if let Status::Aborted(_) = self.do_exec(&mut args) {
                break 'all;
            }
        }
    }

    pub fn run_task(&mut self, mut args: T) {
        while self.do_exec(&mut args) == Status::Continuing {}
    }

    pub fn run_step(&mut self, mut args: T) {
        self.do_exec(&mut args);
    }

    fn execute_task(&mut self, mut args: &mut T) -> Option<Status> {
        if let Some(ref mut task) = self.stack.front_mut() {
            Self::try_log(task, task.context());
            
            let status = task.execute_one(&mut args);
            
            Self::try_log(task, task.action());
            
            match status {
                Status::Done => Self::log(task, "Done"),
                Status::Aborted(ref why) => {
                    let msg = format!("Aborted. {}", why);
                    Self::log(task, &msg);
                }
                Status::Continuing => ()
            }
            
            Some(status)
        } else {
            None
        }
    }

    fn log(task: &Box<Task<T>>, msg: &str) {
        println!("[{}] {}", task.name(), msg);
    }

    fn try_log<'a>(task: &Box<Task<T>>, msg: Option<Cow<'a, str>>) {
        if DEBUG_ALGO {
            if let Some(msg) = msg {
                match msg {
                    Cow::Borrowed(text) => Self::log(task, text),
                    Cow::Owned(text) => Self::log(task, &text),
                }
            }
        }
    }
}


