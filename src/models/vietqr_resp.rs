use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub qr_code: String,
    pub qr_data_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VietQrResp {
    pub code: String,
    pub desc: String,
    pub data: Data,
}
