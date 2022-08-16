use axum::http::StatusCode;
use axum::{
    extract::{self, ContentLengthLimit, Multipart},
    response::{Headers, Html, IntoResponse},
    Json
};
use sea_orm::ActiveModelTrait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ResponseError;
use crate::database::user;
use crate::auth::{generate_jwt, Claims};
use crate::idl::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UpdateUserRequest {
    uuid: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UpdateUserResponse {
    uuid: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}
pub async fn update(Json(req): Json<UpdateUserRequest>) -> Result<Json<UpdateUserResponse>, ResponseError>  {
    let user = user::find_by_uuid(req.uuid)
        .await
        .map_err(|err| {
            tracing::error!("failed to find user: {}", err);
            ResponseError::App(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some("Utilisateur inconnu"),
            )
        })?
        .ok_or_else(|| {
            tracing::info!("user not found");
            ResponseError::App(
                StatusCode::UNAUTHORIZED,
                Some("Utilisateur inconnu"),
            )
        })?;

    user::update(req.uuid, &req.first_name, &req.last_name, &req.email, &req.password).await.map_err(|err| {
        tracing::error!("failed to find user: {}", err);
        ResponseError::App(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("Utilisateur inconnu"),
        )
    })?;

    let user_response  = UpdateUserResponse {
        uuid: req.uuid,
        first_name: req.first_name.clone(),
        last_name: req.last_name.clone(),
        email: req.email.clone(),
        password: req.password.clone(),
    };

    Ok(Json(user_response))
}