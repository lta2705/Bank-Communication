use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::collections::HashMap;

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
    pub tr_uniq_no: String,          // Unique transaction number (STAN)
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
            tr_uniq_no: stan.to_string(),
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

/// Transaction Repository for database operations
pub struct TransactionRepository {
    pool: PgPool,
}

impl TransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Insert a new transaction
    pub async fn insert(&self, tx: &Iso8583Transaction) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO iso8583_payment (
                tr_dt, tr_tm, tr_uniq_no, trm_id, msg_typ,
                field_000, field_001, field_002, field_003, field_004,
                field_007, field_011, field_012, field_013, field_014,
                field_022, field_023, field_025, field_032, field_035,
                field_037, field_038, field_039, field_041, field_042,
                field_043, field_049, field_052, field_054, field_055,
                field_060, field_061, field_062, field_063, field_064,
                field_070, field_090, field_095, field_102, field_103,
                field_123, field_127, field_128,
                inst_dtm, tr_type
            )
            VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25,
                $26, $27, $28, $29, $30,
                $31, $32, $33, $34, $35,
                $36, $37, $38, $39, $40,
                $41, $42, $43,
                $44, $45
            )
            "#,
        )
        .bind(&tx.tr_dt)
        .bind(&tx.tr_tm)
        .bind(&tx.tr_uniq_no)
        .bind(&tx.trm_id)
        .bind(&tx.msg_typ)
        .bind(&tx.field_000)
        .bind(&tx.field_001)
        .bind(&tx.field_002)
        .bind(&tx.field_003)
        .bind(&tx.field_004)
        .bind(&tx.field_007)
        .bind(&tx.field_011)
        .bind(&tx.field_012)
        .bind(&tx.field_013)
        .bind(&tx.field_014)
        .bind(&tx.field_022)
        .bind(&tx.field_023)
        .bind(&tx.field_025)
        .bind(&tx.field_032)
        .bind(&tx.field_035)
        .bind(&tx.field_037)
        .bind(&tx.field_038)
        .bind(&tx.field_039)
        .bind(&tx.field_041)
        .bind(&tx.field_042)
        .bind(&tx.field_043)
        .bind(&tx.field_049)
        .bind(&tx.field_052)
        .bind(&tx.field_054)
        .bind(&tx.field_055)
        .bind(&tx.field_060)
        .bind(&tx.field_061)
        .bind(&tx.field_062)
        .bind(&tx.field_063)
        .bind(&tx.field_064)
        .bind(&tx.field_070)
        .bind(&tx.field_090)
        .bind(&tx.field_095)
        .bind(&tx.field_102)
        .bind(&tx.field_103)
        .bind(&tx.field_123)
        .bind(&tx.field_127)
        .bind(&tx.field_128)
        .bind(&tx.inst_dtm)
        .bind(&tx.tr_type)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update transaction with response data
    pub async fn update_response(
        &self,
        tr_dt: &str,
        tr_tm: &str,
        tr_uniq_no: &str,
        response_code: Option<&str>,
        auth_code: Option<&str>,
        rrn: Option<&str>,
        state: &TransactionState,
    ) -> Result<(), sqlx::Error> {
        let now = Local::now().format("%Y%m%d%H%M%S").to_string();

        sqlx::query(
            r#"
            UPDATE iso8583_payment
            SET field_037 = COALESCE($4, field_037),
                field_038 = COALESCE($5, field_038),
                field_039 = COALESCE($6, field_039),
                tr_type = $7,
                updt_dtm = $8
            WHERE tr_dt = $1 AND tr_tm = $2 AND tr_uniq_no = $3
            "#,
        )
        .bind(tr_dt)
        .bind(tr_tm)
        .bind(tr_uniq_no)
        .bind(rrn)
        .bind(auth_code)
        .bind(response_code)
        .bind(state.as_str())
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Find transaction by key
    pub async fn find_by_key(
        &self,
        tr_dt: &str,
        tr_tm: &str,
        tr_uniq_no: &str,
    ) -> Result<Option<Iso8583Transaction>, sqlx::Error> {
        let result = sqlx::query_as::<_, Iso8583Transaction>(
            r#"
            SELECT * FROM iso8583_payment
            WHERE tr_dt = $1 AND tr_tm = $2 AND tr_uniq_no = $3
            "#,
        )
        .bind(tr_dt)
        .bind(tr_tm)
        .bind(tr_uniq_no)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    /// Find transaction by STAN for today
    pub async fn find_by_stan_today(
        &self,
        stan: &str,
    ) -> Result<Option<Iso8583Transaction>, sqlx::Error> {
        let today = Local::now().format("%Y%m%d").to_string();

        let result = sqlx::query_as::<_, Iso8583Transaction>(
            r#"
            SELECT * FROM iso8583_payment
            WHERE tr_dt = $1 AND tr_uniq_no = $2
            ORDER BY tr_tm DESC
            LIMIT 1
            "#,
        )
        .bind(today)
        .bind(stan)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }
}
