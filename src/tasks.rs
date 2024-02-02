use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub type TaskID = String;

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub tasks_created: u32,
    pub tasks: HashMap<TaskID, Task>
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub base: Option<TaskID>, // task to inherit from
    pub tags: Vec<String>,
    pub name: Option<String>,
    pub logs: Vec<Log>
}

#[derive(Serialize, Deserialize)]
pub enum Log {
    Completed,
    MarkedTemplate,
    ReOpened,
    Renamed(String),
    TagAdded(String),
    TagRemoved(String),
    Info(TaskID)
}

impl Data {
    pub fn new() -> Data {
        Data {
            tasks_created: 0,
            tasks: HashMap::new()
        }

    }
}

impl Task {
    pub fn new() -> Task {
        Task {
            base: None,
            tags: vec![],
            name: None,
            logs: vec![]
        }
    }

    pub fn name(&mut self, new_name: String) {
        self.name = Some(new_name.clone());
        self.logs.push(Log::Renamed(new_name))
    }
}