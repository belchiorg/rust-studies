use std::io::stdin;

struct Task {
    text: String,
    done: bool,
}

fn list_tasks(tasks: &mut Vec<Task>) {
    print!("\x1B[2J\x1B[1;1H");
    let mut index = 0;
    for task in tasks {
        if task.done {
            println!("{}. {}Done:✔️",index, task.text);
        }
        else {
            println!("{}. {}Done:❌",index, task.text);
        }
        index += 1;
        print!("\n");
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    print!("\x1B[2J\x1B[1;1H");
    let mut text = String::new();
    println!("ToDo text:");
    stdin().read_line(&mut text).expect("Couldn't read stdin");
    tasks.push(Task { text: text, done: false });
    print!("\x1B[2J\x1B[1;1H");
}

fn delete_task(tasks: &mut Vec<Task>) {
    print!("\x1B[2J\x1B[1;1H");
    list_tasks(tasks);
    let mut index= String::new();
    println!("Type the index of the task you want to remove:");
    stdin().read_line(&mut index).expect("Couldn't read stdin");
    
    match index.trim().parse::<usize>() {
        Ok(ind) => {
            if ind >= tasks.len() {
                println!("Index out of range!");
            }
            else {
                tasks.remove(ind);
                print!("\x1B[2J\x1B[1;1H");
            }
        },
        Err(_) => println!("Not a valid number!"),
    }
}

fn mark_task(tasks: &mut Vec<Task>) {
    print!("\x1B[2J\x1B[1;1H");
    list_tasks(tasks);
    let mut index = String::new();

    stdin().read_line(&mut index).expect("Couldn't read stdin");

    match index.trim().parse::<usize>() {
        Ok(ind) => {
            if ind >= tasks.len() {
                println!("Index out of range!");
            }
            else {
                match tasks.get(ind) {
                    Some(task) => {
                        tasks.push(Task { text: task.text.clone(), done: true });
                        tasks.swap_remove(ind);
                        print!("\x1B[2J\x1B[1;1H");
                    }
                    None => println!("Couldn't find that task")
                }
            }
        },
        Err(_) => println!("Not a valid number!"),
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut text = String::new();

    println!("Hello my friend! Welcome to Biro's todo cli app!\n\n");

    loop {
        println!("Select one of the options:");
        println!("1. List all tasks");
        println!("2. Add a new task");
        println!("3. Delete a task");
        println!("4. Mark a task as done");

        stdin().read_line(&mut text).expect("Couldn't read stdin");
        
        let option = match text.trim().parse::<u8>() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid number");
                0   
            }
        };

        match option {
            1 => list_tasks(&mut tasks),
            2 => add_task(&mut tasks),
            3 => delete_task(&mut tasks),
            4 => mark_task(&mut tasks),
            _ => println!("Invalid option"),
        };

        text = String::new();
    }
}
