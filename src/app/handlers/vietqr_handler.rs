use std::sync::Arc;

use crate::{
    app::{handlers::handler_error::ControllerError, service::qr_transaction_service::VietQrService}, dto::vietqr_req_dto::VietQrReqDto,
    models::vietqr_req::VietQrReq,
};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[post("/create_qr")]
pub async fn create_qr(req_body: web::Json<VietQrReqDto>, qr_service: web::Data<Arc<VietQrService>>) -> impl Responder {
    let result = qr_service.create_qr(req_body.into_inner()).await;
    HttpResponse::Ok().json(result)
}

#[get("/")]
async fn index() -> Result<&'static str, ControllerError> {
    Err(ControllerError::BadClientData)
}
