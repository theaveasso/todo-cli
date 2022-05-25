pub struct TDList {
    items: Vec<TDItem>
}
impl TDList {
    pub fn new() -> TDList {
        TDList { items: Vec::new() }
    }

    pub fn append(&mut self, todo: String, p_level: usize) {
        let item = TDItem::new(todo, p_level);
        self.items.push(item);
    }

    pub fn remove(&mut self, idx: usize) {
        self.items.remove(idx);
    }

    pub fn update(&mut self, idx: usize, todo: String) {
        self.items[idx].todo = todo;
    }

    pub fn mark_done(&mut self, idx: usize){
        if self.items[idx].done == ' '{
            self.items[idx].done = 'x';
            // change color to green
        } else {
            self.items[idx].done = ' ';
        }
    }

    pub fn sorted() {
        // priority sorted
        // done sorted 
        // undone sorted
    }

    pub fn print(&self) {
        println!("{}", "Tasks:".yellow());
        for (idx, item) in self.items.iter().enumerate() {
            if self.items[idx].done == 'x' {
                println!("{} {} -- [{}] - {}", idx,item.prio.level, item.done, item.todo.truecolor(50, 50, 50));
            } else {
                println!("{} {} -- [{}] - {}", idx,item.prio.level, item.done, item.todo);
                
            }
        }
    }
    
}