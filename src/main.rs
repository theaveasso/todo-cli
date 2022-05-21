use std::io;
use std::env;
use colored::Colorize;


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

    fn update(&mut self, idx: usize, todo: String) {
        self.items[idx].todo = todo;
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
        println!("{}", "Tasks:".yellow());
        for (idx, item) in self.items.iter().enumerate() {
            if self.items[idx].done == 'x' {
                println!("{} [{}] - {}", idx, item.done, item.todo.truecolor(50, 50, 50));
            } else {
                println!("{} [{}] - {}", idx, item.done, item.todo);
                
            }
        }
    }
    
}

struct TDItem {
    done: char,
    todo: String,
    prio: u8
}
impl  TDItem {
    fn new(todo: String) -> TDItem {
        TDItem { done: ' ', todo, prio:2}
    }
    
}

enum CommandArgs {
    Get,
    Todo,
    Add(String),
    Done(usize),
    Remove(usize),
    Update(usize, String),

}

enum  Command {
    Get,
    Add,
    Done,
    Quit,
    Remove,
    Update,
    Invalid,
}


fn main() {
    // color 
    let grey = 50;
    let args: Vec<String> = env::args().collect();

    let cmd = match args[1].as_str() {
        "get" => CommandArgs::Get,
        "todo" => CommandArgs::Todo,
        "add" => CommandArgs::Add(args[2].clone()),
        "done" => CommandArgs::Done(args[2].parse().expect("Error converting to Interger")),
        "remove" => CommandArgs::Remove(args[2].parse().expect("Error converting to Interger")),
        "update" => CommandArgs::Update(args[2].parse().unwrap(), args[3].clone()),
        _ => panic!("You must provide an accepted command!")

    };

    // let todo_item = TDItem::new("Make todo app with Rust🦀".to_string());
    let mut todo_list = TDList::new();
    todo_list.append("Make todo app with Rust🦀 v-1.2".to_string());
    todo_list.append("Build an application".to_string());
    todo_list.append("Improve visualization adding color".to_string());
    todo_list.append("Implement a run a loop and ask the user for their comman every iteration".to_string());
    todo_list.append("Implement a command for changing the task description".to_string());
    todo_list.append("Implement a custom sort command (Priority, Due date ...)".to_string());
    todo_list.append("Implement a Push task data to a todo.data".to_string());
    
    // done 
    todo_list.mark_done(0);
    todo_list.mark_done(1);
    todo_list.mark_done(2);
    todo_list.mark_done(3);
    todo_list.mark_done(4);
 


    
    match cmd {
        CommandArgs::Get => todo_list.print(),
        CommandArgs::Add(todo) => {
            todo_list.append(todo.to_string());
            todo_list.print()
        },
        CommandArgs::Done(idx) => {
            todo_list.mark_done(idx);
            todo_list.print()
        },
        CommandArgs::Remove(idx) => {
            todo_list.remove(idx);
            todo_list.print()  
        },
        CommandArgs::Update(idx, todo) => {
            todo_list.update(idx, todo);
            todo_list.print()
        },
        CommandArgs::Todo => {
            println!("Rust🦀 todo app");
            loop {
                println!("    L --{}    L --{}    L --{}    L --{}    L --{}    L --{}",
                        "get".green(),
                        "add".green(),
                        "done".green(),
                        "remove".green(),
                        "update".green(),
                        "quit".red());
                let cmd = cmd_from_user(input());

                match cmd {
                    Command::Get => {
                        print_header("get");
                        todo_list.print();
                    }
                    Command::Add => {
                        loop {
                            print_header("add");
                            println!("{} / {}", "-[ ] add task".yellow(), "-n".red());
                            let todo = input();
                            if todo == 'n'.to_string() {
                               break;
                            }
                            todo_list.append(todo);
                            todo_list.print()
                        }
                    }
                    Command::Done => {
                        loop {
                            print_header("done");
                            todo_list.print();
                            println!("{} / {}", "-[ ] task index".yellow(), "-n".red());
                            let mut _idex = input();
                            if _idex != 'n'.to_string() {
                                let idx: usize = _idex.parse().unwrap();
                                todo_list.mark_done(idx);
                                todo_list.print();

                            } else if _idex.to_string() == 'n'.to_string() {
                                break
                            }
                        }
                    },
                    Command::Quit => break,
                    Command::Remove => {
                        loop {
                            print_header("remove");
                            todo_list.print();
                            println!("{} / {}", "-[ ] task index".yellow(), "-n".red());
                            let mut _idex = input();
                            if _idex != 'n'.to_string() {
                                let idx: usize = _idex.parse().unwrap();
                                todo_list.remove(idx);

                            } else if _idex.to_string() == 'n'.to_string() {
                                break
                            }
                        }
                    },
                    Command::Update => {
                        loop {
                            print_header("update");
                            todo_list.print();
                            println!("{} / {}", "-[ ] task index".yellow(), "-n".red());
                            let mut _idex = input();

                            if _idex != 'n'.to_string() {
                                // break if user not input the interger
                                let idx: usize = _idex.parse().unwrap();
                                println!("{}", "-[ ] update to:".yellow());
                                let todo = input();
                                todo_list.update(idx, todo);

                            } else if _idex.to_string() == 'n'.to_string() {
                                break
                            }
                            // BUG N IS LARGER THAN LEN OF LIST
                        }
                    },
                    Command::Invalid => todo_list.print(),
                }

            }
        },
    }
}

fn print_header(s: &str) {
    println!("    L --{} ",s.purple());
}

fn input() -> String{
    let mut msg = String::new();
    io::stdin()
        .read_line(&mut msg)
        .expect("Failed to read lines");
    
    msg = msg.trim().to_string();
    return msg;
}

fn cmd_from_user(s: String) -> Command {
    let cmd = match s.as_str() {
        "get" => Command::Get,
        "add" => Command::Add,
        "done" => Command::Done,
        "quit" => Command::Quit,
        "remove" => Command::Remove,
        "update" => Command::Update,
        _ => Command::Invalid

    };
    return cmd;
}