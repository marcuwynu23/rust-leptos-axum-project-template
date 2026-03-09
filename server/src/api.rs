//! REST API routes mounted at /api. Add handlers and shared types here.

use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "leptos-axum-template",
    })
}

pub fn router() -> Router {
    Router::new().route("/health", get(health))
}
