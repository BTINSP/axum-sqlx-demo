use axum::{http::StatusCode, Json, response::{IntoResponse, Response}};
use serde::Serialize;


pub struct Responder<T> {
    status: StatusCode,
    responder_body: ResponderBody<T>
}

#[derive(Serialize, Clone, Debug)]
struct ResponderBody<T> {
    code: u16,
    message: String,
    data: Option<T>
}

impl <T> IntoResponse for Responder<T>
where T: Serialize
{
    fn into_response(self) -> Response {
        let body = Json(
            ResponderBody {
                code: self.responder_body.code,
                message: self.responder_body.message,
                data: self.responder_body.data,
            }
        );
        (self.status, body).into_response()
    }
}

impl<T> Responder<T> {

    pub fn new(status: StatusCode,message: String, data: T) -> Responder<T> {
        Responder {
            status,
            responder_body: ResponderBody {
                code: status.as_u16(),
                message: message.to_string(),
                data: Some(data)
            },
        }
    }

    #[warn(dead_code)]
    pub fn success(data: T) -> Responder<T> {
        Responder {
            status: StatusCode::OK, 
            responder_body: ResponderBody {
                code: StatusCode::OK.as_u16(),
                message: "success".to_string(),
                data: Some(data)
            }
        }
    }


}
