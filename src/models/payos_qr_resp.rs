use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
pub struct PayOsPaymentData {
    pub bin: String,
    pub account_number: String,
    pub account_name: String,
    pub currency: String,
    pub payment_link_id: String,
    pub amount: i64,
    pub description: String,
    pub order_code: i64,
    pub expired_at: i64,
    pub status: PaymentLinkStatus,
    pub checkout_url: String,
    pub qr_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayOsPaymentResponse {
    pub code: String,
    pub desc: String,
    pub data: PayOsPaymentData,
    pub signature: String,
}
