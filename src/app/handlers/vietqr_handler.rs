use std::sync::Arc;

use crate::{
    app::{
        handlers::handler_error::ControllerError, service::qr_transaction_service::VietQrService,
    },
    dto::vietqr_req_dto::VietQrReqDto,
};
use actix_web::{HttpResponse, Responder, get, post, web};

#[post("/create_qr")]
pub async fn create_qr(
    req_body: web::Json<VietQrReqDto>,
    qr_service: web::Data<Arc<VietQrService>>,
) -> Result<impl Responder, ControllerError> {
    // Service now returns Result<VietQrRespDto, AppError>; map errors to ControllerError
    let result = qr_service
        .create_qr(req_body.into_inner())
        .await
        .map_err(ControllerError::from)?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/")]
async fn index() -> Result<&'static str, ControllerError> {
    Ok("VietQR Service up")
}
