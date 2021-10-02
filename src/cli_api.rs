use clap::{App, Arg, AppSettings};

pub fn foo() -> App<'static, 'static> {

    let api =
        App::new(env!("CARGO_PKG_NAME"))
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
        .setting(AppSettings::ArgRequiredElseHelp);


    let tab_completion =
        App::new("tab-completion")
        .setting(AppSettings::Hidden)
            .arg(Arg::with_name("project")
                .short("p")
                .long("project")
            );


    api.subcommand(tab_completion)

}