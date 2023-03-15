use std::{cmp::Ordering, rc::Rc};

use yew::Reducible;

use crate::models::Task;

pub enum TaskAction {
    Set(Vec<Task>),
    Add(Task),
    Delete(String),
    Toggle(String),
}

pub struct TaskState {
    pub tasks: Vec<Task>,
}

impl Default for TaskState {
    fn default() -> Self {
        Self { tasks: vec![] }
    }
}
