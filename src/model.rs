

pub enum Importance {
    One,
    Two,
    Three,
    Four,
    Five
}

pub struct Task<'a> {
    id: u32,
    description: &'a str,
    project: Option<&'a str>,
    importance: Option<Importance>,
    time_stamp: u128
}


pub struct State<'b> {
    active: Vec<Task<'b>>,
    completed: Vec<Task<'b>>,
    vacant_ids: Vec<u8>,
    biggest_id: u8
}


impl<'a> State<'a> {
    pub fn empty() -> State<'static> {
        State {
            active: vec![],
            completed: vec![],
            vacant_ids: vec![],
            biggest_id: 0
        }
    }

    pub fn add_task<'b: 'a>(&mut self, description: &'b str, project: Option<&'b str>, importance: Option<Importance>) -> () {
        let next_id = self.vacant_ids.pop();
        self.vacant_ids.push_b
        self.active.append(&mut vec![Task::new(next_id as u32, description, project, importance)])
    }

    pub fn do_task(&mut self, task_id: u32) -> () {
        let active = self.active.iter().position(|elem| elem.id == task_id);
        match active {
            None => (), // do nothing
            Some(task_index) => {
                let removed_task = self.active.remove(task_index);
                self.completed.append(&mut vec![removed_task]);
            }
        }
    }

    pub fn delete_task(&mut self, task_id: u32) -> () {
        let active = self.active.iter().position(|elem| elem.id == task_id);
        match active {
            None => (), // do nothing
            Some(task_index) => {
                self.active.remove(task_index);
            }
        }

        let completed = self.completed.iter().position(|elem| elem.id == task_id);
        match completed {
            None => (), // do nothing
            Some(task_index) => {
                self.completed.remove(task_index);
            }
        }
    }

    pub fn delete_tasks(&mut self, task_ids: Vec<u8>) -> () {
        for id in task_ids {
            self.delete_task(id as u32)
        }
    }


    pub fn print_tty(&self) -> String {
        let mut res = String::new();
        res.push_str("\u{001b}[1;31mActive\u{001b}[0m \u{23F3}\n");
        let greatest_size = self.active.iter().map(|task|task.id.to_string().len()).max();
        for i in &self.active {
            res.push_str(State::print_task(i, greatest_size.unwrap()).as_str());
        }
        res.push_str("\n");
        res.push_str("\n");

        res.push_str("\u{001b}[1;31mCompleted\u{001b}[0m \u{2705}\n");
        let greatest_size = self.completed.iter().map(|task|task.id.to_string().len()).max();
        for i in &self.completed {
            res.push_str(State::print_task(i, greatest_size.unwrap()).as_str());
        }

        res
    }

    fn print_task<'b>(task: &'b Task<'b>, max_id: usize) -> String {
        format!("{:width$} {} {}\n", task.id, task.description, State::format_project(task.project.unwrap_or_else(||"")), width = max_id)
    }

    fn format_project(project_name: &str) -> String {
        format!("\u{001b}[40;1m\u{001b}[33m{}\u{001b}[0m", project_name)
    }

}