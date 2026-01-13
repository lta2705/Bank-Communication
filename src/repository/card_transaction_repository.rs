use crate::models::transaction::{Iso8583Transaction, TransactionState};
use chrono::Local;
use sqlx::PgPool;

/// Transaction Repository for database operations
pub struct CardTransactionRepository {
    pub pool: PgPool,
}

impl CardTransactionRepository {
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
        tr_uniq_no: &Option<String>,
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
    
    pub async fn find_by_transaction_id_and_trm_id(&self,transaction_id: String, trm_id: String) 
    -> Result<Option<Iso8583Transaction>, sqlx::Error> {
        let result = sqlx::query_as::<_, Iso8583Transaction>(
            r#"SELECT * FROM iso8583_payment
            WHERE tr_uniq_no = $1 AND trm_id = $2
            "#,
        )
        .bind(transaction_id)
        .bind(trm_id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(result)
    }
}
