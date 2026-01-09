use actix_web::{
    App, HttpResponse, error, get,
    http::{StatusCode, header::ContentType},
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

// Convert internal AppError to ControllerError so handlers can `map_err(ControllerError::from)`
impl From<crate::app::error::AppError> for ControllerError {
    fn from(_: crate::app::error::AppError) -> Self {
        // For now map all internal errors to InternalError.
        // Extend this mapping if you want finer-grained HTTP responses per AppError variant.
        ControllerError::InternalError
    }
}
