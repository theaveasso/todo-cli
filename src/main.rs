use std::io;
use colored::Colorize;

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
            println!("test write");
        } else if todo.eq(&read) {
            println!("test read");
        } else {
            println!("{}: ({}: to add, {}: to read, {}: to exit the program)", "hint".purple(), write.green(), read.yellow(), quit.red());
            continue;
        }

    };
}


