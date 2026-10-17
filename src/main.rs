use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn home() -> Json<Value> {
    Json(json!({
        "home": true,
        "status": "running..."
    }))
}

#[tokio::main]
async fn main() {
    
    let app = Router::new()
    .route("/", get(home))

    .route("/health", get(|| async { Json(json!({
        "status": "ok",
        "msg" : "server is running..."
    })) 
    
}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
    .await.unwrap();

    println!("server is running at http://localhost;4000");
    axum::serve(listener, app).await.unwrap();
}
