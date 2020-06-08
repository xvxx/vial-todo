use std::mem;

extern crate rustbreak;
use rustbreak::{deser::Ron, FileDatabase};

type DB = FileDatabase<Vec<Todo>, Ron>;
const DB_PATH: &str = "todo.ron";

pub type Todo = (bool, String);

pub struct TodoDB {
    db: DB,
}

impl Drop for TodoDB {
    fn drop(&mut self) {
        let _ = self.db.save();
    }
}

impl TodoDB {
    pub fn new() -> TodoDB {
        let db: DB = FileDatabase::from_path(DB_PATH, vec![]).unwrap();
        let _ = db.load();
        TodoDB { db }
    }

    pub fn len(&self) -> usize {
        self.db.read(|db| db.len()).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn pop(&self) -> Option<String> {
        self.db.write(|db| db.pop()).unwrap().map(|t| t.1)
    }

    pub fn push(&self, todo: String) {
        self.db.write(|db| db.push((false, todo))).unwrap();
        self.db.save().unwrap();
    }

    /// Toggle TODO status of an individual item.
    pub fn check(&self, id: usize) {
        self.db
            .write(|db| {
                if id < db.len() {
                    let mut todo = db[id].clone();
                    todo.0 = !todo.0;
                    mem::replace(&mut db[id], todo);
                }
            })
            .unwrap();
        self.db.save().unwrap();
    }

    pub fn all(&self) -> Vec<Todo> {
        let mut todos = vec![];
        self.db
            .read(|db| {
                for todo in db {
                    todos.push(todo.clone());
                }
            })
            .unwrap();
        todos
    }
}

pub trait RequestWithTodos {
    fn todos(&self) -> &TodoDB;
}

impl RequestWithTodos for vial::Request {
    fn todos(&self) -> &TodoDB {
        self.state::<TodoDB>()
    }
}
