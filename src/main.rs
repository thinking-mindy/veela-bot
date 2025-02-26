use axum::{routing::{get, post}, Router};
use veela::routes;
use tower_http::cors::CorsLayer;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
    .route("/", get(routes::hello_world))
    .route("/veela", post(routes::veela))
    .layer(CorsLayer::permissive());

    Ok(router.into())
}
