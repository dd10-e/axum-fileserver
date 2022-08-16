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
pub struct UserListResponse {
    users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    id: i32,
    uuid: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    created_at: i64,
}

pub async fn list() -> impl IntoResponse  {
    user::list()
    .await
    .map(|users| {
        Json(UserListResponse {
            users: users
                .into_iter()
                .map(|user| User {
                    id: user.id,
                    uuid: user.uuid,
                    first_name: user.first_name,
                    last_name: user.last_name,
                    email: user.email,
                    created_at: user.created_at.timestamp(),
                })
                .collect(),
        })
    })
    .map_err(|err| {
        tracing::error!("failed to search book, {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}