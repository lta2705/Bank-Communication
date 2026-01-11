use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ISO8583 Message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso8583Message {
    /// Message Type Indicator (4 digits)
    pub mti: String,
    /// Data elements (DE number -> value as hex string)
    pub fields: HashMap<u8, String>,
    /// Bitmap (128 bits represented as hex string)
    pub bitmap: String,
    /// Timestamp when message was created
    pub created_at: DateTime<Utc>,
}

impl Iso8583Message {
    /// Create a new ISO8583 message with given MTI
    pub fn new(mti: &str) -> Self {
        Self {
            mti: mti.to_string(),
            fields: HashMap::new(),
            bitmap: String::new(),
            created_at: Utc::now(),
        }
    }

    /// Set a field value (stores as hex string)
    pub fn set_field(&mut self, de: u8, value: String) {
        if de >= 1 && de <= 128 {
            self.fields.insert(de, value);
        }
    }

    /// Get a field value
    pub fn get_field(&self, de: u8) -> Option<&String> {
        self.fields.get(&de)
    }

    /// Check if a field exists
    pub fn has_field(&self, de: u8) -> bool {
        self.fields.contains_key(&de)
    }

    /// Remove a field
    pub fn remove_field(&mut self, de: u8) -> Option<String> {
        self.fields.remove(&de)
    }

    /// Get all field numbers that are set
    pub fn get_field_numbers(&self) -> Vec<u8> {
        let mut fields: Vec<u8> = self.fields.keys().copied().collect();
        fields.sort();
        fields
    }

    /// Get field count
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Build bitmap from fields
    pub fn build_bitmap(&mut self) {
        self.bitmap = Bitmap::from_fields(&self.fields).to_hex();
    }

    /// Is this a request message?
    pub fn is_request(&self) -> bool {
        matches!(self.mti.as_str(), "0100" | "0200" | "0400" | "0800")
    }

    /// Is this a response message?
    pub fn is_response(&self) -> bool {
        matches!(self.mti.as_str(), "0110" | "0210" | "0410" | "0810")
    }

    /// Get response MTI for this request
    pub fn get_response_mti(&self) -> Option<String> {
        match self.mti.as_str() {
            "0100" => Some("0110".to_string()),
            "0200" => Some("0210".to_string()),
            "0400" => Some("0410".to_string()),
            "0800" => Some("0810".to_string()),
            _ => None,
        }
    }
}

/// Bitmap handler for ISO8583 messages
#[derive(Debug, Clone)]
pub struct Bitmap {
    /// Bits 1-128 (primary + secondary bitmaps)
    bits: [bool; 128],
}

impl Bitmap {
    /// Create empty bitmap
    pub fn new() -> Self {
        Self { bits: [false; 128] }
    }

    /// Create bitmap from field map
    pub fn from_fields(fields: &HashMap<u8, String>) -> Self {
        let mut bitmap = Self::new();
        for &de in fields.keys() {
            if de >= 1 && de <= 128 {
                bitmap.set_bit(de);
            }
        }
        bitmap
    }

    /// Create bitmap from hex string
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.trim().replace(" ", "");
        
        // Primary bitmap is 16 hex chars (8 bytes = 64 bits)
        // Secondary bitmap is another 16 hex chars if bit 1 is set
        if hex.len() != 16 && hex.len() != 32 {
            return Err(format!("Invalid bitmap length: {}", hex.len()));
        }

        let bytes = hex::decode(&hex).map_err(|e| e.to_string())?;
        let mut bitmap = Self::new();

        for (byte_idx, &byte) in bytes.iter().enumerate() {
            for bit_idx in 0..8 {
                if byte & (1 << (7 - bit_idx)) != 0 {
                    let bit_position = (byte_idx * 8 + bit_idx) as u8 + 1;
                    if bit_position <= 128 {
                        bitmap.bits[bit_position as usize - 1] = true;
                    }
                }
            }
        }

        Ok(bitmap)
    }

    /// Set a bit (DE 1-128)
    pub fn set_bit(&mut self, de: u8) {
        if de >= 1 && de <= 128 {
            self.bits[de as usize - 1] = true;
            // If any bit 65-128 is set, bit 1 must be set (secondary bitmap indicator)
            if de > 64 {
                self.bits[0] = true;
            }
        }
    }

    /// Clear a bit
    pub fn clear_bit(&mut self, de: u8) {
        if de >= 1 && de <= 128 {
            self.bits[de as usize - 1] = false;
        }
    }

    /// Check if a bit is set
    pub fn is_set(&self, de: u8) -> bool {
        if de >= 1 && de <= 128 {
            self.bits[de as usize - 1]
        } else {
            false
        }
    }

    /// Convert bitmap to hex string
    pub fn to_hex(&self) -> String {
        let has_secondary = self.bits.iter().skip(64).any(|&b| b);
        let byte_count = if has_secondary { 16 } else { 8 };

        let mut bytes = Vec::with_capacity(byte_count);
        
        for byte_idx in 0..byte_count {
            let mut byte = 0u8;
            for bit_idx in 0..8 {
                let bit_position = byte_idx * 8 + bit_idx;
                if bit_position < 128 && self.bits[bit_position] {
                    byte |= 1 << (7 - bit_idx);
                }
            }
            bytes.push(byte);
        }

        hex::encode_upper(bytes)
    }

    /// Get all set bit numbers
    pub fn get_set_bits(&self) -> Vec<u8> {
        self.bits
            .iter()
            .enumerate()
            .filter(|&(_, &is_set)| is_set)
            .map(|(idx, _)| (idx + 1) as u8)
            .collect()
    }
}

impl Default for Bitmap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_creation() {
        let mut bitmap = Bitmap::new();
        bitmap.set_bit(2);
        bitmap.set_bit(3);
        bitmap.set_bit(4);
        
        assert!(bitmap.is_set(2));
        assert!(bitmap.is_set(3));
        assert!(bitmap.is_set(4));
        assert!(!bitmap.is_set(5));
    }

    #[test]
    fn test_bitmap_secondary() {
        let mut bitmap = Bitmap::new();
        bitmap.set_bit(65); // Secondary bitmap field
        
        assert!(bitmap.is_set(1)); // Bit 1 should be auto-set
        assert!(bitmap.is_set(65));
    }

    #[test]
    fn test_bitmap_hex_conversion() {
        let mut bitmap = Bitmap::new();
        bitmap.set_bit(2);
        bitmap.set_bit(3);
        
        let hex = bitmap.to_hex();
        let parsed = Bitmap::from_hex(&hex).unwrap();
        
        assert!(parsed.is_set(2));
        assert!(parsed.is_set(3));
        assert!(!parsed.is_set(4));
    }
}
