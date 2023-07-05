use axum::{Router, routing::get, http::{Uri, StatusCode}};

use crate::{common::{AppState, Responder}, handle};




pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .nest("/user", Router::new()
            .route("/get/:username", get(handle::get_user_by_username))
            .route("/all", get(handle::get_all_user)) 
        )
    .with_state(app_state)
    .fallback(fallback)
           
}

async fn fallback(uri: Uri) -> Responder<String> {
    Responder::new(StatusCode::NOT_FOUND, format!("Not Found: {}", uri), "".to_string())
}