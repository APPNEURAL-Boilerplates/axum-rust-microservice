use std::time::Duration;

use axum::{http::StatusCode, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer, request_id::MakeRequestUuid, timeout::TimeoutLayer, trace::TraceLayer,
    ServiceBuilderExt,
};

use crate::{
    common::error::{method_not_allowed, not_found},
    modules::{health, items, root},
    state::AppState,
};

pub fn build_router(state: AppState) -> Router {
    let request_timeout = Duration::from_secs(state.config.request_timeout_seconds);
    let middleware = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .layer(TraceLayer::new_for_http())
        .propagate_x_request_id()
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            request_timeout,
        ))
        .layer(CorsLayer::permissive());

    Router::new()
        .route("/", get(root::root).fallback(method_not_allowed))
        .nest("/api/v1", api_v1_router())
        .fallback(not_found)
        .layer(middleware)
        .with_state(state)
}

fn api_v1_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health).fallback(method_not_allowed))
        .route("/ready", get(health::ready).fallback(method_not_allowed))
        .merge(items::routes::router())
}
