use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let counter = Arc::new(AtomicU32::new(0));
    let counter_reads = Arc::clone(&counter);
    let counter_writes = Arc::clone(&counter);

    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/status", get(status))
        .route("/counter", get(move || get_counter(counter_reads)))
        .route("/counter", post(move || increment_counter(counter_writes)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Listening on port :8080");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "root"
}

async fn hello() -> &'static str {
    "Hello, world!"
}

async fn status() -> &'static str {
    "Server is running"
}

async fn get_counter(counter: Arc<AtomicU32>) -> String {
    format!("Count is {}", counter.load(Ordering::SeqCst))
}

async fn increment_counter(counter: Arc<AtomicU32>) -> &'static str {
    counter.fetch_add(1, Ordering::SeqCst);
    "incremented!"
}
