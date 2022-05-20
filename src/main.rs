use std::env;

struct TDList {
    items: Vec<TDItem>
}
impl TDList {
    fn new() -> TDList {
        TDList { items: Vec::new() }
    }

    fn append(&mut self, todo: String) {
        let item = TDItem::new(todo);
        self.items.push(item);
    }

    fn remove(&mut self, idx: usize) {
        self.items.remove(idx);
    }

    fn mark_done(&mut self, idx: usize){
        if self.items[idx].done == ' '{
            self.items[idx].done = 'x';
            // change color to green
        } else {
            self.items[idx].done = ' ';
        }
    }

    fn print(&self) {
        for (idx, item) in self.items.iter().enumerate() {
            println!("{} [{}] - {}", idx, item.done, item.todo)
        }
    }
    
}

struct TDItem {
    done: char,
    todo: String,
}
impl  TDItem {
    fn new(todo: String) -> TDItem {
        TDItem { done: ' ', todo }
    }
    
}

enum Command {
    Get,
    Add(String),
    Done(usize),
    Remove(usize)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "done" => Command::Done(args[2].parse().expect("Error converting to Interger")),
        "remove" => Command::Remove(args[2].parse().expect("Error converting to Interger")),
        _ => panic!("You must provide an accepted command!")
        
    };
    // let todo_item = TDItem::new("Make todo app with Rust🦀".to_string());
    let mut todo_list = TDList::new();
    todo_list.append("Make todo app with Rust🦀 v-1.2".to_string());
    todo_list.append("Build an application".to_string());
    todo_list.append("Improve visualization adding color".to_string());
    todo_list.append("Implement a run a loop and ask the user for their comman every iteration".to_string());
    todo_list.append("Implement a comman for changing the task description".to_string());
    todo_list.append("Implement a custom sort command (Priority, Due date ...)".to_string());
    todo_list.append("Push task data to a todo.data".to_string());
    todo_list.mark_done(0);
    todo_list.mark_done(1);

    


    match cmd {
        Command::Get => todo_list.print(),
        Command::Add(todo) => {
            todo_list.append(todo.to_string());
            todo_list.print()
        },
        Command::Done(idx) => {
            todo_list.mark_done(idx);
            todo_list.print()
        },
        Command::Remove(idx) => {
            todo_list.remove(idx);
            todo_list.print()  
        }
    }
}