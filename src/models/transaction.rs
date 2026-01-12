use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Transaction State
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionState {
    Created,
    Sent,
    Approved,
    Declined,
    Timeout,
    Reversed,
    Voided,
    Failed,
}

impl TransactionState {
    pub fn as_str(&self) -> &str {
        match self {
            TransactionState::Created => "CREATED",
            TransactionState::Sent => "SENT",
            TransactionState::Approved => "APPROVED",
            TransactionState::Declined => "DECLINED",
            TransactionState::Timeout => "TIMEOUT",
            TransactionState::Reversed => "REVERSED",
            TransactionState::Voided => "VOIDED",
            TransactionState::Failed => "FAILED",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "CREATED" => Some(TransactionState::Created),
            "SENT" => Some(TransactionState::Sent),
            "APPROVED" => Some(TransactionState::Approved),
            "DECLINED" => Some(TransactionState::Declined),
            "TIMEOUT" => Some(TransactionState::Timeout),
            "REVERSED" => Some(TransactionState::Reversed),
            "VOIDED" => Some(TransactionState::Voided),
            "FAILED" => Some(TransactionState::Failed),
            _ => None,
        }
    }
}

/// ISO8583 Transaction Record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Iso8583Transaction {
    pub tr_dt: String,               // Transaction date YYYYMMDD
    pub tr_tm: String,               // Transaction time HHMMSS
    pub tr_uniq_no: Option<String>,  // Unique transaction number (STAN)
    pub trm_id: Option<String>,      // Terminal ID
    pub msg_typ: Option<String>,     // Message Type (MTI)
    pub inst_trm_id: Option<String>, // Institution Terminal ID
    pub inst_mer_no: Option<String>, // Institution Merchant Number

    // ISO8583 Fields (field_000 to field_128)
    pub field_000: Option<String>, // MTI
    pub field_001: Option<String>, // Bitmap (hex)
    pub field_002: Option<String>, // PAN
    pub field_003: Option<String>, // Processing Code
    pub field_004: Option<String>, // Amount
    pub field_007: Option<String>, // Transmission Date/Time
    pub field_011: Option<String>, // STAN
    pub field_012: Option<String>, // Time
    pub field_013: Option<String>, // Date
    pub field_014: Option<String>, // Expiration Date
    pub field_022: Option<String>, // POS Entry Mode
    pub field_023: Option<String>, // Card Sequence Number
    pub field_025: Option<String>, // POS Condition Code
    pub field_032: Option<String>, // Acquiring Institution ID
    pub field_035: Option<String>, // Track 2 Data
    pub field_037: Option<String>, // RRN
    pub field_038: Option<String>, // Authorization Code
    pub field_039: Option<String>, // Response Code
    pub field_041: Option<String>, // Terminal ID
    pub field_042: Option<String>, // Merchant ID
    pub field_043: Option<String>, // Merchant Name/Location
    pub field_049: Option<String>, // Currency Code
    pub field_052: Option<String>, // PIN Data
    pub field_054: Option<String>, // Additional Amounts
    pub field_055: Option<String>, // EMV Data
    pub field_060: Option<String>, // Reserved Private
    pub field_061: Option<String>, // Reserved Private
    pub field_062: Option<String>, // Reserved Private
    pub field_063: Option<String>, // Reserved Private
    pub field_064: Option<String>, // MAC
    pub field_070: Option<String>, // Network Management Code
    pub field_090: Option<String>, // Original Data Elements
    pub field_095: Option<String>, // Replacement Amounts
    pub field_102: Option<String>, // Account ID 1
    pub field_103: Option<String>, // Account ID 2
    pub field_123: Option<String>, // Reserved Private
    pub field_127: Option<String>, // Reserved Private
    pub field_128: Option<String>, // MAC 2

    pub inst_dtm: Option<String>, // Insert datetime
    pub updt_dtm: Option<String>, // Update datetime
    pub tr_type: Option<String>,  // Transaction type/state
}

impl Iso8583Transaction {
    /// Create a new transaction record
    pub fn new(stan: &str, mti: &str) -> Self {
        let now = Local::now();
        Self {
            tr_dt: now.format("%Y%m%d").to_string(),
            tr_tm: now.format("%H%M%S").to_string(),
            tr_uniq_no: None,
            trm_id: None,
            msg_typ: Some(mti.to_string()),
            inst_trm_id: None,
            inst_mer_no: None,
            field_000: Some(mti.to_string()),
            field_001: None,
            field_002: None,
            field_003: None,
            field_004: None,
            field_007: None,
            field_011: Some(stan.to_string()),
            field_012: Some(now.format("%H%M%S").to_string()),
            field_013: Some(now.format("%m%d").to_string()),
            field_014: None,
            field_022: None,
            field_023: None,
            field_025: None,
            field_032: None,
            field_035: None,
            field_037: None,
            field_038: None,
            field_039: None,
            field_041: None,
            field_042: None,
            field_043: None,
            field_049: None,
            field_052: None,
            field_054: None,
            field_055: None,
            field_060: None,
            field_061: None,
            field_062: None,
            field_063: None,
            field_064: None,
            field_070: None,
            field_090: None,
            field_095: None,
            field_102: None,
            field_103: None,
            field_123: None,
            field_127: None,
            field_128: None,
            inst_dtm: Some(now.format("%Y%m%d%H%M%S").to_string()),
            updt_dtm: None,
            tr_type: Some(TransactionState::Created.as_str().to_string()),
        }
    }

    /// Set field value by data element number
    pub fn set_field(&mut self, de: u8, value: Option<String>) {
        match de {
            0 => self.field_000 = value,
            1 => self.field_001 = value,
            2 => self.field_002 = value,
            3 => self.field_003 = value,
            4 => self.field_004 = value,
            7 => self.field_007 = value,
            11 => self.field_011 = value,
            12 => self.field_012 = value,
            13 => self.field_013 = value,
            14 => self.field_014 = value,
            22 => self.field_022 = value,
            23 => self.field_023 = value,
            25 => self.field_025 = value,
            32 => self.field_032 = value,
            35 => self.field_035 = value,
            37 => self.field_037 = value,
            38 => self.field_038 = value,
            39 => self.field_039 = value,
            41 => {
                self.field_041 = value.clone();
                self.trm_id = value;
            }
            42 => self.field_042 = value,
            43 => self.field_043 = value,
            49 => self.field_049 = value,
            52 => self.field_052 = value,
            54 => self.field_054 = value,
            55 => self.field_055 = value,
            60 => self.field_060 = value,
            61 => self.field_061 = value,
            62 => self.field_062 = value,
            63 => self.field_063 = value,
            64 => self.field_064 = value,
            70 => self.field_070 = value,
            90 => self.field_090 = value,
            95 => self.field_095 = value,
            102 => self.field_102 = value,
            103 => self.field_103 = value,
            123 => self.field_123 = value,
            127 => self.field_127 = value,
            128 => self.field_128 = value,
            _ => {}
        }
    }

    /// Get field value by data element number
    pub fn get_field(&self, de: u8) -> Option<&String> {
        match de {
            0 => self.field_000.as_ref(),
            1 => self.field_001.as_ref(),
            2 => self.field_002.as_ref(),
            3 => self.field_003.as_ref(),
            4 => self.field_004.as_ref(),
            7 => self.field_007.as_ref(),
            11 => self.field_011.as_ref(),
            12 => self.field_012.as_ref(),
            13 => self.field_013.as_ref(),
            14 => self.field_014.as_ref(),
            22 => self.field_022.as_ref(),
            23 => self.field_023.as_ref(),
            25 => self.field_025.as_ref(),
            32 => self.field_032.as_ref(),
            35 => self.field_035.as_ref(),
            37 => self.field_037.as_ref(),
            38 => self.field_038.as_ref(),
            39 => self.field_039.as_ref(),
            41 => self.field_041.as_ref(),
            42 => self.field_042.as_ref(),
            43 => self.field_043.as_ref(),
            49 => self.field_049.as_ref(),
            52 => self.field_052.as_ref(),
            54 => self.field_054.as_ref(),
            55 => self.field_055.as_ref(),
            60 => self.field_060.as_ref(),
            61 => self.field_061.as_ref(),
            62 => self.field_062.as_ref(),
            63 => self.field_063.as_ref(),
            64 => self.field_064.as_ref(),
            70 => self.field_070.as_ref(),
            90 => self.field_090.as_ref(),
            95 => self.field_095.as_ref(),
            102 => self.field_102.as_ref(),
            103 => self.field_103.as_ref(),
            123 => self.field_123.as_ref(),
            127 => self.field_127.as_ref(),
            128 => self.field_128.as_ref(),
            _ => None,
        }
    }
}
