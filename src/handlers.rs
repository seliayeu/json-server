use axum::{extract::Path, extract::State, routing::put, Json, Router};
use serde_json::Value;
use std::{env, error::Error, sync::Arc};

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