use axum::{Json, Router, routing::get, http::StatusCode};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/health", get(health));

    let addr = "0.0.0.0:4400";


    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
     axum::serve(listener, app).await.unwrap();
    println!("server is running at http://localhost:4400");

}

async fn health() -> (StatusCode, Json<Value>){
    
    (StatusCode::OK, Json(json!({
        "status":"Ok",
        "msg":"I am healthy :)"
    })))
}   
