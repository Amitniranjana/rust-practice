use axum::{
    Json, Router, debug_handler, routing::get
};
use serde::Serialize;

#[tokio::main]

async fn main() {
    let router01 = Router::new().route("/vehicle", get(vehicle_get));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, router01).await.unwrap();
}

#[derive(Serialize)]
struct Vehicle {
    manufacture: String,
    model: i32,
    name: String,
}
#[debug_handler]
async fn vehicle_get() -> Json<Vehicle> {
    println!("retrieved data");

    Json(Vehicle {
        manufacture: "bmw".to_string(),
        model: 1500,
        name: "best car".to_string(),
    })
}
