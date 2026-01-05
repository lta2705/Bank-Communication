use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct VietQrReq {
    pub account_no: String,
    pub account_name: String,
    pub acq_id: i32,
    pub amount: i32,
    pub add_info: String,
    pub format: String,
    pub template: String,
}
