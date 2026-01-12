use serde_json;
use std::io;
use std::sync::Arc;
use tracing::{error, info};

use crate::app::security::mac_calculator::MacCalculator;
use crate::app::service::response_handler::{MockBankResponseHandler, ResponseHandler};
use crate::app::service::stan_generator::StanGenerator;
use crate::app::service::tlv_parser::ParsedEmvData;
use crate::models::card_request::CardRequest;
use crate::models::iso8583_message::Iso8583Message;
use crate::models::transaction::{Iso8583Transaction, TransactionState};
use crate::repository::card_transaction_repository::CardTransactionRepository;
use chrono::Local;

/// ISO8583 Transaction Service
/// Handles complete transaction lifecycle from request to response
pub struct Iso8583TransactionService {
    stan_generator: Arc<StanGenerator>,
    transaction_repo: Arc<CardTransactionRepository>,
    mock_bank_handler: MockBankResponseHandler,
    mac_calculator: MacCalculator,
}

impl Iso8583TransactionService {
    pub fn new(
        stan_generator: Arc<StanGenerator>,
        transaction_repo: Arc<CardTransactionRepository>,
    ) -> Self {
        Self {
            stan_generator,
            transaction_repo,
            mock_bank_handler: MockBankResponseHandler::default_mock(),
            mac_calculator: MacCalculator::new_mock(),
        }
    }

    /// Process incoming transaction request
    pub async fn process_transaction(
        &self,
        card_request: &CardRequest,
    ) -> Result<serde_json::Value, io::Error> {
        info!(
            "Processing transaction: ID={}, Amount={}",
            card_request.transaction_id, card_request.amount
        );

        // 1. Generate STAN
        let stan = self.stan_generator.next().await;
        info!("Generated STAN: {}", stan);

        // 2. Build ISO8583 request message
        let request_msg = self.build_iso_message(card_request, &stan)?;

        // 3. Save transaction to database
        let db_transaction = self.create_db_transaction(&request_msg, card_request)?;

        info!("Saving transaction to database... {:?}", db_transaction);

        self.transaction_repo
            .insert(&db_transaction)
            .await
            .map_err(|e| {
                error!("Failed to save transaction: {}", e);
                io::Error::new(io::ErrorKind::Other, format!("Database error: {}", e))
            })?;

        info!("Transaction saved to database: STAN={}", stan);

        // Update state to SENT
        self.transaction_repo
            .update_response(
                &db_transaction.tr_dt,
                &db_transaction.tr_tm,
                &db_transaction.tr_uniq_no,
                None,
                None,
                None,
                &TransactionState::Sent,
            )
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Database error: {}", e)))?;

        // 4. Send to mock bank and get response (simulating network call)
        info!("Sending request to mock bank...");
        self.mock_bank_handler.simulate_delay().await;

        let response_msg = self.mock_bank_handler.process_request(&request_msg).await;

        // 5. Parse response
        let (state, _response_code) = ResponseHandler::parse_response(&response_msg);
        let response_code_str = response_msg.get_field(39).map(|s| s.as_str());
        let auth_code = response_msg.get_field(38).map(|s| s.as_str());
        let rrn = response_msg.get_field(37).map(|s| s.as_str());

        info!(
            "Received response: Code={:?}, State={:?}",
            response_code_str, state
        );

        // 6. Update transaction with response
        self.transaction_repo
            .update_response(
                &db_transaction.tr_dt,
                &db_transaction.tr_tm,
                &db_transaction.tr_uniq_no,
                response_code_str,
                auth_code,
                rrn,
                &state,
            )
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Database error: {}", e)))?;

        // 7. Build response JSON
        let response_json = self.build_response_json(card_request, &response_msg, &stan, &state);

        info!("Transaction completed: STAN={}, State={:?}", stan, state);

        Ok(response_json)
    }

    /// Build ISO8583 message from card request
    fn build_iso_message(
        &self,
        card_request: &CardRequest,
        stan: &str,
    ) -> Result<Iso8583Message, io::Error> {
        let mut msg = Iso8583Message::new("0200"); // Financial request

        let now = Local::now();

        // DE3: Processing Code (based on transaction type)
        let processing_code = match card_request.msg_type.as_str() {
            "SALE" | "PURCHASE" => "000000",
            "CASH_WITHDRAWAL" => "010000",
            "BALANCE_INQUIRY" => "310000",
            "REFUND" => "200000",
            _ => "000000",
        };
        msg.set_field(3, processing_code.to_string());

        // DE4: Amount (12 digits, no decimal)
        let amount_str = format!("{:012}", (card_request.amount * 100.0) as u64);
        msg.set_field(4, amount_str);

        // DE7: Transmission Date & Time (MMDDhhmmss)
        msg.set_field(7, now.format("%m%d%H%M%S").to_string());

        // DE11: STAN
        msg.set_field(11, stan.to_string());

        // DE12: Time, Local Transaction (hhmmss)
        msg.set_field(12, now.format("%H%M%S").to_string());

        // DE13: Date, Local Transaction (MMDD)
        msg.set_field(13, now.format("%m%d").to_string());

        // DE22: POS Entry Mode
        msg.set_field(22, "051".to_string()); // ICC (Chip)

        // DE25: POS Condition Code
        msg.set_field(25, "00".to_string()); // Normal presentment

        // DE41: Terminal ID
        msg.set_field(41, card_request.trm_id.clone());

        // DE42: Merchant ID (if available)
        if let Some(merchant_id) = &card_request.merchant_id {
            msg.set_field(42, format!("{:15}", merchant_id));
        }

        // DE49: Currency Code (VND = 704)
        msg.set_field(49, "704".to_string());

        // DE55: EMV Data (from cardData if available)
        if let Ok(Some(de55)) = card_request.get_de55() {
            msg.set_field(55, de55);
        }

        // Parse card data for additional fields
        if let Ok(Some(card_data_str)) = card_request.get_card_data_string() {
            if let Ok(emv_data) = ParsedEmvData::from_de55(&card_data_str) {
                // DE2: PAN
                if let Some(pan) = emv_data.get_pan() {
                    msg.set_field(2, pan);
                }

                // DE14: Expiration Date
                if let Some(expiry) = emv_data.get_expiry_date() {
                    msg.set_field(14, expiry);
                }

                // DE23: Card Sequence Number
                if let Some(psn) = emv_data.get_value("5F34") {
                    msg.set_field(23, format!("{:03}", psn));
                }
            }
        }

        Ok(msg)
    }

    /// Create database transaction record from ISO message
    fn create_db_transaction(
        &self,
        msg: &Iso8583Message,
        card_request: &CardRequest,
    ) -> Result<Iso8583Transaction, io::Error> {
        let stan = msg
            .get_field(11)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing STAN"))?;

        let mut db_tx = Iso8583Transaction::new(stan, &msg.mti);

        // Copy all fields from ISO message to DB transaction
        for de in msg.get_field_numbers() {
            if let Some(value) = msg.get_field(de) {
                db_tx.set_field(de, Some(value.clone()));
            }
        }

        // Set terminal ID from card request
        db_tx.trm_id = Some(card_request.trm_id.clone());
        
        db_tx.tr_uniq_no = Some(card_request.transaction_id.clone());
        
        // db_tx.transaction

        // Set bitmap
        db_tx.field_001 = Some(msg.bitmap.clone());

        Ok(db_tx)
    }

    /// Build response JSON for client
    fn build_response_json(
        &self,
        request: &CardRequest,
        response_msg: &Iso8583Message,
        stan: &str,
        state: &TransactionState,
    ) -> serde_json::Value {
        let is_approved = ResponseHandler::is_approved(response_msg);
        let response_desc = ResponseHandler::get_response_description(response_msg);

        serde_json::json!({
            "status": if is_approved { "APPROVED" } else { "DECLINED" },
            "transactionId": request.transaction_id,
            "terminalId": request.trm_id,
            "stan": stan,
            "responseCode": response_msg.get_field(39),
            "authorizationCode": response_msg.get_field(38),
            "rrn": response_msg.get_field(37),
            "responseMessage": response_desc,
            "transactionState": state.as_str(),
            "amount": request.amount,
            "timestamp": Local::now().to_rfc3339(),
        })
    }
}