use chrono::Utc;
use sqlx::PgPool;

use crate::models::transaction::Iso8583Transaction;
use crate::{
    app::error::AppError,
    models::{payos_qr_req::PayOsQrReq, payos_qr_resp::PayOsPaymentResponse},
};

pub struct QrTransactionRepository {
    pub pool: PgPool,
}

impl QrTransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, qr_txn: PayOsQrReq) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        let tr_dt = now.format("%Y%m%d").to_string();
        let tr_tm = now.format("%H%M%S").to_string();
        sqlx::query(
            r#"
            INSERT INTO iso8583_payment (
               tr_dt, tr_tm, field_004, tr_uniq_no, field_048, field_064, field_037
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
            .bind(tr_dt)
            .bind(tr_tm)
            .bind(qr_txn.amount)
            .bind(qr_txn.order_code.to_string())
            .bind(qr_txn.description)
            .bind(qr_txn.signature)
            .bind(qr_txn.order_code)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn find_by_order_code(&self, order_code: i32) -> Result<bool, AppError> {
        let result: Option<(i32,)> = sqlx::query_as(
            r#"
                SELECT 1 FROM iso8583_payment
                WHERE tr_uniq_no = $1
                LIMIT 1
            "#,
        )
        .bind(order_code.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }
}
