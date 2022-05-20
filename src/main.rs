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

    fn print(&self) {
        for item in &self.items {
            println!("[{}] - {}", item.done, item.todo)
        }
    }
    
}
struct TDItem {
    done: char,
    todo: String
}
impl  TDItem {
    fn new(todo: String) -> TDItem {
        TDItem { done: ' ', todo }
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];
    // let todo_item = TDItem::new("Make todo app with Rust🦀".to_string());
    let mut todo_list = TDList::new();
    todo_list.append("Make todo app with Rust🦀".to_string());

    if cmd == "get" {
        todo_list.print()
    }
}