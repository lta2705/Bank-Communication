use serde_json;
use std::io;
use tracing::{debug, error, info, warn};

use crate::app::service::tlv_parser::{ParsedEmvData, TlvParseError};
use crate::models::card_request::CardRequest;

/// Result of processing a card request
#[derive(Debug)]
pub struct ProcessedCardData {
    pub transaction_id: String,
    pub terminal_id: String,
    pub amount: String,
    pub msg_type: String,
    pub emv_data: Option<ParsedEmvData>,
}

/// Handle incoming TCP message from terminal
/// Parse JSON, extract cardData -> emvData -> de55, then parse TLV
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

    // Step 2: Extract DE55 from cardData -> emvData -> de55
    let de55 = match card_request.get_de55() {
        Ok(Some(de55_str)) => {
            info!("Extracted DE55 (length={}): {}", de55_str.len(), de55_str);
            Some(de55_str)
        }
        Ok(None) => {
            warn!("No cardData/DE55 present in message");
            None
        }
        Err(e) => {
            error!("Failed to parse cardData: {}", e);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid cardData JSON: {}", e),
            ));
        }
    };

    // Step 3: Parse TLV from DE55
    let emv_data = if let Some(de55_hex) = &de55 {
        match ParsedEmvData::from_de55(de55_hex) {
            Ok(parsed) => {
                // Log parsed EMV data
                parsed.print_summary();

                // Log important fields
                if let Some(pan) = parsed.get_pan() {
                    info!("PAN: {}", mask_pan(&pan));
                }
                if let Some(name) = parsed.get_cardholder_name() {
                    info!("Cardholder Name: {}", name);
                }
                if let Some(expiry) = parsed.get_expiry_date() {
                    info!("Expiry Date: {}", expiry);
                }
                if let Some(amount) = parsed.get_amount() {
                    info!("EMV Amount: {}", amount);
                }
                if let Some(aid) = parsed.get_aid() {
                    info!("AID: {}", aid);
                }
                if let Some(atc) = parsed.get_atc() {
                    info!("ATC: {}", atc);
                }

                Some(parsed)
            }
            Err(e) => {
                error!("Failed to parse TLV from DE55: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Step 4: Build processed data structure
    let processed = ProcessedCardData {
        transaction_id: card_request.transaction_id.clone(),
        terminal_id: card_request.trm_id.clone(),
        amount: card_request.amount.clone(),
        msg_type: card_request.msg_type.clone(),
        emv_data,
    };

    info!(
        "Successfully processed card data for transaction: {}",
        processed.transaction_id
    );

    // For now, return a success acknowledgment
    let response = build_ack_response(&card_request);

    Ok(response.into_bytes())
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

/// Build acknowledgment response
fn build_ack_response(request: &CardRequest) -> String {
    serde_json::json!({
        "status": "RECEIVED",
        "transactionId": request.transaction_id,
        "terminalId": request.trm_id,
        "message": "Transaction data received and parsed successfully"
    })
    .to_string()
}
