use axum::{
    routing::get,
    extract::State,
    Router,
};
use std::path::PathBuf;
use std::{env, sync::Arc};
use std::fs;
use clap::{arg, command, value_parser, Command};
use std::net::SocketAddr;
use serde_json::{Result, Value};
use handlers::*;

pub mod handlers;

#[tokio::main]

async fn main() {
    let matches = cli().get_matches();

    let db_path = matches.get_one::<PathBuf>("watch").unwrap();
    let port = matches.get_one::<u16>("port").unwrap().clone(); 
    let contents = fs::read_to_string(db_path)
        .expect("Couldn't find database file");
    let parsed_json: Value = serde_json::from_str(&contents).unwrap();

    let app = get_routes(Arc::new(parsed_json));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_routes(state: Arc<Value>) -> Router {
    let parsed_json = (*state).clone();

    let json_map = parsed_json.as_object().unwrap();

    for (key, value) in json_map {
        println!("{}: {}", key, value);
    }

    let router = Router::new()
        .route("/db", get(get_db))
        .route("/", get(get_static));

    return router.with_state(state)
}

fn cli() -> Command {
    command!()
        .arg(
            arg!(
                --watch <FILE> "Set file to watch and serve"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
            --port <PORT_NUMBER> "Set port to host the server on"
            )
            .default_value("5000")
            .value_parser(value_parser!(u16))
        )
}