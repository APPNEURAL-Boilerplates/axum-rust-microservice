use super::dto::CreateItemRequest;
use crate::{
    common::{
        error::{method_not_allowed, AppError},
        response,
    },
    state::AppState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/items",
            get(list_items)
                .post(create_item)
                .fallback(method_not_allowed),
        )
        .route("/items/{id}", get(get_item).fallback(method_not_allowed))
}

async fn list_items(State(state): State<AppState>) -> impl IntoResponse {
    response::ok(state.items.list().await)
}

async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    Ok(response::ok(state.items.get(id).await?))
}

async fn create_item(
    State(state): State<AppState>,
    payload: Result<Json<CreateItemRequest>, axum::extract::rejection::JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(payload) = payload.map_err(AppError::from_json_rejection)?;
    Ok(response::created(state.items.create(payload).await?))
}
