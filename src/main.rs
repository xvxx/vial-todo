use vial::prelude::*;

mod db;
use db::{TodoDB, WithTodos};

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
