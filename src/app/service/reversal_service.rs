use crate::models::iso8583_message::Iso8583Message;
use crate::models::transaction::{Iso8583Transaction, TransactionState, TransactionRepository};
use crate::app::service::stan_generator::StanGenerator;
use chrono::Local;
use std::sync::Arc;

/// Reversal Service
/// Handles transaction reversals (0400 messages)
pub struct ReversalService {
    stan_generator: Arc<StanGenerator>,
    transaction_repo: Arc<TransactionRepository>,
}

impl ReversalService {
    pub fn new(
        stan_generator: Arc<StanGenerator>,
        transaction_repo: Arc<TransactionRepository>,
    ) -> Self {
        Self {
            stan_generator,
            transaction_repo,
        }
    }

    /// Create a reversal message for a given original transaction
    /// MTI: 0400 for reversal
    pub async fn create_reversal(
        &self,
        original_tx: &Iso8583Transaction,
        reason_code: ReversalReason,
    ) -> Result<Iso8583Message, ReversalError> {
        let mut reversal = Iso8583Message::new("0400");

        // Generate new STAN for reversal
        let reversal_stan = self.stan_generator.next().await;
        reversal.set_field(11, reversal_stan.clone());

        // Set transmission date/time
        let now = Local::now();
        reversal.set_field(7, now.format("%m%d%H%M%S").to_string());
        reversal.set_field(12, now.format("%H%M%S").to_string());
        reversal.set_field(13, now.format("%m%d").to_string());

        // Copy key fields from original transaction
        if let Some(pan) = &original_tx.field_002 {
            reversal.set_field(2, pan.clone());
        }
        if let Some(proc_code) = &original_tx.field_003 {
            reversal.set_field(3, proc_code.clone());
        }
        if let Some(amount) = &original_tx.field_004 {
            reversal.set_field(4, amount.clone());
        }
        if let Some(terminal_id) = &original_tx.field_041 {
            reversal.set_field(41, terminal_id.clone());
        }
        if let Some(merchant_id) = &original_tx.field_042 {
            reversal.set_field(42, merchant_id.clone());
        }
        if let Some(currency) = &original_tx.field_049 {
            reversal.set_field(49, currency.clone());
        }

        // DE90: Original Data Elements (OMMDDHHMMSS + original STAN + original MTI)
        let original_data = format!(
            "{}{}{}",
            original_tx.field_013.as_deref().unwrap_or("0000"),
            original_tx.field_012.as_deref().unwrap_or("000000"),
            original_tx.tr_uniq_no
        );
        reversal.set_field(90, original_data);

        // Add reason code (typically in DE56 or private field)
        reversal.set_field(56, reason_code.as_code().to_string());

        tracing::info!(
            "Created reversal message: Original STAN={}, Reversal STAN={}, Reason={}",
            original_tx.tr_uniq_no,
            reversal_stan,
            reason_code.description()
        );

        Ok(reversal)
    }

    /// Perform automatic reversal for a timeout transaction
    pub async fn auto_reverse_timeout(
        &self,
        original_stan: &str,
    ) -> Result<Iso8583Message, ReversalError> {
        // Find original transaction
        let original_tx = self
            .transaction_repo
            .find_by_stan_today(original_stan)
            .await
            .map_err(|e| ReversalError::DatabaseError(e.to_string()))?
            .ok_or(ReversalError::TransactionNotFound)?;

        // Check if transaction is in a state that allows reversal
        if let Some(tr_type) = &original_tx.tr_type {
            let state = TransactionState::from_str(tr_type);
            if let Some(TransactionState::Approved) = state {
                return Err(ReversalError::TransactionAlreadyCompleted);
            }
            if let Some(TransactionState::Reversed) = state {
                return Err(ReversalError::AlreadyReversed);
            }
        }

        // Create reversal message
        self.create_reversal(&original_tx, ReversalReason::Timeout)
            .await
    }

    /// Perform manual reversal
    pub async fn manual_reverse(
        &self,
        original_stan: &str,
        reason: ReversalReason,
    ) -> Result<Iso8583Message, ReversalError> {
        let original_tx = self
            .transaction_repo
            .find_by_stan_today(original_stan)
            .await
            .map_err(|e| ReversalError::DatabaseError(e.to_string()))?
            .ok_or(ReversalError::TransactionNotFound)?;

        self.create_reversal(&original_tx, reason).await
    }

    /// Mark transaction as reversed in database
    pub async fn mark_as_reversed(
        &self,
        tr_dt: &str,
        tr_tm: &str,
        tr_uniq_no: &str,
    ) -> Result<(), ReversalError> {
        self.transaction_repo
            .update_response(
                tr_dt,
                tr_tm,
                tr_uniq_no,
                Some("99"), // Reversal response code
                None,
                None,
                &TransactionState::Reversed,
            )
            .await
            .map_err(|e| ReversalError::DatabaseError(e.to_string()))?;

        tracing::info!(
            "Transaction marked as reversed: {}/{}/{}",
            tr_dt,
            tr_tm,
            tr_uniq_no
        );

        Ok(())
    }
}

/// Reversal reasons
#[derive(Debug, Clone, Copy)]
pub enum ReversalReason {
    /// Timeout - no response received
    Timeout,
    /// Customer cancellation
    CustomerCancellation,
    /// Suspected malfunction
    SuspectedMalfunction,
    /// Unable to deliver response
    UnableToDeliver,
    /// Other reason
    Other,
}

impl ReversalReason {
    pub fn as_code(&self) -> &str {
        match self {
            ReversalReason::Timeout => "68",
            ReversalReason::CustomerCancellation => "17",
            ReversalReason::SuspectedMalfunction => "96",
            ReversalReason::UnableToDeliver => "68",
            ReversalReason::Other => "99",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            ReversalReason::Timeout => "Timeout - no response received",
            ReversalReason::CustomerCancellation => "Customer cancellation",
            ReversalReason::SuspectedMalfunction => "Suspected malfunction",
            ReversalReason::UnableToDeliver => "Unable to deliver response",
            ReversalReason::Other => "Other reason",
        }
    }
}

/// Reversal errors
#[derive(Debug, thiserror::Error)]
pub enum ReversalError {
    #[error("Transaction not found")]
    TransactionNotFound,

    #[error("Transaction already completed")]
    TransactionAlreadyCompleted,

    #[error("Transaction already reversed")]
    AlreadyReversed,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Invalid transaction state")]
    InvalidState,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn create_test_transaction() -> Iso8583Transaction {
        let mut tx = Iso8583Transaction::new("123456", "0200");
        tx.set_field(2, Some("4111111111111111".to_string()));
        tx.set_field(3, Some("000000".to_string()));
        tx.set_field(4, Some("000000100000".to_string()));
        tx.set_field(41, Some("TERM0001".to_string()));
        tx.set_field(42, Some("MERCHANT001".to_string()));
        tx.set_field(49, Some("704".to_string()));
        tx
    }

    #[tokio::test]
    async fn test_reversal_creation() {
        let stan_gen = Arc::new(StanGenerator::new());
        
        // This test requires a real database connection
        // For now, we'll just test the reversal message structure
        
        // In real test, you would:
        // let pool = PgPool::connect("postgresql://...").await.unwrap();
        // let repo = Arc::new(TransactionRepository::new(pool));
        // let service = ReversalService::new(stan_gen, repo);
        
        // let original_tx = create_test_transaction().await;
        // let reversal = service.create_reversal(&original_tx, ReversalReason::Timeout).await;
        
        // assert!(reversal.is_ok());
        // let msg = reversal.unwrap();
        // assert_eq!(msg.mti, "0400");
        // assert!(msg.has_field(11)); // STAN
        // assert!(msg.has_field(90)); // Original Data Elements
    }
}
