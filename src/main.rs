use std::io;

struct Task{
    task: String,
    done: bool,
}

impl Task{
    fn create(task: String,done: bool)->Task{
        return Task { task, done };
    }

    fn show_all(tasks: &mut Vec<Task>){
        for item in tasks.iter() { 
            println!("tarea=> {}",item.task)
        }
    }
}


fn create_task(tasks: &mut Vec<Task>,task: String,done: bool){
    let new_task = Task::create(task,done);
    tasks.push(new_task);
    println!("la tarea creada es=>{}",tasks[tasks.len() - 1].task);
}

fn main() {
    //creamos un nuevo vector mutable de tipo Task
    let mut tasks: Vec<Task> = Vec::new(); 
    //iteramos
    loop {
        let mut action = String::new();
        let mut task = String::new();
        let stdi = io::stdin();

        println!("Que accion deseas realizar?");
        io::stdin().read_line(&mut action);
      
        action = action.trim().to_owned();

        if action == "break" {
            break;
        }

        if action == "create" {
            println!("Ingrese el nombre de la tarea");
            io::stdin().read_line(&mut task);
            task = task.trim().to_owned();
        }
        
        println!("action {}",action);

        match action.trim().to_owned().as_ref() {
            "create" => create_task(&mut tasks,task.to_string(),false),
            "show all" => Task::show_all(&mut tasks),
            _ => println!("la opcion es invalida"),
        }
        

    }
}
