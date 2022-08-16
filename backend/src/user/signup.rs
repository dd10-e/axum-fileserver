use axum::http::StatusCode;
use axum::{
    extract::{self, ContentLengthLimit, Multipart},
    response::{Headers, Html, IntoResponse},
    Json
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::user;
use crate::auth::{generate_jwt, Claims};
use crate::idl::*;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RegisterResponse {
    pub success: bool,
}

pub async fn register(
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    user::create(&req.first_name, &req.last_name, &req.email, &req.password)
        .await
        .map(|_| Json(RegisterResponse { success: true }))
        .map_err(|err| {
            tracing::error!("failed to create user: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}