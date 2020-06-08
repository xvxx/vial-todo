use vial::prelude::*;

mod db;
use db::{RequestWithTodos, TodoDB};

routes! {
    GET "/" => list;
    POST "/" => create;
    POST "/check/:id" => check;
}

fn check(req: Request) -> impl Responder {
    if let Some(id) = req.arg("id") {
        let id = id.parse().unwrap_or(0);
        if id < req.todos().len() {
            req.todos().check(id);
        }
    }
}

fn list(req: Request) -> vial::Result<String> {
    Ok(asset::to_string("layout.html")?.replace(
        "{body}",
        &asset::to_string("list.html")?.replace("{todos}", &todo_partial(&req)?),
    ))
}

fn create(req: Request) -> Option<Response> {
    let todo = req.form("todo")?;
    req.todos().push(todo.to_string());
    Some(Response::redirect_to("/"))
}

fn todo_partial(req: &Request) -> vial::Result<String> {
    let mut out = String::new();
    let template = asset::to_string("_todo.html")?;
    for (id, todo) in req.todos().all().iter().enumerate() {
        let checked = if todo.0 { "checked='checked'" } else { "" };
        out.push_str(
            &template
                .replace("{checked}", checked)
                .replace("{id}", &id.to_string())
                .replace("{todo}", &todo.1),
        );
    }
    Ok(out)
}

fn main() {
    asset_dir!("src/templates");
    use_state!(TodoDB::new());
    run!().unwrap();
}
