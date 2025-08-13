use std::io;
use uuid::Uuid;
use cfonts::{say, Colors, Fonts, Options, Rgb};

#[derive(Debug, Clone)]

struct Task {
    id : Uuid,
    title : String,
    description : String,
    mark_done : bool
}

struct List {
    list : Vec<Task>
}

impl List {
    fn add_a_task(&mut self){
        let new_id = Uuid::new_v4();
        let mut title = String::new();
        let mut description = String::new();
        println!("Add a title : ");
        io::stdin().read_line(&mut title).expect("Failed to read title");
        println!("Add a description : ");
        io::stdin().read_line(&mut description).expect("Failed to read description");

        let task = Task{
            id : new_id,
            title : title.to_string().trim().to_owned(),
            description : description.to_string().trim().to_owned(),
            mark_done : false
        };

        self.list.push(task);
        println!("Task added to list");
    }

    fn print_all_task(&self){
       let tasks = self.list.clone();

       for i in tasks {
            println!("-----------------------------------");
            println!("Id : {}", i.id);
            println!("Title : {} ", i.title);
            println!("Description : {} ", i.description);
            println!("Mark Done : {} ", i.mark_done);
            println!("-----------------------------------");
       }
    }

    fn mark_task_done(&mut self){
        let mut  id_input = String::new();
        println!("Enter a id to mark task done : ");
        io::stdin().read_line(&mut id_input).expect("Failed to take UUID ");

        let id_input = id_input.trim().parse::<Uuid>().unwrap();

        for task in &mut self.list {
            if id_input == task.id {
                task.mark_done = !task.mark_done;
                println!("Task mark done");
            } else {
                println!("Task not found");
            }
        }

    }

    fn delete_task(&mut self){
        let mut id_input = String::new();
        println!("Enter an id to delete a task from list : ");
        io::stdin().read_line(&mut id_input).expect("Failed to take input from id for deleting a task");

        let id_input =  id_input.trim().parse::<Uuid>().unwrap();

        let mut index = 0;
        for task in self.list.iter() {
            if task.id == id_input {
                break;
            } 
            index = index + 1;
        }

        self.list.remove(index);

    }

}

fn main() {
    // let mut list : Vec<Task> = Vec::new();
    let mut list = List {
        list : vec![]
    };

    println!();
    // let figlet_font = FIGfont::standard().unwrap();
    // let font = figlet_font.convert("TODO APP");
    // println!("{}" , font.unwrap());
    say(Options {
        text: String::from("Todo App"),
        font: Fonts::FontBlock,
        colors: vec![
            Colors::Rgb(Rgb::Val(0, 0, 0)),
            Colors::GreenBright
        ],
        ..Options::default()
    });
    println!();
    loop {
        println!("\x1b[1m Options : \x1b[0m ");
        println!("1. Add a task 📝");
        println!("2. Mark task done ✅");
        println!("3. Delete task ❌");
        println!("4. Print list of task 📄");
        println!("5. Exit 🔚");
        println!();

        let mut input = String::new();
        println!("Enter your choice : ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let ch  = input.trim().parse::<i32>().unwrap();

        match ch {
            1 => list.add_a_task(),
            2 => list.mark_task_done(),
            3 => list.delete_task(),
            4 => list.print_all_task(),
            5 => {
                println!("See you Soon👋");
                break;
            },
            _ => println!("Enter a valid option")
        }
        println!();
    }
}


