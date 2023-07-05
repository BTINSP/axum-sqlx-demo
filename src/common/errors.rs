

use axum::{extract::rejection::{ FormRejection, JsonRejection, PathRejection, QueryRejection}, http::StatusCode, response::{IntoResponse, Response}};

#[derive(thiserror::Error, Debug)]
pub enum Errors {

    //  sqlx 错误
    #[error(transparent)]
    DataBase(#[from] sqlx::Error),

    //  axum 默认拒绝器 错误
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error(transparent)]
    QueryRejection(#[from] QueryRejection),

    #[error(transparent)]
    FormRejection(#[from] FormRejection),

    #[error(transparent)]
    PathRejection(#[from] PathRejection),

    //  其他错误消息
    #[error(transparent)]
    Other(#[from] anyhow::Error)
}


impl Errors {
    
    fn get_error(&self) -> (StatusCode, String) {
        match self {
            Errors::DataBase(err) => database_error(err),
            Errors::JsonRejection(_) => (StatusCode::BAD_REQUEST, "序列化失败".to_string()),
            Errors::QueryRejection(_) => (StatusCode::BAD_REQUEST, "请检查参数".to_string()),
            Errors::FormRejection(_) => (StatusCode::BAD_REQUEST, "请检查携带参数".to_string()),
            Errors::PathRejection(_) => (StatusCode::BAD_REQUEST, "请检查路径是否正确".to_string()),
            Errors::Other(_) => (StatusCode::BAD_REQUEST, "请联系管理员.".to_string()),
        }
    }

}

fn database_error(err: &sqlx::Error) -> (StatusCode, String) {
    match err {
        sqlx::Error::Configuration(_) => todo!(),
        sqlx::Error::Database(_) => todo!(),
        sqlx::Error::Io(_) => todo!(),
        sqlx::Error::Tls(_) => todo!(),
        sqlx::Error::Protocol(_) => todo!(),
        sqlx::Error::RowNotFound =>  (StatusCode::NOT_FOUND, "".to_string()),
        sqlx::Error::TypeNotFound { type_name } => todo!(),
        sqlx::Error::ColumnIndexOutOfBounds { index, len } => todo!(),
        sqlx::Error::ColumnNotFound(_) => (StatusCode::NOT_FOUND, "".to_string()),
        sqlx::Error::ColumnDecode { index, source } => todo!(),
        sqlx::Error::Decode(_) => todo!(),
        sqlx::Error::PoolTimedOut => todo!(),
        sqlx::Error::PoolClosed => todo!(),
        sqlx::Error::WorkerCrashed => todo!(),
        sqlx::Error::Migrate(_) => todo!(),
        _ => (StatusCode::BAD_REQUEST, "请联系管理员.".to_string()),
    }
}

impl IntoResponse for Errors {
    fn into_response(self) -> Response {
        self.get_error().into_response()
    }
}