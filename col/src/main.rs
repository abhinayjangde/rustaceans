use axum::{Router, routing::get, extract::{Path, Query}};
use std::{collections::HashMap};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/tasks", get(list_task).post(create_task))
        .route("/api/tasks/{id}", get(get_task));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("server is running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn list_task(Query(params): Query<HashMap<String,String>>) -> String {
    let filter = params
    .get("done")
    .map_or("everything".to_string(), |v| format!("done={}", v));

    format!("listing tasks, filter: {}", filter)
}

async fn create_task() -> &'static str{
    
    "new task created"
}

async fn get_task(Path(id): Path<u32>) -> String{
    format!("you asked for task {} ", id)
}
async fn root() -> &'static str {
    "running..."
}
async fn health() -> &'static str {
    "ok"
}
