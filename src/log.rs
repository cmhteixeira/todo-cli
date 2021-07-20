use clap::ArgMatches;
use crate::log::Action::Complete;

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
    task_id: u8
}

impl CompleteTask {
    fn new(id: u8) -> CompleteTask {
        CompleteTask {
            task_id: id
        }
    }
}

pub enum Action<'k> {
    Add(AddTask<'k>),
    Complete(CompleteTask),
    List,
}

pub fn process_arguments<'y>(i: &'y ArgMatches<'y>) -> Result<Action<'y>, String> {
    let add = i.value_of("add");
    let complete = i.value_of("complete");
    let list = i.is_present("list");


   match (add, complete, list) {
       (None, None, true) => Ok(Action::List),
       (None, Some(b), _) =>
           b.parse::<u8>()
               .map_err(|_| String::from("Not a valid id"))
               .map(|id| Action::Complete(CompleteTask::new(id)))
       ,
       (Some(a), None, _) => Ok(Action::Add(AddTask::new(a))),
       (_, _, _) => Err(String::from("Not supported yet!")),
   }
}