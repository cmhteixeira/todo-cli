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

    pub fn print_tty(&self) -> String {
        let mut res = String::new();
        res.push_str("\u{001b}[1;31mActive\u{001b}[0m \u{23F3}\n");
        for i in &self.active {
            res.push_str(format!("{}{}", i.description, "\n").as_str());
        }
        res.push_str("\n");
        res.push_str("\n");

        res.push_str("\u{001b}[1;31mCompleted\u{001b}[0m \u{2705}\n");
        for i in &self.completed {
            res.push_str(format!("{}{}", i.description, "\n").as_str());
        }

        res
    }
}