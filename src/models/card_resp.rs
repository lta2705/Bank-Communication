use derive_more::derive;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CardResp {
    pub transaction_type: String,
    pub amount: String,
    pub status: String,
    pub message: String,
    pub transaction_id: String,
    pub tip_amt: u32,
    pub curr_cd: String,
    pub terminal_id: String,
    pub pc_pos_id: String,
}
