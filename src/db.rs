extern crate rustbreak;
use rustbreak::{deser::Ron, FileDatabase};

type DB = FileDatabase<Vec<String>, Ron>;

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
        let db: DB = FileDatabase::from_path("todo.ron", vec![]).unwrap();
        let _ = db.load();
        TodoDB { db }
    }

    pub fn pop(&self) -> Option<String> {
        self.db.write(|db| db.pop()).unwrap()
    }

    pub fn push(&self, todo: String) {
        self.db.write(|db| db.push(todo)).unwrap();
        self.db.save().unwrap();
    }

    pub fn all(&self) -> Vec<String> {
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

pub trait WithTodos {
    fn db(&self) -> &TodoDB;
}

impl WithTodos for vial::Request {
    fn db(&self) -> &TodoDB {
        self.state::<TodoDB>()
    }
}
