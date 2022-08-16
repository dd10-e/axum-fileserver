use axum::{
    body::{StreamBody, Body},
    http::{header, StatusCode, Request, Response},
    extract::{self, ContentLengthLimit, Multipart},
    response::{Headers, Html, IntoResponse},
    routing::{get, post, get_service},
    Json, Router
};
use uuid::Uuid;
use time::{Instant, OffsetDateTime};
use serde::{Deserialize, Serialize};

use crate::database::project;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Project {
    uuid: Uuid,
    number: i32,
    name: String,
    description: String,
    created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct SearchProjectResponse {
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct SearchProjectRequest {
    pub query: String,
}

pub async fn read(extract::Path(id): extract::Path<Uuid>) -> Result<Json<project::Model>, StatusCode>  {
    let project = project::find_by_uuid(id)
        .await
        .map_err(|err| {
            tracing::error!("failed to find project: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            tracing::info!("project not found for read");
            StatusCode::UNAUTHORIZED
        })?;

    Ok(Json(project))
}


#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ProjectCreateRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ProjectCreateResponse {
    pub success: bool,
}

pub async fn create(Json(req): Json<ProjectCreateRequest>) -> Result<Json<ProjectCreateResponse>, StatusCode> {
    project::create(&req.name, &req.description)
        .await
        .map(|_| Json(ProjectCreateResponse { success: true }))
        .map_err(|err| {
            tracing::error!("failed to create project: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}


#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ProjectPutRequest {
    pub name: String,
    pub description: String,
}

pub async fn update(Json(req): Json<ProjectPutRequest>, extract::Path(id): extract::Path<Uuid>) -> Result<Json<project::Model>, StatusCode>  {
    let project = 
        project::find_by_uuid(id)
            .await
            .map_err(|err| {
                tracing::error!("failed to update project: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok_or_else(|| {
                tracing::info!("project not found for update");
                StatusCode::UNAUTHORIZED
            })?;

    let project = project::update(project, &req.name, &req.description)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            tracing::error!("failed to search project, {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            tracing::info!("project not found for update");
            StatusCode::UNAUTHORIZED
        })?;

    Ok(Json(project))
}

pub async fn delete(extract::Path(id): extract::Path<Uuid>) -> impl IntoResponse  {
    let _project = project::delete_by_uuid(id);

    (StatusCode::OK, "Deleted".to_string())
}

pub async fn search(
    Json(req): Json<SearchProjectRequest>,
) -> Result<Json<SearchProjectResponse>, StatusCode> {
    project::search(&req.query)
        .await
        .map(|projects| {
            Json(SearchProjectResponse {
                projects: projects
                    .into_iter()
                    .map(|project| Project {
                        uuid: project.uuid,
                        number: project.number,
                        name: project.name,
                        description: project.description,
                        created_at: project.created_at.timestamp(),
                    })
                    .collect(),
            })
        })
        .map_err(|err| {
            tracing::error!("failed to search project, {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn list() -> Result<Json<SearchProjectResponse>, StatusCode>  {
    project::list()
        .await
        .map(|projects| {
            Json(SearchProjectResponse {
                projects: projects
                    .into_iter()
                    .map(|project| Project {
                        uuid: project.uuid,
                        number: project.number,
                        name: project.name,
                        description: project.description,
                        created_at: project.created_at.timestamp(),
                    })
                    .collect(),
            })
        })
        .map_err(|err| {
            println!("{:?}", err);
            tracing::error!("failed to search project, {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
