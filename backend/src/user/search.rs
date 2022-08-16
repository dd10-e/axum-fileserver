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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct SearchUserRequest {
    pub query: String,
}

// pub struct SearchUserResponse {
//     pub projects: Vec<User>,
// }

// pub async fn search(
//     Json(req): Json<SearchUserRequest>,
// ) -> Result<Json<SearchProjectResponse>, StatusCode> {
//     project::search(&req.query)
//         .await
//         .map(|projects| {
//             Json(SearchProjectResponse {
//                 projects: projects
//                     .into_iter()
//                     .map(|project| Project {
//                         uuid: project.uuid,
//                         number: project.number,
//                         name: project.name,
//                         description: project.description,
//                         created_at: project.created_at.timestamp(),
//                     })
//                     .collect(),
//             })
//         })
//         .map_err(|err| {
//             tracing::error!("failed to search project, {}", err);
//             StatusCode::INTERNAL_SERVER_ERROR
//         })
// }