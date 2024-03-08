use std::collections::HashMap;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::{fs_ops::JsonFileSearcher, AppState};

use super::RichText;

#[derive(Serialize, Deserialize)]
pub struct UpsertProps {
    uuid: Uuid,
    props: HashMap<String, serde_json::Value>,
}

#[axum::debug_handler]
pub async fn upsert_props(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(AppState { data_directory }): State<AppState>,
    Json(UpsertProps { uuid, props }): Json<UpsertProps>,
) -> (StatusCode, Json<HashMap<String, serde_json::Value>>) {
    let searcher = JsonFileSearcher::new();

    match searcher.search_and_replace_props(&data_directory, &uuid, &props) {
        Ok(_) => {}
        Err(err) => {
            info!("{:?}", err)
        }
    }
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(props))
}
