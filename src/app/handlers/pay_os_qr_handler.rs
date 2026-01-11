use std::sync::Arc;

use crate::{
    app::{handlers::handler_error::ControllerError, service::pay_os_service::PayOsQrService}, dto::qr_req_dto::QrReqDto
};
use actix_web::{HttpResponse, Responder, get, post, web};
use tracing::info;

#[post("/create_qr")]
pub async fn create_qr(
    req_body: web::Json<QrReqDto>,
    qr_service: web::Data<Arc<PayOsQrService>>,
) -> Result<impl Responder, ControllerError> {
    // Obtain a reference to the inner service and call its async method
    let svc: &PayOsQrService = qr_service.get_ref().as_ref();
    info!("Creating PayOs QR transaction");
    let result = svc
        .create_qr(req_body.into_inner())
        .await
        .map_err(ControllerError::from)?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/")]
async fn index() -> Result<&'static str, ControllerError> {
    Ok("PayOs QR Service up")
}
