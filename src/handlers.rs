use axum::{extract::Path, extract::State, routing::put, Json, Router, extract::MatchedPath};
use serde_json::{Value, json};
use std::{env, error::Error, sync::Arc};

use crate::AppState;

#[axum_macros::debug_handler]
pub async fn get_db(State(state): State<Arc<Value>>) -> Json<Value> {
    let contents = (*state).clone();
    Json(contents)
}

#[axum_macros::debug_handler]
pub async fn get_static(State(state): State<Arc<Value>>) -> Json<Value> {
    let contents = (*state).clone();
    Json(contents)
}


#[axum_macros::debug_handler]
pub async fn post_json(
    Path(collection_name): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    println!("{}", collection_name);
    let mut parsed_json = state.json_dict.as_ref().lock().unwrap();
    parsed_json.insert(String::from("a"), payload.clone());

    Json(payload)
}

#[axum_macros::debug_handler]
pub async fn put_json(
    Path((collection_name, id)): Path<(String, String)>,
    State(state): State<AppState>,
    Json(payload): Json<Value>,
)  {
    assert!(payload.is_object());

    let state_val = { 
        let mut parsed_json = state.json_dict.lock().unwrap();
        let collection_json = parsed_json[&collection_name].as_array_mut().unwrap();
        
        let id = id.parse::<i32>().unwrap();
        let mut payload = payload;
        println!("{:?}", payload);
        let payload = payload.as_object_mut().unwrap();
        payload.insert(String::from("id"), json!(id));
        let payload: Value = payload.clone().into();

        collection_json.push(payload);
        Value::from(parsed_json.clone())
    };

    tokio::fs::write(state.json_path.clone(), state_val.to_string()).await.unwrap();
}

#[axum_macros::debug_handler]
pub async fn post_nested_json(
    Path((collection_name, id, nested_name)): Path<(String, String, String)>,
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) {
    let id = id.parse::<i32>().unwrap();
    let mut payload = payload;
    let payload = payload.as_array_mut().unwrap();
    let parent_id = json!({ format!("{}id", collection_name).as_str(): 2 });
    let nested_id = json!({ String::from("id"): 2 });
    payload.append(&mut vec![parent_id]);
    payload.append(&mut vec![nested_id]);
    let payload: Value = payload.clone().into();
    let mut parsed_json = state.json_dict.as_ref().lock().unwrap();
    parsed_json.insert(nested_name, payload.clone());

    print!("{}", Value::from(parsed_json.clone()));
}
