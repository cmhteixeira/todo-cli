use clap::ArgMatches;
use crate::log::Action::Complete;
use std::num::ParseIntError;

pub struct AddTask<'a> {
    pub task_name: &'a str
}

impl<'a> AddTask<'a> {
    fn new(name: &'a str) -> AddTask {
        AddTask {
            task_name: name
        }
    }
}

pub struct CompleteTask {
    pub task_id: u8
}

impl CompleteTask {
    fn new(id: u8) -> CompleteTask {
        CompleteTask {
            task_id: id
        }
    }
}

pub struct DeleteTasks {
    pub task_ids: Vec<u8>
}

impl DeleteTasks {
    fn new(id: u8) -> DeleteTasks {
        DeleteTasks {
            task_ids: vec![id]
        }
    }

    fn new_many(ids: Vec<u8>) -> DeleteTasks {
        DeleteTasks {
            task_ids: ids
        }
    }
}


pub enum Action<'k> {
    Add(AddTask<'k>),
    Complete(CompleteTask),
    List,
    Delete(DeleteTasks),
}

pub fn process_arguments<'y>(i: &'y ArgMatches<'y>) -> Result<Action<'y>, String> {
    let add = i.value_of("add");
    let complete = i.value_of("complete");
    let list = i.is_present("list");
    let delete: Option<Result<Vec<u8>, String>> =
        i.values_of("delete")
            .map(|values|
                values.map(|del_id_maybe|
                    parse_id(del_id_maybe)
                        .map_err(|error| format!("Error parsing value '{}'", del_id_maybe))
                ).collect()
            );


   match (add, complete, delete, list) {
       (None, None, None, true) => Ok(Action::List),
       (None, Some(b), None, _) =>
           b.parse::<u8>()
               .map_err(|_| String::from("Not a valid id"))
               .map(|id| Action::Complete(CompleteTask::new(id)))
       ,
       (None, None, Some(Ok(tasks_to_delete)), _) =>
           Ok(Action::Delete(DeleteTasks::new_many(tasks_to_delete))),
       (None, None, Some(Err(error)), _) => Err(error),
       (Some(a), None, None, _) => Ok(Action::Add(AddTask::new(a))),
       (_, _, _, _) => Err(String::from("Not supported yet!")),
   }
}

fn parse_id(id: &str) -> Result<u8, ParseIntError> {
    id.parse::<u8>()
}