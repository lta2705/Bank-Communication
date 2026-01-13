use sqlx::PgPool;

use crate::{app::error::AppError, models::{payos_qr_req::PayOsQrReq, payos_qr_resp::PayOsPaymentResponse}};
use crate::models::transaction::Iso8583Transaction;

pub struct QrTransactionRepository {
    pub pool: PgPool,
}

impl QrTransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, qr_txn: PayOsQrReq) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO iso8583_payment (
                field004, tr_uniq_no, field048, field064, field037
            )
            VALUES ($1, $2, $3, $4)
            "#,
        )
            .bind(qr_txn.amount)
            .bind(qr_txn.order_code)
            .bind(qr_txn.description)
            .bind(qr_txn.signature)
            .bind(qr_txn.order_code)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn find_by_order_code_and_trm_id(&self, order_code: i32, trm_id: &str) -> Result<Option<Iso8583Transaction>, AppError> {
        let result = sqlx::query_as::<_, Iso8583Transaction>(
            r#"
                SELECT * FROM iso8583_payment
                WHERE order_code = $1 AND trm_id = $2
"#, 
        )
    }
}
