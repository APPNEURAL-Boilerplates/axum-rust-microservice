use crate::{common::response, state::AppState};
use axum::{extract::State, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct HealthResponse {
    service: String,
    environment: String,
    status: &'static str,
    uptime_seconds: u64,
}

#[derive(Debug, Serialize)]
struct ReadyResponse {
    ready: bool,
    checks: Vec<ReadinessCheck>,
}

#[derive(Debug, Serialize)]
struct ReadinessCheck {
    name: &'static str,
    ok: bool,
}

pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    response::ok(HealthResponse {
        service: state.config.app_name.clone(),
        environment: state.config.app_env.clone(),
        status: "healthy",
        uptime_seconds: state.uptime_seconds(),
    })
}

pub async fn ready() -> impl IntoResponse {
    response::ok(ReadyResponse {
        ready: true,
        checks: vec![ReadinessCheck {
            name: "in_memory_repository",
            ok: true,
        }],
    })
}
