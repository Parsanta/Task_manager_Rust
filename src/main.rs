use std::io;
mod task_func;
use crate::task_func::task_func::{task, complete, delete, print_task};

fn main() {
    let mut task_list: Vec<task> = Vec::new();
    let mut input = String::new();
    while input!="5"{
        println!("TASK MANAGER");
        println!("press numbers accordingly:");
        println!("1 => Add task ");
        println!("2 => delete task ");
        println!("3 => mark completed task ");
        println!("4 => print tasks");
        println!("5 => quit ");
    
        input.clear(); 
        io::stdin().read_line(&mut input).expect("invalid");
        let mut input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                return;
            }
        };
        match input {
            1 => {
                println!("Add task :");
                let mut val = String::new();
                io::stdin().read_line(&mut val).expect("invalid");
                let task = task::new(task_list.len() as i32 + 1, val.trim().to_string());
                task_list.push(task);
            }
            2 => {
                println!("Enter task id to delete :");
                let mut val = String::new();
                io::stdin().read_line(&mut val).expect("invalid");
                let val = match val.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                        return;
                    }
                };
                delete(&mut task_list, val);
            }
            3 => {
                println!("Enter task id to mark as completed:");
                let mut val = String::new();
                io::stdin().read_line(&mut val).expect("invalid");
                let val = match val.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                        return;
                    }
                };
                complete(&mut task_list, val);
            }
            4 =>{
                print_task(&task_list);
            }
            5 => {
                println!("Goodbye");
            }
            _ => {
                println!("Invalid choice. Please choose a valid option (1-4).");
            }
        }
    }    
}
