use axum::http::StatusCode;
use axum::{
    extract::{self, ContentLengthLimit, Multipart},
    response::{Headers, Html, IntoResponse},
    Json
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ResponseError;
use crate::database::user;
use crate::user::authentication::{generate_jwt, Claims};
use crate::idl::*;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct LoginResponse {
    pub success: bool,
    pub token: String,
}

pub async fn login(Json(req): Json<LoginRequest>) -> Result<Json<LoginResponse>, ResponseError> {
    println!("{:?}", req);

    let user = user::find_by_email(&req.email)
        .await
        .map_err(|err| {
            tracing::error!("failed to find user: {}", err);
            ResponseError::App(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some("Ces identifiants ne correspondent pas à nos enregistrements."),
            )
        })?
        .ok_or_else(|| {
            tracing::info!("user not found");
            ResponseError::App(
                StatusCode::UNAUTHORIZED,
                Some("Ces identifiants ne correspondent pas à nos enregistrements."),
            )
        })?;

    if user.password != req.password {
        tracing::info!("password mismatch");
        return Err(ResponseError::App(
            StatusCode::UNAUTHORIZED,
            Some("Ces identifiants ne correspondent pas à nos enregistrements."),
        ))
    }

    let jwt_token = generate_jwt(&Claims {
        uuid: user.uuid,
        email: user.email,
        exp: 10000000000,
    })
    .map_err(|err| {
        tracing::error!("failed to generate jwt token: {}", err);
        ResponseError::App(StatusCode::INTERNAL_SERVER_ERROR, None)
    })?;

    Ok(Json(LoginResponse {
        success: true,
        token: jwt_token,
    }))
}   
