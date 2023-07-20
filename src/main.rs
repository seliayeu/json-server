use axum::routing::{post, put};
use axum::{
    routing::get,
    extract::State,
    Router,
};
use std::path::{PathBuf, Path};
use std::sync::Mutex;
use std::{env, sync::Arc};
use std::fs;
use clap::{arg, command, value_parser, Command};
use std::net::SocketAddr;
use serde_json::{Result, Value, Map};
use handlers::*;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

pub mod handlers;

#[derive(Clone)]
pub struct AppState {
    json_dict: Arc<Mutex<Map<String, Value>>>,
    json_path: PathBuf,
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    let db_path = matches.get_one::<PathBuf>("watch").unwrap().clone();


    let port = *matches.get_one::<u16>("port").unwrap(); 
    let contents = fs::read_to_string(db_path.clone())
        .expect("Couldn't find database file");

    let parsed_json: Value = serde_json::from_str(&contents).unwrap();
    let parsed_json = parsed_json.as_object().unwrap().clone();
    let state = AppState{ 
        json_dict: Arc::new( Mutex::new(parsed_json)),
        json_path: db_path.clone(),
    };

    let app = get_routes(state.clone());
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // tokio::spawn(async move {
    //     if let Err(e) = watch(state.clone()) {
    //         println!("error: {:?}", e)
    //     }
    // });

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


// fn watch(state: AppState) -> notify::Result<()> {
//     let (tx, rx) = std::sync::mpsc::channel();

//     let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

//     let path = state.json_path;
//     watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

//     let mut json_data = state.json_dict.as_ref().lock().unwrap();

//     for res in rx {
//         match res {
//             Ok(event) => {
//                 assert!(event.paths.len() == 1);
//                 let path = event.paths.get(0).unwrap();
//                 let contents = fs::read_to_string(path).unwrap();
//                 println!("{:?}", contents);
//                 let contents_str = contents.as_str();
//                 println!("{:?}", contents_str);
//                 let parsed_json: Value = serde_json::from_str(contents_str).unwrap();
//             },
//             Err(e) => println!("watch error: {:?}", e),
//         }
//     }

//     Ok(())
// }

fn get_routes(state: AppState) -> Router {
    let router = Router::new()
        // .route("/db", get(get_db))
        // .route("/", get(get_static))
        .route("/:data_type", post(post_json))
        .route("/:data_type/:id", put(put_json))
        .route("/:data_type/:id/:nested", post(post_nested_json));

    router.with_state(state)
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