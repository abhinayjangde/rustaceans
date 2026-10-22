use axum::{routing::get, Router};

async fn root() -> &'static str{
    "i am running"
}
async fn health() -> &'static str{
    "i am healthy"
}

#[tokio::main]
async fn main() {
  let app = Router::new()
  .route("/", get(root))
  .route("/health", get(health));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("server is running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}


