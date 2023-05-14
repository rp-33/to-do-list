use std::io;

struct Task{
    task: String,
    done: bool,
}

impl Task{
    fn create(task: String,done: bool)->Task{
        Task { task, done }
    }

    fn find(&self){
        println!("la tarea es {}",self.task)
    }
}


fn create_task(task: String,done: bool){
    let todo_instance = Task::create(task,done);
    println!("la tarea es => {}",todo_instance.task)
}

fn main() {
    loop {
        let mut action = String::new();
        let mut task = String::new();
        let stdi = io::stdin();

        println!("Que accion deseas realizar?");
        io::stdin().read_line(&mut action);
      
        
        if action.trim().to_owned() == "break" {
            break;
        }

        println!("Ingrese el nombre de la tarea");
        io::stdin().read_line(&mut task);
        
        println!("action {}",action);

        match action.trim().to_owned().as_ref() {
            "create" => create_task(task.to_string(),false),
            _ => println!("la opcion es invalida"),
        }
        

    }
}
