use serde_json;
use std::io;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use crate::app::service::tlv_parser::{ParsedEmvData, TlvParseError};
use crate::app::service::iso8583_transaction_service::Iso8583TransactionService;
use crate::app::service::stan_generator::StanGenerator;
use crate::models::card_request::CardRequest;
use crate::models::transaction::TransactionRepository;
use sqlx::PgPool;

// Global service instances (to be initialized in builder)
lazy_static::lazy_static! {
    static ref TRANSACTION_SERVICE: tokio::sync::OnceCell<Arc<Iso8583TransactionService>> = 
        tokio::sync::OnceCell::new();
}

/// Initialize the transaction service (call this from builder)
pub async fn init_service(db_pool: Arc<PgPool>) {
    let stan_generator = Arc::new(StanGenerator::new());
    let transaction_repo = Arc::new(TransactionRepository::new((*db_pool).clone()));
    let service = Arc::new(Iso8583TransactionService::new(
        stan_generator,
        transaction_repo,
    ));
    
    let _ = TRANSACTION_SERVICE.set(service);
    info!("ISO8583 Transaction Service initialized");
}

/// Handle incoming TCP message from terminal
/// Parse JSON, process as ISO8583 transaction, return response
pub async fn handle_message(raw_msg: &str) -> io::Result<Vec<u8>> {
    info!("Received raw message: {:?}", raw_msg);

    // Step 1: Parse the main JSON message
    let card_request: CardRequest = serde_json::from_str(raw_msg).map_err(|e| {
        error!("Failed to parse JSON message: {}", e);
        io::Error::new(io::ErrorKind::InvalidData, format!("Invalid JSON: {}", e))
    })?;

    info!(
        "Parsed CardRequest - msgType: {}, trmId: {}, transactionId: {}, amount: {}",
        card_request.msg_type,
        card_request.trm_id,
        card_request.transaction_id,
        card_request.amount
    );

    // Step 2: Get transaction service
    let service = TRANSACTION_SERVICE
        .get()
        .ok_or_else(|| {
            error!("Transaction service not initialized");
            io::Error::new(io::ErrorKind::Other, "Service not initialized")
        })?;

    // Step 3: Process transaction through complete ISO8583 flow
    let response_json = service
        .process_transaction(&card_request)
        .await?;

    // Step 4: Parse EMV data for logging (optional)
    if let Ok(Some(de55_hex)) = card_request.get_de55() {
        if let Ok(parsed) = ParsedEmvData::from_de55(&de55_hex) {
            if let Some(pan) = parsed.get_pan() {
                info!("PAN: {}", mask_pan(&pan));
            }
            if let Some(aid) = parsed.get_aid() {
                info!("AID: {}", aid);
            }
        }
    }

    // Step 5: Return response as JSON bytes
    let response_str = serde_json::to_string(&response_json)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(response_str.into_bytes())
}

/// Mask PAN for logging (show first 6 and last 4 digits)
fn mask_pan(pan: &str) -> String {
    if pan.len() <= 10 {
        return "*".repeat(pan.len());
    }
    let first = &pan[..6];
    let last = &pan[pan.len() - 4..];
    format!("{}{}{}", first, "*".repeat(pan.len() - 10), last)
}
