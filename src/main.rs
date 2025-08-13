use std::io;
use std::io::stdin;

use todo_list::ToDo;
use todo_list::ToDoList;
use todo_list::FinishedList;
use todo_list::capitalize_first;

fn main() {
    let mut database = ToDoList::new();
    let mut finished = FinishedList::new();

    loop {
        println!("\nWelcome To To_Do_List App, please select a number :\n");
        println!("1. Input a task\n2. See To Do List\n3. Mark Task Done\n4. See Finished Task\n5. Exit program");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = match input.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => {
                println!("\nPlease enter a new task : ");
                let mut new_task = String::new();
                io::stdin().read_line(&mut new_task).expect("Failed to read input");
                database.add_task(capitalize_first(new_task));
            },
            2 => {
                database.task_list();
            },
            3 => {
                println!("\nPlease select the task ID that you have finished :");
                let mut id_done = String::new();
                io::stdin().read_line(&mut id_done).expect("Failed to read task ID");
                let id_done = id_done.trim().parse::<u8>().unwrap();
                database.finished_task(id_done,&mut finished);
            },
            4 => {
                finished.finished_task_list();
            },
            5 => {
                println!("\nThank you for using this app, bye - bye\n");
                break;
            }
            _ => {
                println!("Invalid selection, please select the provided option");
                continue
            }
        }
    }
    
}
