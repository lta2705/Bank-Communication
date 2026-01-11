use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(dead_code)]
pub enum PaymentLinkStatus {
    Pending,
    Cancelled,
    Underpaid,
    Paid,
    Expired,
    Processing,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PayOsPaymentData {
    pub bin: String,
    pub account_number: String,
    pub account_name: String,
    pub currency: String,
    pub payment_link_id: String,
    pub amount: i64,
    pub description: String,
    pub order_code: i64,
    pub expired_at: Option<i64>,
    pub status: PaymentLinkStatus,
    pub checkout_url: String,
    pub qr_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PayOsPaymentResponse {
    pub code: String,
    pub desc: String,
    pub data: Option<PayOsPaymentData>,
    pub signature: String,
}
