use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpResponse,
};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ControllerError {
    #[display("internal error")]
    InternalError,

    #[display("bad request")]
    BadClientData,

    #[display("timeout")]
    Timeout,
}

impl error::ResponseError for ControllerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ControllerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ControllerError::BadClientData => StatusCode::BAD_REQUEST,
            ControllerError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
