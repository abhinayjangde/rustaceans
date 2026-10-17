use axum::{Json, Router, routing::get};
use serde_json::json;

#[tokio::main]
async fn main() {
    
    let app = Router::new()
    .route("/health", get(|| async { Json(json!({
        "status": "ok",
        "msg" : "server is running..."
    })) 
}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
    .await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
