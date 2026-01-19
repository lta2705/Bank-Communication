use crate::repository::qr_transaction_repository::QrTransactionRepository;
use crate::{
    app::error::AppError,
    app::utils::kafka_message_sender::KafkaMessageSender,
    dto::{qr_req_dto::QrReqDto, qr_resp_dto::QrRespDto},
    models::{payos_qr_req::PayOsQrReq, payos_qr_resp::PayOsPaymentResponse},
};
use hmac::{Hmac, Mac};
use rdkafka::producer::FutureProducer;
use reqwest::Client;
use sha2::Sha256;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use tracing::{error, info, instrument};

// 1. Tách Config ra struct riêng để dễ quản lý và Dependency Injection
#[derive(Clone)]
pub struct PayOsConfig {
    pub api_key: String,
    pub client_id: String,
    pub return_url: String,
    pub checksum_key: String,
    pub payment_url: String,
}

impl PayOsConfig {
    pub fn from_env() -> Self {
        Self {
            api_key: env::var("X_API_KEY").expect("X_API_KEY must be set"),
            client_id: env::var("X_CLIENT_ID").expect("X_CLIENT_ID must be set"),
            return_url: env::var("RETURN_URL").expect("RETURN_URL must be set"),
            checksum_key: env::var("CHECKSUM_KEY").expect("CHECKSUM_KEY must be set"),
            payment_url: env::var("PAYMENT_URL").expect("PAYMENT_URL must be set"),
        }
    }
}

pub struct PayOsQrService {
    client: Client,
    config: Arc<PayOsConfig>, // Dùng Arc để share config nhẹ nhàng hơn
    qr_transaction_repository: QrTransactionRepository,
    kafka_sender: Arc<KafkaMessageSender>,
}

impl PayOsQrService {
    // Inject Config vào thay vì load lại mỗi lần new
    pub fn new(
        pg_pool: PgPool,
        config: Arc<PayOsConfig>,
        kafka_producer: Arc<FutureProducer>,
    ) -> Self {
        PayOsQrService {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to build reqwest client"),
            config,
            qr_transaction_repository: QrTransactionRepository::new(pg_pool),
            kafka_sender: Arc::new(KafkaMessageSender::new(kafka_producer)),
        }
    }

    #[instrument(skip(self, payload))] // Auto log function entry/exit với tracing
    pub async fn create_qr(&self, payload: QrReqDto) -> Result<QrRespDto, AppError> {
        // Validate input

        info!("Validating payload");
        payload
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // Parse order_code (Sử dụng i64 an toàn hơn i32 cho payment ID)
        let order_code: i64 = payload
            .transaction_id
            .parse()
            .map_err(|_| AppError::Validation("transaction_id must be numeric for PayOS".into()))?;

        info!("Checking for duplicate transaction ID {}", order_code);

        if self
            .qr_transaction_repository
            .find_by_order_code(order_code as i32)
            .await?
        {
            return Err(AppError::Validation("Transaction ID already exists".into()));
        }

        info!("Transaction ID is unique, proceeding...");

        let description = "DON HANG MOI";

        // Create signature
        let signature = create_signature(
            payload.amount,
            &self.config.return_url, // Cancel URL tạm dùng return_url
            description,
            order_code,
            &self.config.return_url,
            &self.config.checksum_key,
        )
        .map_err(AppError::Config)?;
        info!("Signature created successfully {}", signature.clone());

        let model = PayOsQrReq {
            order_code: order_code as i32, // Lưu ý type casting tùy DB
            amount: payload.amount,
            description: description.to_string(),
            return_url: self.config.return_url.clone(),
            cancel_url: self.config.return_url.clone(),
            signature: signature,
            ..Default::default()
        };

        info!("Prepared PayOsQrReq model: {:?}", model);
        // DB Insert (Write-Ahead Log)
        self.qr_transaction_repository
            .insert(model.clone())
            .await
            .map_err(AppError::Database)?;

        info!(
            order_code = model.order_code,
            amount = model.amount,
            "Sending request to PayOS"
        );

        // Send Request using .json() helper (Tự động set Content-Type và Serialize)
        let resp = self
            .client
            .post(&self.config.payment_url)
            .header("x-client-id", &self.config.client_id)
            .header("x-api-key", &self.config.api_key)
            .json(&model) // Reqwest tự động serialize body
            .send()
            .await
            .map_err(AppError::Http)?;

        info!("Received response from PayOS, processing...");
        self.handle_payos_response(resp, payload.pc_pos_id).await
    }

    pub async fn cancel_qr(&self, payload: QrReqDto) -> Result<QrRespDto, AppError> {
        let description = "Change my mind"; // Hoặc lấy từ payload nếu có lý do
        // Logic tạo URL clean hơn
        // Giả sử payment_url là "https://api.payos.vn/v2/payment-requests"
        // Target: ".../payment-requests/{id}/cancel"
        let cancel_url = format!(
            "{}/{}/cancel",
            self.config.payment_url, payload.transaction_id
        );

        let resp = self
            .client
            .post(&cancel_url)
            .header("x-client-id", &self.config.client_id)
            .header("x-api-key", &self.config.api_key)
            .json(&serde_json::json!({ "cancellationReason": description })) // Gợi ý: PayOS thường nhận JSON object
            .send()
            .await
            .map_err(AppError::Http)?;

        self.handle_payos_response(resp, payload.pc_pos_id).await
    }

    // Tách logic xử lý response để tái sử dụng cho cả Create và Cancel
    async fn handle_payos_response(
        &self,
        resp: reqwest::Response,
        pc_pos_id: String,
    ) -> Result<QrRespDto, AppError> {
        let status = resp.status();
        let body_text = resp.text().await.map_err(AppError::Http)?;

        if !status.is_success() {
            error!(status = %status, body = %body_text, "PayOS returned HTTP error");
            return Err(AppError::ExternalService(body_text));
        }

        let payos_resp: PayOsPaymentResponse = serde_json::from_str(&body_text).map_err(|e| {
            error!("Serde Parse Error: {} | Raw Body: {}", e, body_text);
            AppError::Config(format!("Parse PayOS JSON failed: {}", e))
        })?;

        if payos_resp.code != "00" {
            error!(code = %payos_resp.code, desc = %payos_resp.desc, "PayOS Business Error");
            return Err(AppError::ExternalService(format!(
                "PayOS Error [{}]: {}",
                payos_resp.code, payos_resp.desc
            )));
        }

        // Lấy data an toàn
        let success_data = payos_resp.data.ok_or_else(|| {
            AppError::ExternalService("PayOS success but data is null".to_string())
        })?;

        let qr_resp_dto = QrRespDto {
            response_code: payos_resp.code.clone(),
            qr_code: success_data.qr_code.clone(),
            transaction_id: success_data.order_code.to_string(),
            pc_pos_id: pc_pos_id.clone(),
            amount: success_data.amount,

        };

        // Send message to Kafka after successful processing
        info!("Sending PayOS QR response to Kafka...");
        if let Err(e) = self
            .kafka_sender
            .send(
                "payment_notifications",
                format!("QR_{}", qr_resp_dto.transaction_id).as_str(),
                &qr_resp_dto,
            )
            .await
        {
            error!("Failed to send PayOS response to Kafka: {}", e);
            // Don't fail the request if Kafka send fails, just log it
        }

        Ok(qr_resp_dto)
    }
}

// Helper function: Tối ưu types
fn create_signature(
    amount: i32,
    cancel_url: &str,
    description: &str,
    order_code: i64, // Dùng số thay vì string để clean hơn ở caller
    return_url: &str,
    checksum_key: &str,
) -> Result<String, String> {
    // Format string trực tiếp, không cần parse lại order_code
    let data = format!(
        "amount={}&cancelUrl={}&description={}&orderCode={}&returnUrl={}",
        amount, cancel_url, description, order_code, return_url
    );

    let mut mac = Hmac::<Sha256>::new_from_slice(checksum_key.as_bytes())
        .map_err(|_| "Invalid HMAC key".to_string())?;

    mac.update(data.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}
