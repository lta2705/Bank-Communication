use crate::{
    app::{handlers::handler_error::ControllerError, service::pay_os_service::PayOsQrService},
    dto::qr_req_dto::QrReqDto,
};
use actix_web::{HttpResponse, Responder, post, web};
use tracing::info;

#[post("/create_qr")]
pub async fn create_qr(
    req_body: web::Json<QrReqDto>,
    qr_service: web::Data<PayOsQrService>,
) -> Result<impl Responder, ControllerError> {
    let json = serde_json::to_string(&req_body).unwrap_or_else(|_| "<invalid json>".to_string());

    info!("QR raw payload: {}", json);

    let result = qr_service
        .create_qr(req_body.into_inner())
        .await
        .map_err(ControllerError::from)?;
    info!("Response payload from PayOS, {:?}", result);
    Ok(HttpResponse::Ok().json(result))
}

#[post("/cancel_qr")]
pub async fn cancel_qr(
    req_body: web::Json<QrReqDto>,
    qr_service: web::Data<PayOsQrService>,
) -> Result<impl Responder, ControllerError> {
    let _json = serde_json::to_string(&req_body).unwrap_or_else(|_| "<invalid json>".to_string());

    // info!("Cancel QR request payload", _json.clone());

    let result = qr_service
        .cancel_qr(req_body.into_inner())
        .await
        .map_err(ControllerError::from)?;

    info!("Response payload from PayOS, {:?}", result);
    Ok(HttpResponse::Ok().json(result))
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("PayOs QR Service up")
}
