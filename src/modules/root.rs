use crate::{common::response, state::AppState};
use axum::{extract::State, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct RootResponse {
    service: String,
    environment: String,
    version: &'static str,
    uptime_seconds: u64,
}

pub async fn root(State(state): State<AppState>) -> impl IntoResponse {
    response::ok(RootResponse {
        service: state.config.app_name.clone(),
        environment: state.config.app_env.clone(),
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: state.uptime_seconds(),
    })
}
