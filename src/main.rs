use axum::{
    routing::{get, post},
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use std::{env, sync::Arc};
use std::fs;
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Couldn't find database file");

    let shared_state = Arc::new(contents);

    let app = Router::new()
        .route("/db", get(get_db))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_db(
    State(state): State<Arc<String>>,
) -> String {
    let contents  = (*state).clone();
    contents
}