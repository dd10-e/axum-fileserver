mod post;
mod user;
mod error;
mod project;
pub mod document;
pub mod database;
pub mod idl;
pub mod auth;

use axum::{
    routing::{get, post},
    Router
};
use std::{net::SocketAddr};
use tower_http::{
    cors::{self, CorsLayer, Origin}
};

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;
type ResponseResult<T> = Result<T, error::ResponseError>;

#[tokio::main]
async fn main() -> DynResult<()> {
    dotenv::dotenv().ok();
    
    let env_var = std::env::var("ENV").expect("ENV not set");

    let is_production = if env_var == "production" {
        true
    } else {
        false
    };
    
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_multipart_form=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    // build our application with some routes
    let mut router = Router::new()
        .route("/fileserver/documents/:uuid/serve/*path", get(document::list_documents))
        .route("/fileserver/documents/:uuid/upload-file/*path", post(document::upload_file))
        .route("/fileserver/documents/:uuid/create-folder/*path", post(document::create_folder))
        .route("/fileserver/documents/:uuid/download-file/*path", get(document::download_file))
        .route("/fileserver/documents/:uuid/serve-file/*path", get(document::serve_file))
        .route("/fileserver/documents/:uuid/edit-file/*path", post(document::edit_file))
        .route("/fileserver/documents/:uuid/delete/*path", post(document::delete))

        .route("/projects", get(project::list).post(project::create))
        .route("/projects/:id", get(project::read).put(project::update).delete(project::delete))
        .route("/projects/search", post(project::search))

        .route("/users", get(user::list::list))
        // .route("/users/search", get(user::search::search))
        .route("/users/:id", get(user::get::get).post(user::update::update).delete(user::delete::handler))
        .route("/users/login", post(user::login::login))
        .route("/users/register", post(user::signup::register))

        .route("/test-protected-route", get(user::protected_test));



    if is_production {
        // ENV=production
        router = router
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                .allow_origin(Origin::list(vec![
                    std::env::var("SITE_FRONTEND_URL").expect("SITE_FRONTEND_URL not set").parse().unwrap(),
                ]))
                .allow_headers(cors::any())
                .allow_methods(cors::any())
            );
    } else {
        // ENV=dev
        router = router
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                .allow_origin(cors::any())
                .allow_headers(cors::any())
                .allow_methods(cors::any())
            );
    }



    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    tracing::debug!("listening on {}", addr);

    tokio::select! {
        _ = axum::Server::bind(&addr).serve(router.into_make_service()) => {}
        _ = tokio::signal::ctrl_c() => {}
    }

    tracing::info!("Finished serving.");

    Ok(())
}
