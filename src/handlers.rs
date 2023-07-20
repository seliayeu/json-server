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
    println!("{:?}", state.json_dict);
    let mut parsed_json = state.json_dict.lock().unwrap();
    let collection_json = parsed_json[&collection_name].as_array_mut().unwrap();

    let id = id.parse::<i32>().unwrap();
    let mut payload = payload;
    let payload = payload.as_array_mut().unwrap();
    let collection_id = json!({ String::from("id"): id.to_string() });
    payload.append(&mut vec![collection_id]);
    let mut payload: Value = payload.clone().into();

    collection_json.append(payload.as_array_mut().unwrap());

    println!("{}", Value::from(parsed_json.clone()));

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
