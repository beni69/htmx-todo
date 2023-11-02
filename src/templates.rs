use crate::db::get_todos;
use askama::Template;
use axum::response::{IntoResponse, Response};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    todos: Vec<String>,
}

pub async fn index() -> Response {
    let maybe_todos = get_todos().await;
    match maybe_todos {
        Ok(todos) => Index { todos }.into_response(),
        Err(e) => {
            eprintln!("{e}");
            "ERROR".into_response()
        }
    }
}

#[derive(Template)]
#[template(path = "todos.html")]
pub struct Todos {
    todos: Vec<String>,
}

pub async fn todos() -> Response {
    let maybe_todos = get_todos().await;
    match maybe_todos {
        Ok(todos) => Todos { todos }.into_response(),
        Err(e) => {
            eprintln!("{e}");
            "ERROR".into_response()
        }
    }
}

// #[derive(Template)]
// #[template(path = "hello.html")]
// pub struct Hello {
//     pub name: String,
// }
//
// pub async fn render() -> impl IntoResponse {
//     Hello {
//         name: "Beni".to_string(),
//     }
// }
// fn main() {
//     let name = "Beni".to_string();
//     let rendered = (templates::Hello { name }).render().unwrap();
//
//     println!("{rendered}");
// }
