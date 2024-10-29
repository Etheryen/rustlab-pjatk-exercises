use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RequestContent {
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct ResponseContent<'a> {
    status: &'a str,
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/submit", post(submit));

    println!("Listening on port :8080");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn submit(Json(payload): Json<RequestContent>) -> Json<ResponseContent<'static>> {
    let response = ResponseContent {
        status: "success",
        name: payload.name,
        age: payload.age,
    };

    Json(response)
}
