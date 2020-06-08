use vial::prelude::*;

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
    fn new() -> TodoDB {
        let db: DB = FileDatabase::from_path("todo.ron", vec![]).unwrap();
        let _ = db.load();
        TodoDB { db }
    }

    fn pop(&self) -> Option<String> {
        self.db.write(|db| db.pop()).unwrap()
    }

    fn push(&self, todo: String) {
        self.db.write(|db| db.push(todo)).unwrap();
        self.db.save().unwrap();
    }

    fn all(&self) -> Vec<String> {
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

trait WithTodos {
    fn db(&self) -> &TodoDB;
}

impl WithTodos for Request {
    fn db(&self) -> &TodoDB {
        self.state::<TodoDB>()
    }
}

routes! {
    GET "/" => list;
    POST "/" => create;
}

fn list(req: Request) -> vial::Result<String> {
    Ok(asset::to_string("layout.html")?.replace(
        "{body}",
        &asset::to_string("list.html")?.replace("{todos}", &todo_partial(&req)?),
    ))
}

fn create(req: Request) -> Option<Response> {
    let todo = req.form("todo")?;
    req.db().push(todo.to_string());
    Some(Response::redirect_to("/"))
}

fn todo_partial(req: &Request) -> vial::Result<String> {
    let mut out = String::new();
    let template = asset::to_string("_todo.html")?;
    for todo in req.db().all() {
        out.push_str(&template.replace("{todo}", &todo));
    }
    Ok(out)
}

fn main() {
    asset_dir!("src/templates");
    use_state!(TodoDB::new());
    run!().unwrap();
}
