use std::io;
use chrono;
use colored::Colorize;

struct TDItem {
    todo: String,
    done: String
}

fn main() {
    let write = String::from("write");
    let read = String::from("read");
    let quit = String::from("quit");
    

    println!("🧾 todo ...");
    println!("  L --{}  L --{}  L --{}", write.green(), read.yellow(), quit.red());

    loop {
        // get user input
        let mut todo = String::new();
        io::stdin()
            .read_line(&mut todo)
            .expect("Failed to read line!");
        let todo = todo.trim();

        if todo.eq(&quit) {
            break;
        } else if todo.eq(&write) {
            write_list()
        } else if todo.eq(&read) {
            println!("test read");
        } else {
            println!("{}: ({}: to add, {}: to read, {}: to exit the program)", "hint".purple(), write.green(), read.yellow(), quit.red());
            continue;
        };
    }
}

// write function
// adding list to a Todo item
fn td_append(s: String) -> TDItem {
    TDItem { todo: s, done: String::from("[ ]") }
}
// collect user input than add to to do list
fn write_list(){
    let stop = String::from('n');

    loop {
        println!("      L --continue: {}", "[n]".red());
        let mut done = String::new();
        
        // BUGS println!("    -Todo item:"); FIXED
        io::stdin()
           .read_line(&mut done)
           .expect("Failed to read lines");
        let done = done.trim();

        // check if the user is done adding
        if done.eq(&stop){
            println!("  L --{}  L --{}  L --{}","write".green(), 
                                                "read".yellow(), 
                                                "quit".red());
            break;
        } else {
            let todo = done.trim();
            td_append(String::from(todo));
        }
       
    }
}