use axum::{
    http::{StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ResponseError {
    App(StatusCode, Option<&'static str>),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        let response = match self {
            Self::App(status_code, error) => {
                let error_message = match error {
                    Some(message) => message,
                    None => "500: Internal servor error",
                };
                
                let body = Json(json!({
                    "error": error_message,
                }));

                (status_code, body).into_response()
            }
        };

        response
    }
}