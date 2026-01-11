use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;
use tracing::{error, info};

use reqwest::Client;
use serde_json;

use crate::{
    app::error::AppError,
    dto::{qr_req_dto::QrReqDto, qr_resp_dto::QrRespDto},
    models::{payos_qr_req::PayOsQrReq, payos_qr_resp::PayOsPaymentResponse},
};

pub struct PayOsQrService {
    client: Client,
    api_key: String,
    client_id: String,
    return_url: String,
    checksum_key: String,
}

impl PayOsQrService {
    pub fn new() -> Self {
        PayOsQrService {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to build reqwest client"),
            api_key: env::var("X_API_KEY").expect("X_API_KEY must be set"),
            client_id: env::var("X_CLIENT_ID").expect("X_CLIENT_ID must be set"),
            return_url: env::var("RETURN_URL").expect("RETURN_URL must be set"),
            checksum_key: env::var("CHECKSUM_KEY").expect("CHECKSUM_KEY must be set"),
        }
    }

    pub async fn create_qr(&self, payload: QrReqDto) -> Result<QrRespDto, AppError> {
        // 1. Validate input
        payload
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        info!(
            transaction_id = %payload.transaction_id,
            amount = payload.amount,
            "Mapping essential fields"
        );

        // 2. Parse orderCode ONCE (không random, không parse lại)
        let order_code: i32 = payload
            .transaction_id
            .parse()
            .map_err(|_| AppError::Validation("transaction_id must be numeric".into()))?;

        let amount = payload.amount;
        let description = "DON HANG MOI";
        let return_url = self.return_url.as_str();
        let cancel_url = self.return_url.as_str();
        // let mut expired_at = chrono::Utc::now().timestamp() + 15 * 60; // 15 phút

        // 3. Create signature (GIỐNG 100% payload gửi đi)
        let signature = create_signature(
            amount,
            cancel_url,
            description,
            // expired_at,
            &order_code.to_string(),
            return_url,
            &self.checksum_key,
        )
        .map_err(AppError::Config)?;

        // 4. Build PayOS request model
        let mut model = PayOsQrReq::new();
        model.order_code = order_code;
        model.amount = amount;
        model.description = description.to_string();
        // model.expired_at = Some(expired_at as u64);
        model.return_url = return_url.to_string();
        model.cancel_url = cancel_url.to_string();
        model.signature = signature;

        info!(
            order_code = model.order_code,
            amount = model.amount,
            "Sending request to PayOS"
        );

        // 5. Send HTTPS request (KHÔNG serialize thủ công)
        let body_json =
            serde_json::to_string(&model).map_err(|e| AppError::Config(e.to_string()))?;

        info!("Body created {:?}", body_json);
        let resp = self
            .client
            .post("https://api-merchant.payos.vn/v2/payment-requests")
            .header("x-client-id", self.client_id.as_str())
            .header("x-api-key", self.api_key.as_str())
            .header("content-type", "application/json")
            .body(body_json)
            .send()
            .await
            .map_err(AppError::Http)?;

        let status = resp.status();
        info!("Received HTTP response code {}", status);
        let body = resp.text().await.map_err(AppError::Http)?;

        info!("========================================");
        info!("PayOS HTTP Status: {}", status);
        info!("PayOS Raw Body: {}", body);
        info!("========================================");

        // 6. Handle PayOS error explicitly
        if !status.is_success() {
            error!(
                status = %status,
                body = %body,
                "PayOS returned error"
            );
            return Err(AppError::ExternalService(body));
        }

        // 7. Parse success response
        let payos_resp: PayOsPaymentResponse = match serde_json::from_str(&body) {
            Ok(v) => v,
            Err(e) => {
                error!("Serde Parse Error: {} | Raw Body: {}", e, body);
                return Err(AppError::Config(format!("Parse PayOS JSON failed: {}", e)));
            }
        };

        if payos_resp.code != "00" {
            error!(
                code = %payos_resp.code,
                desc = %payos_resp.desc,
                "PayOS returned business error"
            );
            return Err(AppError::ExternalService(format!(
                "PayOS Error [{}]: {}",
                payos_resp.code, payos_resp.desc
            )));
        }

        let success_data = payos_resp.data.ok_or_else(|| {
            AppError::ExternalService("PayOS returned success code but data is null".to_string())
        })?;
        let response_dto = QrRespDto {
            response_code: payos_resp.code,

            qr_code: success_data.qr_code,

            transaction_id: success_data.order_code.to_string(),

            pc_pos_id: payload.pc_pos_id.clone(),
        };

        info!("Mapped payload success: {:?}", response_dto);
        Ok(response_dto)
    }
}

/// Create HMAC-SHA256 signature according to PayOS spec
fn create_signature(
    amount: i32,
    cancel_url: &str,
    description: &str,
    // expired_at: i64,
    order_code: &str,
    return_url: &str,
    checksum_key: &str,
) -> Result<String, String> {
    // IMPORTANT: thứ tự & key phải đúng tuyệt đối theo PayOS
    let data = format!(
        "amount={}&cancelUrl={}&description={}&orderCode={}&returnUrl={}",
        amount, cancel_url, description, order_code, return_url
    );

    let mut mac =
        Hmac::<Sha256>::new_from_slice(checksum_key.as_bytes()).map_err(|e| e.to_string())?;

    mac.update(data.as_bytes());

    let result = mac.finalize();
    Ok(hex::encode(result.into_bytes()))
}
