use std::fs;
use std::io::prelude::Write;
use std::path::Path;
use todo_cli::log;
use clap::{App, Arg, SubCommand, Values, AppSettings};
use todo_cli::log::{process_arguments, Action};
use todo_cli::data;
use std::io::Read;
use todo_cli::data::{DataPersisted, Task};
use todo_cli::cli_api;

mod io;


fn main() -> Result<(), String> {
    let matches = cli_api::foo().get_matches();

    let persisted = io::read_user_state()?;

    let deserialized: Option<DataPersisted> = match &persisted {
        None => Ok(None),
        Some(data_str) => serde_json::from_str(data_str.as_str()).map(|data| Some(data)).map_err(|error| error.to_string())
    }?;

    let action = log::process_arguments(&matches)?;

    let mut bar = deserialized.unwrap_or_else(|| DataPersisted::empty());
    let mut what_to_print = String::new();
    match action {
        Action::Add(r) =>
            bar.add_active(r.task_name, r.project, r.context, None),
        Action::Complete(task) => bar.complete_tasks(task.task_ids),
        Action::List => what_to_print.push_str(bar.print_tty().as_str()),
        Action::Delete(log::DeleteTasks { task_ids }) => bar.delete_tasks(task_ids),
        Action::BashCompletion(arg) => {
            println!("console: {}", arg);
            std::io::stdout().write(arg.as_bytes());
            std::process::exit(0);
        }
    };

    let qux = serde_json::to_string_pretty(&bar).map_err(|x| x.to_string())?;
    std::io::stdout().write(what_to_print.as_bytes());
    io::persist_user_state(format!("{}{}", qux.as_str(), "\n").as_str())
}