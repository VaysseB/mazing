use std::collections::VecDeque;

use super::settings::DEBUG_ALGO;


#[derive(Debug, PartialEq)]
pub enum Status {
    Done,
    Continuing
}


pub trait Task<T> {
    fn name(&self) -> &'static str;
    fn execute_one(&mut self, args: &mut T) -> Status;

    // Message to use before execution
    fn context(&self) -> Option<&String> { None }
    
    // Message to use after execution
    fn action(&self) -> Option<&String> { None }
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

    pub fn run(&mut self, mut args: T) {
        while !self.stack.is_empty() {
            let maybe_status = self.execute_task(&mut args);
            if maybe_status == Some(Status::Done) {
                self.stack.pop_front();
            }
        }
    }

    pub fn run_step(&mut self, mut args: T) {
        let maybe_status = self.execute_task(&mut args);
        if maybe_status == Some(Status::Done) {
            self.stack.pop_front();
        }
    }

    fn execute_task(&mut self, mut args: &mut T) -> Option<Status> {
        if let Some(ref mut task) = self.stack.front_mut() {
            Self::try_log(task, task.context());
            
            let status = task.execute_one(&mut args);
            
            Self::try_log(task, task.action());
            
            if status == Status::Done {
                Self::log(task, "Done");
            }
            
            Some(status)
        } else {
            None
        }
    }

    fn log(task: &Box<Task<T>>, msg: &str) {
        println!("[{}] {}", task.name(), msg);
    }

    fn try_log(task: &Box<Task<T>>, msg: Option<&String>) {
        if DEBUG_ALGO {
            if let Some(msg) = msg {
                Self::log(task, msg);
            }
        }
    }
}


