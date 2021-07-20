use std::time::Instant;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum Importance {
    One,
    Two,
    Three,
    Four,
    Five
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task<'a> {
    id: u32,
    #[serde(borrow)]
    description: &'a str,
    importance: Option<Importance>,
    time_stamp: u128
}

impl Task<'_> {
    pub fn new(id: u32, description: &str, importance: Option<Importance>) -> Task {
        Task {
            id,
            description,
            importance,
            time_stamp: std::time::Instant::now().elapsed().as_millis(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DataPersisted<'b> {
    #[serde(borrow)]
    active: Vec<Task<'b>>,
    #[serde(borrow)]
    completed: Vec<Task<'b>>
}

impl<'a> DataPersisted<'a> {
    pub fn empty() -> DataPersisted<'static> {
        DataPersisted {
            active: vec![],
            completed: vec![]
        }
    }

    pub fn add_active<'b: 'a>(&mut self, task: Task<'b>) -> () {
        self.active.append(&mut vec![task])
    }
}