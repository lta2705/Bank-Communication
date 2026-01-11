use hex;
use ring::hmac;

/// MAC (Message Authentication Code) Calculator
/// Provides MAC generation and verification for ISO8583 messages
pub struct MacCalculator {
    /// Mock key for demonstration (in production, use HSM)
    mock_key: Vec<u8>,
}

impl MacCalculator {
    /// Create a new MAC calculator with a mock key
    pub fn new_mock() -> Self {
        // Mock key for testing (16 bytes for 3DES)
        // In production, this would come from HSM or secure key storage
        let mock_key =
            hex::decode("0123456789ABCDEFFEDCBA9876543210").expect("Failed to decode mock key");

        Self { mock_key }
    }

    /// Create with custom key
    pub fn with_key(key: Vec<u8>) -> Self {
        Self { mock_key: key }
    }

    /// Calculate MAC for message data
    /// Returns 8-byte MAC as hex string
    pub fn calculate_mac(&self, data: &[u8]) -> String {
        // Using HMAC-SHA256 for simplicity (production would use retail MAC/CBC-MAC)
        let key = hmac::Key::new(hmac::HMAC_SHA256, &self.mock_key);
        let tag = hmac::sign(&key, data);

        // Take first 8 bytes and convert to hex
        let mac_bytes = &tag.as_ref()[..8];
        hex::encode_upper(mac_bytes)
    }

    /// Verify MAC for message data
    pub fn verify_mac(&self, data: &[u8], expected_mac: &str) -> bool {
        let calculated_mac = self.calculate_mac(data);
        calculated_mac == expected_mac.to_uppercase()
    }

    /// Calculate MAC for ISO8583 message
    /// MAC is calculated over message from MTI to before MAC field (typically DE64 or DE128)
    pub fn calculate_iso_mac(&self, message_hex: &str) -> Result<String, String> {
        // Remove spaces and ensure valid hex
        let clean_hex = message_hex.trim().replace(" ", "");

        let message_bytes =
            hex::decode(&clean_hex).map_err(|e| format!("Invalid hex string: {}", e))?;

        Ok(self.calculate_mac(&message_bytes))
    }

    /// Verify ISO8583 message MAC
    pub fn verify_iso_mac(&self, message_hex: &str, mac: &str) -> Result<bool, String> {
        let calculated = self.calculate_iso_mac(message_hex)?;
        Ok(calculated == mac.to_uppercase())
    }
}

impl Default for MacCalculator {
    fn default() -> Self {
        Self::new_mock()
    }
}

/// PIN Block encryption/decryption
/// Mock implementation for demonstration
pub struct PinBlockHandler {
    /// Mock PIN key
    mock_pin_key: Vec<u8>,
}

impl PinBlockHandler {
    /// Create new PIN block handler with mock key
    pub fn new_mock() -> Self {
        let mock_pin_key =
            hex::decode("0123456789ABCDEFFEDCBA9876543210").expect("Failed to decode mock PIN key");

        Self { mock_pin_key }
    }

    /// Encrypt PIN block (Format 0 - ISO 9564-1)
    /// PIN block = PIN field XOR PAN field
    pub fn encrypt_pin(&self, pin: &str, pan: &str) -> Result<String, String> {
        if pin.len() < 4 || pin.len() > 12 {
            return Err("PIN must be 4-12 digits".to_string());
        }

        // Format: 0L[PIN][FFFF...]
        // L = PIN length
        let pin_field = format!("0{}{:0<14}", pin.len(), pin);

        // PAN field: 0000[last 12 digits of PAN excluding check digit]
        let pan_digits: String = pan.chars().filter(|c| c.is_digit(10)).collect();

        if pan_digits.len() < 13 {
            return Err("Invalid PAN length".to_string());
        }

        let pan_part = &pan_digits[pan_digits.len() - 13..pan_digits.len() - 1];
        let pan_field = format!("0000{}", pan_part);

        // XOR the two fields
        let pin_bytes =
            hex::decode(&pin_field).map_err(|_| "Failed to encode PIN field".to_string())?;
        let pan_bytes =
            hex::decode(&pan_field).map_err(|_| "Failed to encode PAN field".to_string())?;

        let mut result = Vec::with_capacity(8);
        for i in 0..8 {
            result.push(pin_bytes[i] ^ pan_bytes[i]);
        }

        // In production, this would be encrypted with PIN key using 3DES
        // For mock, we'll just return the XOR result
        Ok(hex::encode_upper(result))
    }

    /// Mock PIN verification
    pub fn verify_pin(&self, encrypted_pin: &str, pin: &str, pan: &str) -> Result<bool, String> {
        let calculated = self.encrypt_pin(pin, pan)?;
        Ok(calculated == encrypted_pin.to_uppercase())
    }
}

impl Default for PinBlockHandler {
    fn default() -> Self {
        Self::new_mock()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_calculation() {
        let calculator = MacCalculator::new_mock();
        let data = b"Hello, World!";

        let mac1 = calculator.calculate_mac(data);
        let mac2 = calculator.calculate_mac(data);

        // Same data should produce same MAC
        assert_eq!(mac1, mac2);
        assert_eq!(mac1.len(), 16); // 8 bytes = 16 hex chars
    }

    #[test]
    fn test_mac_verification() {
        let calculator = MacCalculator::new_mock();
        let data = b"Test message";

        let mac = calculator.calculate_mac(data);
        assert!(calculator.verify_mac(data, &mac));
        assert!(!calculator.verify_mac(data, "0000000000000000"));
    }

    #[test]
    fn test_pin_encryption() {
        let handler = PinBlockHandler::new_mock();
        let pin = "1234";
        let pan = "4111111111111111";

        let encrypted = handler.encrypt_pin(pin, pan);
        assert!(encrypted.is_ok());

        let encrypted_pin = encrypted.unwrap();
        assert_eq!(encrypted_pin.len(), 16); // 8 bytes = 16 hex chars
    }

    #[test]
    fn test_pin_verification() {
        let handler = PinBlockHandler::new_mock();
        let pin = "1234";
        let pan = "4111111111111111";

        let encrypted = handler.encrypt_pin(pin, pan).unwrap();
        assert!(handler.verify_pin(&encrypted, pin, pan).unwrap());
        assert!(!handler.verify_pin(&encrypted, "5678", pan).unwrap());
    }
}
