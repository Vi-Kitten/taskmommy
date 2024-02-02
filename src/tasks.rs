use std::collections::HashMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

pub type TaskID = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub tasks_created: u32,
    pub tasks: HashMap<TaskID, Task>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub time_created: SystemTime,
    pub base: Option<TaskID>, // task to inherit from
    pub tags: Vec<String>,
    pub name: Option<String>,
    pub logs: Vec<(SystemTime, Log)>
}

#[derive(Serialize, Deserialize, Debug)]
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
            time_created: SystemTime::now(),
            base: None,
            tags: vec![],
            name: None,
            logs: vec![]
        }
    }

    pub fn name(&mut self, new_name: String) {
        self.name = Some(new_name.clone());
        self.logs.push((SystemTime::now(), Log::Renamed(new_name)))
    }
}