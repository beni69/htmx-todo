mod db;
mod templates;

use askama_axum::IntoResponse;
use axum::{
    extract::Path,
    routing::{delete, get, post},
    Form, Router,
};
use std::{collections::HashMap, net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(templates::index))
        .route("/todo", get(post_todo))
        .route("/todo", post(post_todo))
        .route("/todo/:id", delete(delete_todo));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn post_todo(Form(form): Form<HashMap<String, String>>) -> impl IntoResponse {
    let todo = form.get("todo").unwrap();
    dbg!(todo);

    db::add_todo(todo.to_string()).await.unwrap();

    templates::todos().await
}

async fn delete_todo(Path(path): Path<String>) -> impl IntoResponse {
    let id: usize = path.parse().unwrap();
    dbg!(id);

    db::delete_todo(id).await.unwrap();

    templates::todos().await
}
