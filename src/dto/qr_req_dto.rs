use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QrReqDto {
    pub transaction_id: String,
    pub terminal_id: String,
    pub amount: i32,
    pub currency: String,
    pub transaction_type: String,
    pub pc_pos_id: String
}

impl QrReqDto {
    #[allow(dead_code)]
    pub fn new() -> Self {
        QrReqDto {
            transaction_id: String::new(),
            terminal_id: String::new(),
            amount: 0,
            currency: String::new(),
            transaction_type: String::new(),
            pc_pos_id: String::new()
        }
    }
    
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.amount <= 0 {
            return Err("Amount must be greater than 0");
        }
        Ok(())
    }
}