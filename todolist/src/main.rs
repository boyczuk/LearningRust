use std::io::{self};

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("------------------------------");
        println!("Hey! Welcome to your todo list");
        println!("Please select an option");
        println!("------------------------------");
        println!("1. Create new task");
        println!("2. Complete task");
        println!("3. Edit task");
        println!("4. Exit");
        println!("------------------------------");

        

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let answer: i32 = match answer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        if answer == 1 {
            create_task(&mut tasks);
        } else if answer == 2 {
            complete_task(&mut tasks);
        } else if answer == 3 {
            //edit_task(&mut tasks);
        } else if answer == 4 {
            break;
        } else {
            println!("Invalid input");
        }
    }
}

fn create_task(tasks: &mut Vec<String>) {
    println!("Enter task:");
    let mut new_task = String::new();

    io::stdin()
        .read_line(&mut new_task)
        .expect("Failed to read line");

    tasks.push(new_task.trim().to_string());
    println!("Current tasks: {:?}", tasks);
}

fn complete_task(tasks: &mut Vec<String>) {
    println!("------------------------------");
    println!("Current tasks: {:?}", tasks);
    println!("------------------------------");
    println!("Enter task:");
    let mut completed_task = String::new();

    io::stdin()
        .read_line(&mut completed_task)
        .expect("Failed to read line");

    completed_task = completed_task.trim().to_string();

    if let Some(index) = tasks.iter().position(|task| *task == completed_task) {
        tasks.remove(index);
        println!("Task '{}' removed.", completed_task);
    } else {
        println!("Task '{}' not found. ", completed_task);
    }

    println!("Updated tasks: {:?}", tasks);
}
