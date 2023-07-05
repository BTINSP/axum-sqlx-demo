use axum::extract::{State, Path};

use crate::{common::{Responder, Errors, AppState}, model::User};
use axum_extra::extract::WithRejection;


//  Path: /user/get/:useranme   Method: GET
pub async fn get_user_by_username(
    State(app_state): State<AppState>,
    WithRejection(Path(username),_): WithRejection<Path<String>, Errors>
) -> Result<Responder<User>, Errors> {
    let result = User::get_user_by_username(app_state.pool, username).await;
    match result {
        Ok(data) => {Ok(Responder::success(data))},
        Err(err) => {Err(err)},
    }
}

//  Path: /user/all   Method: GET
pub async fn get_all_user(State(app_state): State<AppState>) -> Result<Responder<Vec<User>>, Errors> {
    let result = User::get_all_user(app_state.pool).await;
    match result {
        Ok(data) => {Ok(Responder::success(data))},
        Err(err) => {Err(err)},
    }
}