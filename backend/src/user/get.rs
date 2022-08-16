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
pub struct User {
    id: i32,
    uuid: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserResponse {
    first_name: String,
    last_name: String,
    email: String,
}

pub async fn get(extract::Path(uuid): extract::Path<Uuid>) -> Result<Json<user::Model>, StatusCode>  {
    let user = user::find_by_uuid(uuid)
        .await
        .map_err(|err| {
            tracing::error!("failed to find user: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            tracing::info!("user not found");
            StatusCode::UNAUTHORIZED
        })?;

    Ok(Json(user))
}