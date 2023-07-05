use axum::{Router, routing::get, http::{Uri, StatusCode}};
use axum::middleware;

use crate::{common::{AppState, Responder}, handle, middleware::authenticator};




pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .nest("/user", Router::new()
            .route("/all", get(handle::get_all_user)) 
            .route("/get/:username", get(handle::get_user_by_username))
            .layer(middleware::from_fn(authenticator))
        )
    
    .with_state(app_state)
    .fallback(fallback)
           
}

async fn index() -> Responder<String> {
    Responder::new(StatusCode::OK, "index".to_string(), "".to_string())
}

async fn fallback(uri: Uri) -> Responder<String> {
    Responder::new(StatusCode::NOT_FOUND, format!("Not Found: {}", uri), "".to_string())
}