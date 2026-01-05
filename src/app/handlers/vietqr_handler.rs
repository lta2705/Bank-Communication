use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::app::handlers::handler_error::ControllerError;

#[post("/create_qr")]
async fn create_qr(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
async fn index() -> Result<&'static str, ControllerError> {
    Err(ControllerError::BadClientData)
}