use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QrRespDto {
    pub response_code: String,
    pub qr_code: String,
    pub transaction_id: String,
    pub pc_pos_id: String
}

impl QrRespDto {
    pub fn new() -> Self {
        return QrRespDto { 
            response_code: String::new(), 
            qr_code: String::new(), 
            transaction_id: String::new(), 
            pc_pos_id: String::new() 
        };
    }
    
    pub fn validate(&self) -> Result<(), &'static &str> {
        if self.qr_code.is_empty() {
            return Err(&"QR Code cannot be empty");
        }
        Ok(())
    }
}