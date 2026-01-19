use actix_web::{post, web, HttpResponse, Responder};
use crate::models::payos_qr_resp::PayOsPaymentResponse;
use tracing::info;

#[post("/receive_qr")]
pub async fn receive_qr(
    req: web::Json<PayOsPaymentResponse>
) -> impl Responder {
    info!("Received PayOS QR payment notification: {:?}", req);
    
    // TODO: Process the PayOS payment response
    // This is a webhook handler for PayOS callbacks
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "received",
        "message": "Payment notification received successfully"
    }))
}