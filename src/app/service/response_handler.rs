use crate::models::iso8583_message::Iso8583Message;
use crate::models::transaction::TransactionState;
use chrono::Local;

/// Response codes for ISO8583 messages
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResponseCode {
    /// 00 - Approved
    Approved,
    /// 05 - Do not honor
    DoNotHonor,
    /// 12 - Invalid transaction
    InvalidTransaction,
    /// 13 - Invalid amount
    InvalidAmount,
    /// 14 - Invalid card number
    InvalidCard,
    /// 30 - Format error
    FormatError,
    /// 51 - Insufficient funds
    InsufficientFunds,
    /// 54 - Expired card
    ExpiredCard,
    /// 55 - Incorrect PIN
    IncorrectPin,
    /// 57 - Transaction not permitted
    NotPermitted,
    /// 58 - Transaction not permitted to terminal
    NotPermittedTerminal,
    /// 61 - Exceeds withdrawal limit
    ExceedsLimit,
    /// 91 - Issuer or switch inoperative
    IssuerInoperative,
    /// 96 - System malfunction
    SystemMalfunction,
}

impl ResponseCode {
    pub fn as_str(&self) -> &str {
        match self {
            ResponseCode::Approved => "00",
            ResponseCode::DoNotHonor => "05",
            ResponseCode::InvalidTransaction => "12",
            ResponseCode::InvalidAmount => "13",
            ResponseCode::InvalidCard => "14",
            ResponseCode::FormatError => "30",
            ResponseCode::InsufficientFunds => "51",
            ResponseCode::ExpiredCard => "54",
            ResponseCode::IncorrectPin => "55",
            ResponseCode::NotPermitted => "57",
            ResponseCode::NotPermittedTerminal => "58",
            ResponseCode::ExceedsLimit => "61",
            ResponseCode::IssuerInoperative => "91",
            ResponseCode::SystemMalfunction => "96",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "00" => Some(ResponseCode::Approved),
            "05" => Some(ResponseCode::DoNotHonor),
            "12" => Some(ResponseCode::InvalidTransaction),
            "13" => Some(ResponseCode::InvalidAmount),
            "14" => Some(ResponseCode::InvalidCard),
            "30" => Some(ResponseCode::FormatError),
            "51" => Some(ResponseCode::InsufficientFunds),
            "54" => Some(ResponseCode::ExpiredCard),
            "55" => Some(ResponseCode::IncorrectPin),
            "57" => Some(ResponseCode::NotPermitted),
            "58" => Some(ResponseCode::NotPermittedTerminal),
            "61" => Some(ResponseCode::ExceedsLimit),
            "91" => Some(ResponseCode::IssuerInoperative),
            "96" => Some(ResponseCode::SystemMalfunction),
            _ => None,
        }
    }

    pub fn to_transaction_state(&self) -> TransactionState {
        match self {
            ResponseCode::Approved => TransactionState::Approved,
            _ => TransactionState::Declined,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            ResponseCode::Approved => "Approved",
            ResponseCode::DoNotHonor => "Do not honor",
            ResponseCode::InvalidTransaction => "Invalid transaction",
            ResponseCode::InvalidAmount => "Invalid amount",
            ResponseCode::InvalidCard => "Invalid card number",
            ResponseCode::FormatError => "Format error",
            ResponseCode::InsufficientFunds => "Insufficient funds",
            ResponseCode::ExpiredCard => "Expired card",
            ResponseCode::IncorrectPin => "Incorrect PIN",
            ResponseCode::NotPermitted => "Transaction not permitted",
            ResponseCode::NotPermittedTerminal => "Transaction not permitted to terminal",
            ResponseCode::ExceedsLimit => "Exceeds withdrawal limit",
            ResponseCode::IssuerInoperative => "Issuer or switch inoperative",
            ResponseCode::SystemMalfunction => "System malfunction",
        }
    }
}

/// Mock Bank Response Handler
/// Simulates bank responses for testing without real bank connection
pub struct MockBankResponseHandler {
    /// Success rate (0.0 to 1.0)
    success_rate: f64,
}

impl MockBankResponseHandler {
    /// Create a new mock handler with specified success rate
    pub fn new(success_rate: f64) -> Self {
        Self {
            success_rate: success_rate.clamp(0.0, 1.0),
        }
    }

    /// Create a mock handler with 90% success rate
    pub fn default_mock() -> Self {
        Self::new(0.9)
    }

    /// Process a request and generate a mock response
    pub async fn process_request(&self, request: &Iso8583Message) -> Iso8583Message {
        let mut response = Iso8583Message::new(
            &request.get_response_mti().unwrap_or("0210".to_string())
        );

        // Copy request fields to response
        for de in [2, 3, 4, 11, 12, 13, 14, 22, 41, 42, 49] {
            if let Some(value) = request.get_field(de) {
                response.set_field(de, value.clone());
            }
        }

        // Generate mock RRN (Retrieval Reference Number)
        let rrn = self.generate_rrn();
        response.set_field(37, rrn);

        // Determine response code based on success rate
        let response_code = self.determine_response_code();
        response.set_field(39, response_code.as_str().to_string());

        // Generate authorization code for approved transactions
        if response_code == ResponseCode::Approved {
            let auth_code = self.generate_auth_code();
            response.set_field(38, auth_code);
        }

        // Add transmission date/time
        let now = Local::now();
        response.set_field(7, now.format("%m%d%H%M%S").to_string());

        // Copy bitmap from request as base
        response.bitmap = request.bitmap.clone();

        tracing::info!(
            "Mock bank response generated: MTI={}, Response Code={} ({})",
            response.mti,
            response_code.as_str(),
            response_code.description()
        );

        response
    }

    /// Determine response code based on success rate and randomization
    fn determine_response_code(&self) -> ResponseCode {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let roll: f64 = rng.sample(rand::distributions::Standard);

        if roll < self.success_rate {
            ResponseCode::Approved
        } else {
            // Randomly select an error code
            let error_codes = [
                ResponseCode::DoNotHonor,
                ResponseCode::InsufficientFunds,
                ResponseCode::InvalidCard,
                ResponseCode::ExpiredCard,
                ResponseCode::NotPermitted,
            ];
            let idx = rng.gen_range(0..error_codes.len());
            error_codes[idx]
        }
    }

    /// Generate a mock RRN (Retrieval Reference Number)
    /// Format: YYDDDHHNNNNNN (12 digits)
    fn generate_rrn(&self) -> String {
        let now = Local::now();
        let yy = now.format("%y").to_string();
        let ddd = now.format("%j").to_string();
        let hh = now.format("%H").to_string();
        
        let mut rng = rand::thread_rng();
        let nnnnnn = format!("{:06}", rand::Rng::gen_range(&mut rng, 0..999999));
        
        format!("{}{}{}{}", yy, ddd, hh, nnnnnn)
    }

    /// Generate a mock authorization code
    fn generate_auth_code(&self) -> String {
        let mut rng = rand::thread_rng();
        format!("{:06}", rand::Rng::gen_range(&mut rng, 100000..999999))
    }

    /// Simulate network delay
    pub async fn simulate_delay(&self) {
        let delay_ms = {
            let mut rng = rand::thread_rng();
            rand::Rng::gen_range(&mut rng, 50..500)
        };
        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
    }
}

impl Default for MockBankResponseHandler {
    fn default() -> Self {
        Self::default_mock()
    }
}

/// Real Response Handler for parsing actual bank responses
pub struct ResponseHandler;

impl ResponseHandler {
    /// Parse response code and convert to transaction state
    pub fn parse_response(response: &Iso8583Message) -> (TransactionState, Option<ResponseCode>) {
        if let Some(code_str) = response.get_field(39) {
            if let Some(code) = ResponseCode::from_str(code_str) {
                return (code.to_transaction_state(), Some(code));
            }
        }
        (TransactionState::Failed, None)
    }

    /// Check if response is approved
    pub fn is_approved(response: &Iso8583Message) -> bool {
        response.get_field(39).map(|c| c == "00").unwrap_or(false)
    }

    /// Get response description
    pub fn get_response_description(response: &Iso8583Message) -> String {
        if let Some(code_str) = response.get_field(39) {
            if let Some(code) = ResponseCode::from_str(code_str) {
                return code.description().to_string();
            }
        }
        "Unknown response".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_response_generation() {
        let handler = MockBankResponseHandler::default_mock();
        let mut request = Iso8583Message::new("0200");
        request.set_field(2, "4111111111111111".to_string());
        request.set_field(4, "000000100000".to_string());
        request.set_field(11, "123456".to_string());

        let response = handler.process_request(&request).await;

        assert_eq!(response.mti, "0210");
        assert!(response.has_field(37)); // RRN
        assert!(response.has_field(39)); // Response Code
    }

    #[test]
    fn test_response_code_conversion() {
        let code = ResponseCode::Approved;
        assert_eq!(code.as_str(), "00");
        assert_eq!(code.to_transaction_state(), TransactionState::Approved);

        let code = ResponseCode::InsufficientFunds;
        assert_eq!(code.as_str(), "51");
        assert_eq!(code.to_transaction_state(), TransactionState::Declined);
    }
}
