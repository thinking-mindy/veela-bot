use axum::{extract::Json as RJson, response::Json};
use serde_json::Value;
use crate::minds::veela;

pub async fn hello_world() -> &'static str {"Access Denied"}

pub async fn veela(RJson(params):RJson<Value>)->Json<serde_json::Value>{
    let info=veela::start(params["qsn"].to_string()).await;
    Json(serde_json::json!({"rp":info}))
}
