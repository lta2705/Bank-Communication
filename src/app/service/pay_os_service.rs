use hmac::{Hmac, Mac};
use sha2::Sha256;
use tracing::info;
use std::env;

use crate::{
    app::error::AppError,
    dto::{qr_req_dto::QrReqDto, qr_resp_dto::QrRespDto},
    models::payos_qr_req::PayOsQrReq,
};
use reqwest::Client;
use serde_json;

pub struct PayOsQrService {
    client: Client,
    api_key: String,
    client_id: String,
    return_url: String,
    checksum_key: String,
}

impl PayOsQrService {
    pub fn new() -> Self {
        return PayOsQrService {
            
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
            api_key: env::var("X_API_KEY").expect("X_API_KEY must be set"),
            client_id: env::var("X_CLIENT_ID").expect("X_CLIENT_ID must be set"),
            return_url: env::var("RETURN_URL").expect("RETURN_URL must be set"),
            checksum_key: env::var("CHECKSUM_KEY").expect("CHECKSUM_KEY must be set"),
        };
    }

    pub async fn create_qr(&self, payload: QrReqDto) -> Result<QrRespDto, AppError> {
        payload.validate().map_err(AppError::Validation)?;
        info!("Mapping essential fields");
        let amount = payload.amount.clone();
        let order_code = payload.transaction_id.clone();
        let cancel_url = self.return_url.clone();
        let description = "DON HANG MOI";
        let return_url = self.return_url.clone();
        let checksum_key = self.checksum_key.clone();

        let signature = create_signature(
            amount,
            cancel_url.as_str(),
            description,
            order_code.as_str(),
            return_url.as_str(),
            checksum_key.as_str(),
        )
        .map_err(AppError::Config)?;

        let mut model = PayOsQrReq::new();
        model.amount = amount;
        model.order_code = payload.transaction_id;
        model.description = description.to_string();
        model.cancel_url = self.return_url.clone();
        model.return_url = self.return_url.clone();
        model.signature = signature;

        // Serialize request body explicitly to avoid relying on reqwest `json` feature.
        let body =
            serde_json::to_string(&mut model).map_err(|e| AppError::Config(e.to_string()))?;
        
        info!("Creating QR request with payload {:?}", body);
        
        let resp = self
            .client
            .post("https://api-merchant.payos.vn/v2/payment-requests")
            .header("x-client-id", &self.client_id)
            .header("x-api-key", &self.api_key)
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await? // network error -> AppError
            .error_for_status()?; // HTTP 4xx/5xx -> AppError

        // Read response text and parse with serde_json (map serde errors into AppError::Config).
        let text = resp.text().await?;
        let parsed: QrRespDto =
            serde_json::from_str(&text).map_err(|e| AppError::Config(e.to_string()))?;

        Ok(parsed)
    }
}

fn create_signature(
    amount: i32,
    cancel_url: &str,
    description: &str,
    order_code: &str,
    return_url: &str,
    checksum_key: &str,
) -> Result<String, String> {
    let data = format!(
        "amount={}&cancelUrl={}&description={}&orderCode={}&returnUrl={}",
        amount, cancel_url, description, order_code, return_url
    );

    let mut mac =
        Hmac::<Sha256>::new_from_slice(checksum_key.as_bytes()).map_err(|e| e.to_string())?;

    mac.update(data.as_bytes());

    // 3. Convert sang hex string
    let result = mac.finalize();
    let signature_bytes = result.into_bytes();

    Ok(hex::encode(signature_bytes))
}
