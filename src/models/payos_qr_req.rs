use serde::{Deserialize, Serialize};

/// ----------------------
/// Item
/// ----------------------
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayOsItem {
    pub name: String,
    pub quantity: i32,
    pub price: i64, // VND
    pub unit: String,
    pub tax_percent: PayOsTaxPercent,
}

/// ----------------------
/// Tax percent (type-safe)
/// ----------------------
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(i8)]
pub enum PayOsTaxPercent {
    #[serde(rename = "-2")]
    NegativeTwo = -2,

    #[serde(rename = "-1")]
    NegativeOne = -1,

    #[serde(rename = "0")]
    Zero = 0,

    #[serde(rename = "5")]
    Five = 5,

    #[serde(rename = "8")]
    Eight = 8,

    #[serde(rename = "10")]
    Ten = 10,
}

/// ----------------------
/// QR Request
/// ----------------------
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayOsQrReq {
    pub order_code: i32,
    pub amount: i32, // Tổng tiền (VND)
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_company_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_tax_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PayOsItem>>,

    pub return_url: String,
    pub cancel_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    
    //Unix timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<u64>,

    pub signature: String,
}
impl PayOsQrReq {
    pub fn new() -> Self {
        PayOsQrReq {
            order_code: rand::random::<i32>().abs(),
            amount: 0,
            description: String::new(),
            buyer_name: None,
            buyer_email: None,
            buyer_company_name: None,
            buyer_phone: None,
            buyer_address: None,
            buyer_tax_code: None,
            items: None,
            return_url: String::new(),
            cancel_url: String::new(),
            invoice: None,
            expired_at: None,
            signature: String::new(),
        }
    }
}
