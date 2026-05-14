use axum::{
    body::{to_bytes, Body},
    http::{header::CONTENT_TYPE, Method, Request, StatusCode},
};
use axum_microservice_boilerplate::{app::build_router, config::AppConfig, state::AppState};
use pretty_assertions::assert_eq;
use serde_json::{json, Value};
use tower::ServiceExt;

fn test_app() -> axum::Router {
    build_router(AppState::new(AppConfig::test()))
}

async fn json_body(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn root_returns_service_metadata() {
    let response = test_app()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["data"]["service"], "axum-microservice-test");
}

#[tokio::test]
async fn health_returns_healthy() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["data"]["status"], "healthy");
}

#[tokio::test]
async fn ready_returns_ready() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["data"]["ready"], true);
}

#[tokio::test]
async fn list_items_returns_empty_list() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .uri("/api/v1/items")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["data"], json!([]));
}

#[tokio::test]
async fn create_item_returns_created_item() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/items")
                .header(CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{"name":"Keyboard","description":"Mechanical","price":99.99}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = json_body(response).await;
    assert_eq!(body["data"]["name"], "Keyboard");
}

#[tokio::test]
async fn get_created_item_by_id() {
    let app = test_app();
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/items")
                .header(CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"name":"Mouse","price":25}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    let create_body = json_body(create_response).await;
    let id = create_body["data"]["id"].as_str().unwrap();
    let get_response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/items/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn invalid_json_returns_400() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/items")
                .header(CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"name":"broken""#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = json_body(response).await;
    assert_eq!(body["error"]["code"], "BAD_REQUEST");
}

#[tokio::test]
async fn validation_error_returns_400() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/items")
                .header(CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"name":"","price":-1}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .uri("/missing")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn unsupported_method_returns_405() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/api/v1/items")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn request_id_is_propagated() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .header("x-request-id", "test-request-id")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        response
            .headers()
            .get("x-request-id")
            .unwrap()
            .to_str()
            .unwrap(),
        "test-request-id"
    );
}
