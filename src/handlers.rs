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
) -> Json<Value> {
    println!("{}, {}", collection_name, id);
    let mut parsed_json = state.json_dict.as_ref().lock().unwrap();
    parsed_json.insert(String::from("a"), payload.clone());

    Json(payload)
}

#[axum_macros::debug_handler]
pub async fn post_nested_json(
    Path((collection_name, id, nested_name)): Path<(String, String, String)>,
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let mut payload = payload.clone();
    let mut payload = payload.as_array_mut().unwrap();
    let parentId = json!({format!("{}-{}", nested_name, id).as_str(): 13});
    payload.append(&mut vec![parentId]);
    
    let payload: Value = payload.clone().into();
    let mut parsed_json = state.json_dict.as_ref().lock().unwrap();
    parsed_json.insert(String::from("a"), payload.clone());

    Json(payload)
}
