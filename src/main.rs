use std::fs;
use std::io::prelude::Write;
use std::path::Path;
use todo_cli::log;
use clap::{App, Arg, SubCommand, Values, AppSettings};
use todo_cli::log::{process_arguments, Action};
use todo_cli::data;
use std::io::Read;
use todo_cli::data::{DataPersisted, Task};


mod io;


fn main() -> Result<(), String> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Manage your tasks")
        .arg(Arg::with_name("add")
            .short("a")
            .long("add")
            .help("Add a task/item to your todo list")
            .takes_value(true))
        .arg(Arg::with_name("project")
            .short("p")
            .long("project")
            .help("Specify a project this task should be added too.")
            .takes_value(true)
            .requires("add"))
        .arg(Arg::with_name("context")
            .short("k")
            .long("kontext")
            .help("Specify the context for this task.")
            .takes_value(true)
            .requires("add"))
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("List all active tasks"))
        .arg(Arg::with_name("complete")
            .short("c")
            .long("complete")
            .help("Set a task as being done/completed.")
            .takes_value(true)
            .multiple(true)
            .use_delimiter(true))
        .arg(Arg::with_name("delete")
            .short("d")
            .long("delete")
            .help("Delete a task/item. Works on both 'active' and 'completed' tasks.")
            .takes_value(true)
            .multiple(true)
            .use_delimiter(true)
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let persisted = io::read_user_state()?;

   let deserialized: Option<DataPersisted> = match &persisted {
        None => Ok(None),
        Some(data_str) => serde_json::from_str(data_str.as_str()).map(|data| Some(data)).map_err(|error|error.to_string())
    }?;

   let action = log::process_arguments(&matches)?;

    let mut bar = deserialized.unwrap_or_else(|| DataPersisted::empty());
    let mut what_to_print = String::new();
    match action {
        Action::Add(r) =>
            bar.add_active(r.task_name, r.project, r.context, None),
        Action::Complete(task) => bar.complete_tasks(task.task_ids),
        Action::List => what_to_print.push_str(bar.print_tty().as_str()),
        Action::Delete(log::DeleteTasks {task_ids}) => bar.delete_tasks(task_ids),
    };

    let qux = serde_json::to_string_pretty(&bar).map_err(|x| x.to_string())?;
    std::io::stdout().write(what_to_print.as_bytes());
    io::persist_user_state(format!("{}{}", qux.as_str(), "\n").as_str())
}