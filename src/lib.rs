#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ToDo {
    id: u8,
    task: String,
    status: bool,
}

pub struct ToDoList {
    list: Vec<ToDo>,
}

impl ToDoList {
    pub fn new() -> ToDoList {
        ToDoList {
            list: vec![],
        }
    }

    pub fn add_task(&mut self, task: String) {
        let id = self.list.len() as u8 + 1;
        let new_task = ToDo {
            id,
            task: task.clone(),
            status: false,
        };
        self.list.push(new_task);
        println!("Added : {}", task);
    }

    pub fn task_list(&self) {
        if self.list.is_empty() {
            println!("\nYour task is empty");
            return;
        }

        let max_task_len = self
            .list
            .iter()
            .map(|item| item.task.len())
            .max()
            .unwrap_or(4);

        println!("\n{:<5} {:<width$} {:<10}", "ID", "TASK", "STATUS", width = max_task_len);
        println!("{}","-".repeat(5 + 1 + max_task_len + 1 + 10));

        for item in &self.list {
            let clean_task = item.task.replace("\n", "").trim().to_string();
            println!("{:<5} {:<width$} {:<10}", 
            item.id, 
            clean_task, 
            if !item.status { "Not done" } else { "Done" },
            width = max_task_len)
        };
    }

    pub fn finished_task(&mut self, id: u8, finished: &mut FinishedList) {
        if let Some(task) = self.list.iter_mut().find(|task| task.id == id) {
            task.status = true;
            let clean_task = task.task.replace("\n", "").trim().to_string();
            println!("Task: {} has been done, congrats!!", clean_task);
        };

        let mut done: Vec<ToDo> = Vec::new();

        self.list.retain(|task|
            if task.status {
                done.push(task.clone());
                false
            } else {
                true
            }
        );

        finished.list.extend(done);
        finished.list.sort();

    }
}

pub struct FinishedList {
    list: Vec<ToDo>,
}

impl FinishedList {
    pub fn new() -> FinishedList {
        FinishedList {
            list: vec![],
        }
    }

    pub fn finished_task_list(&self) {
        if self.list.is_empty() {
            println!("\nYou have not done anything");
            return;
        }

        let max_task_len = self
            .list
            .iter()
            .map(|item| item.task.len())
            .max()
            .unwrap_or(4);

        println!("\n{:<5} {:<width$} {:<10}", "ID", "TASK", "STATUS", width = max_task_len);
        println!("{}","-".repeat(5 + 1 + max_task_len + 1 + 10));

        for item in &self.list {
            let clean_task = item.task.replace("\n", "").trim().to_string();
            println!("{:<5} {:<width$} {:<10}", 
            item.id, 
            clean_task, 
            if !item.status { "Not done" } else { "Done" },
            width = max_task_len)
        };
    }
}

pub fn capitalize_first(word: String) -> String {
    let mut char = word.chars();
    match char.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + char.as_str(),
    }
}