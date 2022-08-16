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
pub struct Response {
    message: String,
}

pub async fn handler(extract::Path(uuid): extract::Path<Uuid>) -> Result<Json<Response>, ResponseError>  {
    user::delete(uuid).await.map_err(|err| {
        tracing::error!("failed to delete user: {}", err);
        ResponseError::App(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("Utilisateur inconnu"),
        )
    })?;

    let response  = Response {
        message: "OK".to_string(),
    };

    Ok(Json(response))
}