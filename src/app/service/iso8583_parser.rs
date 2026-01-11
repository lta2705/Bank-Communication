use crate::models::iso8583_message::{Iso8583Message, Bitmap};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid message length: {0}")]
    InvalidLength(usize),
    
    #[error("Invalid MTI: {0}")]
    InvalidMti(String),
    
    #[error("Invalid bitmap: {0}")]
    InvalidBitmap(String),
    
    #[error("Invalid field {de}: {msg}")]
    InvalidField { de: u8, msg: String },
    
    #[error("Missing required field: {0}")]
    MissingField(u8),
    
    #[error("Hex decode error: {0}")]
    HexError(String),
}

/// ISO8583 Field format specification
#[derive(Debug, Clone, Copy)]
pub enum FieldFormat {
    /// Fixed length numeric (BCD encoded)
    FixedNumeric(usize),
    /// Fixed length alphanumeric (ASCII)
    FixedAlpha(usize),
    /// Variable length with 2-digit length prefix (LLVAR)
    Llvar(usize), // max length
    /// Variable length with 3-digit length prefix (LLLVAR)
    Lllvar(usize), // max length
    /// Binary data
    Binary(usize),
}

/// ISO8583 Message Parser and Builder
pub struct Iso8583Parser {
    field_formats: HashMap<u8, FieldFormat>,
}

impl Iso8583Parser {
    /// Create a new parser with default field formats
    pub fn new() -> Self {
        let mut parser = Self {
            field_formats: HashMap::new(),
        };
        parser.init_default_formats();
        parser
    }

    /// Initialize default field formats based on ISO8583 standard
    fn init_default_formats(&mut self) {
        use FieldFormat::*;

        // Common field formats
        self.field_formats.insert(2, Llvar(19));        // PAN
        self.field_formats.insert(3, FixedNumeric(6));  // Processing Code
        self.field_formats.insert(4, FixedNumeric(12)); // Amount, Transaction
        self.field_formats.insert(7, FixedNumeric(10)); // Transmission Date & Time
        self.field_formats.insert(11, FixedNumeric(6)); // STAN
        self.field_formats.insert(12, FixedNumeric(6)); // Time, Local Transaction
        self.field_formats.insert(13, FixedNumeric(4)); // Date, Local Transaction
        self.field_formats.insert(14, FixedNumeric(4)); // Date, Expiration
        self.field_formats.insert(18, FixedNumeric(4)); // Merchant Type
        self.field_formats.insert(22, FixedNumeric(3)); // POS Entry Mode
        self.field_formats.insert(23, FixedNumeric(3)); // Card Sequence Number
        self.field_formats.insert(25, FixedNumeric(2)); // POS Condition Code
        self.field_formats.insert(32, Llvar(11));       // Acquiring Institution ID
        self.field_formats.insert(35, Llvar(37));       // Track 2 Data
        self.field_formats.insert(37, FixedAlpha(12));  // RRN
        self.field_formats.insert(38, FixedAlpha(6));   // Authorization Code
        self.field_formats.insert(39, FixedAlpha(2));   // Response Code
        self.field_formats.insert(41, FixedAlpha(8));   // Terminal ID
        self.field_formats.insert(42, FixedAlpha(15));  // Merchant ID
        self.field_formats.insert(43, FixedAlpha(40));  // Merchant Name/Location
        self.field_formats.insert(49, FixedNumeric(3)); // Currency Code
        self.field_formats.insert(52, Binary(8));       // PIN Data
        self.field_formats.insert(54, Lllvar(120));     // Additional Amounts
        self.field_formats.insert(55, Lllvar(999));     // EMV Data (DE55)
        self.field_formats.insert(60, Lllvar(999));     // Reserved Private
        self.field_formats.insert(61, Lllvar(999));     // Reserved Private
        self.field_formats.insert(62, Lllvar(999));     // Reserved Private
        self.field_formats.insert(63, Lllvar(999));     // Reserved Private
        self.field_formats.insert(64, Binary(8));       // MAC
        self.field_formats.insert(70, FixedNumeric(3)); // Network Management Code
        self.field_formats.insert(90, FixedNumeric(42)); // Original Data Elements
        self.field_formats.insert(95, FixedAlpha(42));  // Replacement Amounts
        self.field_formats.insert(102, Llvar(28));      // Account ID 1
        self.field_formats.insert(103, Llvar(28));      // Account ID 2
        self.field_formats.insert(123, Lllvar(999));    // Reserved Private
        self.field_formats.insert(127, Lllvar(999));    // Reserved Private
        self.field_formats.insert(128, Binary(8));      // MAC 2
    }

    /// Parse ISO8583 message from hex string
    pub fn parse(&self, hex_data: &str) -> Result<Iso8583Message, ParseError> {
        let data = hex::decode(hex_data.trim().replace(" ", ""))
            .map_err(|e| ParseError::HexError(e.to_string()))?;

        if data.len() < 12 {
            return Err(ParseError::InvalidLength(data.len()));
        }

        let mut pos = 0;

        // Parse MTI (4 bytes)
        let mti = String::from_utf8(data[pos..pos + 4].to_vec())
            .map_err(|_| ParseError::InvalidMti("Invalid MTI encoding".to_string()))?;
        pos += 4;

        // Parse Primary Bitmap (8 bytes)
        let primary_bitmap = &data[pos..pos + 8];
        pos += 8;

        // Check if secondary bitmap exists (bit 1 set)
        let has_secondary = (primary_bitmap[0] & 0x80) != 0;
        let bitmap_hex = if has_secondary {
            if data.len() < pos + 8 {
                return Err(ParseError::InvalidBitmap("Missing secondary bitmap".to_string()));
            }
            let secondary_bitmap = &data[pos..pos + 8];
            pos += 8;
            hex::encode_upper([primary_bitmap, secondary_bitmap].concat())
        } else {
            hex::encode_upper(primary_bitmap)
        };

        let bitmap = Bitmap::from_hex(&bitmap_hex)
            .map_err(|e| ParseError::InvalidBitmap(e))?;

        let mut message = Iso8583Message::new(&mti);
        message.bitmap = bitmap_hex;

        // Parse fields based on bitmap
        for de in bitmap.get_set_bits() {
            if de == 1 {
                continue; // Skip bitmap indicator
            }

            let (field_value, bytes_read) = self.parse_field(&data[pos..], de)?;
            message.set_field(de, field_value);
            pos += bytes_read;
        }

        Ok(message)
    }

    /// Parse a single field
    fn parse_field(&self, data: &[u8], de: u8) -> Result<(String, usize), ParseError> {
        let format = self.field_formats.get(&de)
            .ok_or_else(|| ParseError::InvalidField {
                de,
                msg: "Unknown field format".to_string(),
            })?;

        match format {
            FieldFormat::FixedNumeric(len) => {
                let bcd_len = (len + 1) / 2;
                if data.len() < bcd_len {
                    return Err(ParseError::InvalidField {
                        de,
                        msg: format!("Insufficient data for fixed numeric field"),
                    });
                }
                let value = hex::encode_upper(&data[..bcd_len]);
                Ok((value[..(*len).min(value.len())].to_string(), bcd_len))
            }

            FieldFormat::FixedAlpha(len) => {
                if data.len() < *len {
                    return Err(ParseError::InvalidField {
                        de,
                        msg: "Insufficient data for fixed alpha field".to_string(),
                    });
                }
                let value = String::from_utf8_lossy(&data[..*len]).to_string();
                Ok((value, *len))
            }

            FieldFormat::Llvar(max_len) => {
                if data.len() < 1 {
                    return Err(ParseError::InvalidField {
                        de,
                        msg: "Missing length prefix".to_string(),
                    });
                }
                let len_str = String::from_utf8_lossy(&data[..2]).to_string();
                let len: usize = len_str.parse().map_err(|_| ParseError::InvalidField {
                    de,
                    msg: "Invalid length prefix".to_string(),
                })?;

                if len > *max_len {
                    return Err(ParseError::InvalidField {
                        de,
                        msg: format!("Length {} exceeds maximum {}", len, max_len),
                    });
                }

                if data.len() < 2 + len {
                    return Err(ParseError::InvalidField {
                        de,
                        msg: "Insufficient data".to_string(),
                    });
                }

                let value = String::from_utf8_lossy(&data[2..2 + len]).to_string();
                Ok((value, 2 + len))
            }

            FieldFormat::Lllvar(max_len) => {
                if data.len() < 3 {
                    return Err(ParseError::InvalidField {
                        de,
                        msg: "Missing length prefix".to_string(),
                    });
                }
                let len_str = String::from_utf8_lossy(&data[..3]).to_string();
                let len: usize = len_str.parse().map_err(|_| ParseError::InvalidField {
                    de,
                    msg: "Invalid length prefix".to_string(),
                })?;

                if len > *max_len {
                    return Err(ParseError::InvalidField {
                        de,
                        msg: format!("Length {} exceeds maximum {}", len, max_len),
                    });
                }

                if data.len() < 3 + len {
                    return Err(ParseError::InvalidField {
                        de,
                        msg: "Insufficient data".to_string(),
                    });
                }

                let value = hex::encode_upper(&data[3..3 + len]);
                Ok((value, 3 + len))
            }

            FieldFormat::Binary(len) => {
                if data.len() < *len {
                    return Err(ParseError::InvalidField {
                        de,
                        msg: "Insufficient data for binary field".to_string(),
                    });
                }
                let value = hex::encode_upper(&data[..*len]);
                Ok((value, *len))
            }
        }
    }

    /// Build ISO8583 message into hex string
    pub fn build(&self, message: &mut Iso8583Message) -> Result<String, ParseError> {
        let mut result = Vec::new();

        // Add MTI
        result.extend_from_slice(message.mti.as_bytes());

        // Build bitmap
        message.build_bitmap();
        let bitmap_bytes = hex::decode(&message.bitmap)
            .map_err(|e| ParseError::HexError(e.to_string()))?;
        result.extend_from_slice(&bitmap_bytes);

        // Add fields in order
        for de in message.get_field_numbers() {
            if de == 1 {
                continue; // Skip bitmap indicator
            }

            if let Some(value) = message.get_field(de) {
                let field_bytes = self.build_field(de, value)?;
                result.extend_from_slice(&field_bytes);
            }
        }

        Ok(hex::encode_upper(result))
    }

    /// Build a single field
    fn build_field(&self, de: u8, value: &str) -> Result<Vec<u8>, ParseError> {
        let format = self.field_formats.get(&de)
            .ok_or_else(|| ParseError::InvalidField {
                de,
                msg: "Unknown field format".to_string(),
            })?;

        match format {
            FieldFormat::FixedNumeric(len) => {
                let padded = format!("{:0>width$}", value, width = len);
                let bytes = hex::decode(&padded)
                    .map_err(|e| ParseError::HexError(e.to_string()))?;
                Ok(bytes)
            }

            FieldFormat::FixedAlpha(len) => {
                let padded = format!("{:<width$}", value, width = len);
                Ok(padded.as_bytes().to_vec())
            }

            FieldFormat::Llvar(_) => {
                let len = format!("{:02}", value.len());
                let mut result = len.as_bytes().to_vec();
                result.extend_from_slice(value.as_bytes());
                Ok(result)
            }

            FieldFormat::Lllvar(_) => {
                let hex_bytes = hex::decode(value)
                    .map_err(|e| ParseError::HexError(e.to_string()))?;
                let len = format!("{:03}", hex_bytes.len());
                let mut result = len.as_bytes().to_vec();
                result.extend_from_slice(&hex_bytes);
                Ok(result)
            }

            FieldFormat::Binary(_) => {
                hex::decode(value).map_err(|e| ParseError::HexError(e.to_string()))
            }
        }
    }
}

impl Default for Iso8583Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_simple_message() {
        let parser = Iso8583Parser::new();
        let mut msg = Iso8583Message::new("0200");
        
        msg.set_field(3, "000000".to_string());
        msg.set_field(4, "000000100000".to_string());
        msg.set_field(11, "123456".to_string());
        
        let result = parser.build(&mut msg);
        assert!(result.is_ok());
    }
}
