use loco_rs::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct RootResponse {
    pub name: String,
    pub version: String,
    pub endpoints: Vec<String>,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

/// GET /
pub async fn root_info() -> Result<Response> {
    format::json(RootResponse {
        name: "Task API".to_string(),
        version: "1.0".to_string(),
        endpoints: vec!["/tasks".to_string()],
    })
}

/// GET /health
pub async fn health() -> Result<Response> {
    format::json(HealthResponse {
        status: "ok".to_string(),
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(root_info))
        .add("/health", get(health))
}