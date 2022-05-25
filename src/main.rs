use std::io;
use std::env;
use colored::ColoredString;
use colored::Colorize;

struct TDList {
    items: Vec<TDItem>
}
impl TDList {
    fn new() -> TDList {
        TDList { items: Vec::new() }
    }

    fn append(&mut self, todo: String, p_level: usize) {
        let item = TDItem::new(todo, p_level);
        self.items.push(item);
    }

    fn remove(&mut self, idx: usize) {
        self.items.remove(idx);
    }

    fn update(&mut self, idx: usize, todo: String) {
        self.items[idx].todo = todo;
    }

    fn mark_done(&mut self, idx: usize){
        if self.items[idx].done == " ".truecolor(50, 50, 50){
            self.items[idx].done = "x".green();
            // change color to green
        } else {
            self.items[idx].done = " ".truecolor(50, 50, 50);
        }
    }

    fn sorted(&self) {
        println!("hello world");
    }

    fn print(&self) {
        println!("{}", "Tasks:".yellow());
        for (idx, item) in self.items.iter().enumerate() {
            if self.items[idx].done == "x".green() {
                println!("{} {} -- [{}] - {}", idx,item.prio.level, item.done, item.todo.truecolor(50, 50, 50));
            } else {
                println!("{} {} -- [{}] - {}", idx,item.prio.level, item.done, item.todo);
                
            }
        }
    }
    
}

struct TDItem {
    done: ColoredString,
    todo: String,
    prio: Priority
}

impl  TDItem {
    fn new(todo: String, prio:usize) -> TDItem {
        let p_level = Priority::new(prio);
        TDItem { done: " ".truecolor(50, 50, 50), todo, prio: Priority{level:p_level}}
    }
    
}

#[derive(Debug)]
struct Priority {
    level: ColoredString
}
impl Priority {
    fn new(level: usize) -> ColoredString {
        let p_level = match level {
            1 => "high       ".red(),
            2 => "medium     ".purple(),
            3 => "low        ".blue(),
            _ => "no priority".white()
        };
        return p_level;
    }
    
}

enum Sorted {
    done,
    undone,
    priority
}

enum CommandArgs {
    Get,
    Sort,
    Todo,
    Add(String, usize),
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
    Sort,
    Invalid,
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = match args[1].as_str() {
        "get" => CommandArgs::Get,
        "todo" => CommandArgs::Todo,
        "add" => CommandArgs::Add(args[2].clone(), args[3].parse().unwrap()),
        "done" => CommandArgs::Done(args[2].parse().expect("Error converting to Interger")),
        "remove" => CommandArgs::Remove(args[2].parse().expect("Error converting to Interger")),
        "update" => CommandArgs::Update(args[2].parse().unwrap(), args[3].clone()),
        "sorted" => CommandArgs::Sort,
        _ => panic!("You must provide an accepted command!")

    };

    // let todo_item = TDItem::new("Make todo app with Rust🦀".to_string());
    let mut todo_list = TDList::new();
    todo_list.append("Make todo app with Rust🦀 v-1.2".to_string(), 1);
    todo_list.append("Build an application".to_string(), 2);
    todo_list.append("Improve visualization adding color".to_string(), 3);
    todo_list.append("Implement a run a loop and ask the user for their comman every iteration".to_string(), 1);
    todo_list.append("Implement a command for changing the task description".to_string(), 4);
    todo_list.append("Implement a custom sort command (Priority, Due date ...)".to_string(), 3);
    todo_list.append("Implement a Save and Load Json file".to_string(), 1);
    
    // done 
    todo_list.mark_done(0);
    todo_list.mark_done(1);
    todo_list.mark_done(2);
    todo_list.mark_done(3);
    todo_list.mark_done(4);
 
    
    match cmd {
        CommandArgs::Get => todo_list.print(),
        CommandArgs::Sort=> todo_list.sorted(),
        CommandArgs::Add(todo, p_level) => {
            todo_list.append(todo.to_string(), p_level);
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
                println!("    L --{}    L --{}    L --{}    L --{}    L --{}    L --{}    L --{}",
                        "get".green(),
                        "add".green(),
                        "done".green(),
                        "remove".green(),
                        "update".green(),
                        "sorted".green(),
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
                            println!("{} 1 - 🟥 {}    2 - 🟪 {}    3 - 🟦 {}   4 - ⬜️ {}", "Priority Level:".yellow()
                                                                     , "high".red()
                                                                     , "meduim".purple()
                                                                     , "low".blue()
                                                                     , "no priority");
                            let p_level = input().parse().unwrap();

                            println!("{}", "task:".yellow());
                            let todo = input();

                            // ask if the use want to continue adding todo
                            println!("{} / {}", "-y continue".yellow(), "-n".red());
                            let continue_adding = input();
                            if continue_adding == 'n'.to_string() {
                               break;
                            }

                            // push to vector
                            todo_list.append(todo, p_level);
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
                    Command::Sort => {
                        loop {
                            print_header("sorted by priority");
                            let sorted_q = input();
                            match sorted_q {
                                Sorted::done => println!("done"),
                                "undone" => println!("undone"),
                                "priority" => println!("priority"),
                                "_" => panic!()
                            }
                        }
                    }
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
        "sorted" => Command::Sort,
        _ => Command::Invalid

    };
    return cmd;
}

