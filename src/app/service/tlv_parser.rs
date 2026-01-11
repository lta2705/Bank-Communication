use std::collections::HashMap;
use std::fmt;
use tracing::{debug, info, warn};

/// Represents a single TLV (Tag-Length-Value) element
#[derive(Debug, Clone)]
pub struct TlvElement {
    pub tag: String,
    pub length: usize,
    pub value: Vec<u8>,
}

impl TlvElement {
    /// Get value as hex string
    pub fn value_hex(&self) -> String {
        self.value.iter().map(|b| format!("{:02X}", b)).collect()
    }

    /// Get value as ASCII string (if printable)
    pub fn value_ascii(&self) -> Option<String> {
        if self.value.iter().all(|&b| b >= 0x20 && b <= 0x7E) {
            String::from_utf8(self.value.clone()).ok()
        } else {
            None
        }
    }

    /// Get value as BCD decoded number string
    pub fn value_bcd(&self) -> String {
        self.value.iter().map(|b| format!("{:02X}", b)).collect()
    }
}

impl fmt::Display for TlvElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tag: {}, Length: {}, Value: {}",
            self.tag,
            self.length,
            self.value_hex()
        )
    }
}

/// TLV Parser for EMV DE55 data
pub struct TlvParser;

impl TlvParser {
    /// Parse a hex string containing TLV encoded data
    /// Returns a HashMap of tag -> TlvElement
    pub fn parse(hex_string: &str) -> Result<HashMap<String, TlvElement>, TlvParseError> {
        let bytes = Self::hex_to_bytes(hex_string)?;
        Self::parse_bytes(&bytes)
    }

    /// Parse a list of TLV elements from hex string
    /// Returns a Vec preserving order
    pub fn parse_to_vec(hex_string: &str) -> Result<Vec<TlvElement>, TlvParseError> {
        let bytes = Self::hex_to_bytes(hex_string)?;
        Self::parse_bytes_to_vec(&bytes)
    }

    /// Convert hex string to bytes
    fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, TlvParseError> {
        let hex = hex.trim().replace(" ", "");

        if hex.len() % 2 != 0 {
            return Err(TlvParseError::InvalidHexLength);
        }

        (0..hex.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&hex[i..i + 2], 16)
                    .map_err(|_| TlvParseError::InvalidHexChar(hex[i..i + 2].to_string()))
            })
            .collect()
    }

    /// Parse bytes into HashMap
    fn parse_bytes(bytes: &[u8]) -> Result<HashMap<String, TlvElement>, TlvParseError> {
        let elements = Self::parse_bytes_to_vec(bytes)?;
        let mut map = HashMap::new();
        for elem in elements {
            map.insert(elem.tag.clone(), elem);
        }
        Ok(map)
    }

    /// Parse bytes into Vec of TlvElements
    fn parse_bytes_to_vec(bytes: &[u8]) -> Result<Vec<TlvElement>, TlvParseError> {
        let mut elements = Vec::new();
        let mut pos = 0;

        while pos < bytes.len() {
            // Parse TAG
            let (tag, tag_len) = Self::parse_tag(&bytes[pos..])?;
            pos += tag_len;

            if pos >= bytes.len() {
                warn!("Unexpected end of data after tag {}", tag);
                break;
            }

            // Parse LENGTH (BER-TLV encoding)
            let (length, len_bytes) = Self::parse_length(&bytes[pos..])?;
            pos += len_bytes;

            // Validate we have enough bytes for value
            if pos + length > bytes.len() {
                warn!(
                    "Tag {} claims length {} but only {} bytes remain",
                    tag,
                    length,
                    bytes.len() - pos
                );
                // Take what we can
                let available = bytes.len() - pos;
                let value = bytes[pos..pos + available].to_vec();
                elements.push(TlvElement {
                    tag,
                    length: available,
                    value,
                });
                break;
            }

            // Extract VALUE
            let value = bytes[pos..pos + length].to_vec();
            pos += length;

            debug!("Parsed TLV - Tag: {}, Length: {}", tag, length);

            elements.push(TlvElement { tag, length, value });
        }

        Ok(elements)
    }

    /// Parse tag from bytes, returns (tag_string, bytes_consumed)
    fn parse_tag(bytes: &[u8]) -> Result<(String, usize), TlvParseError> {
        if bytes.is_empty() {
            return Err(TlvParseError::UnexpectedEndOfData);
        }

        let first_byte = bytes[0];

        // Check if this is a multi-byte tag
        // If first byte's lower 5 bits are all 1s (0x1F), it's a multi-byte tag
        if (first_byte & 0x1F) == 0x1F {
            // Multi-byte tag
            let mut tag_bytes = vec![first_byte];
            let mut i = 1;

            // Subsequent bytes: if bit 8 is 1, more bytes follow
            while i < bytes.len() {
                tag_bytes.push(bytes[i]);
                if (bytes[i] & 0x80) == 0 {
                    // Last byte of tag
                    break;
                }
                i += 1;
            }

            let tag_str: String = tag_bytes.iter().map(|b| format!("{:02X}", b)).collect();
            Ok((tag_str, tag_bytes.len()))
        } else {
            // Single byte tag
            Ok((format!("{:02X}", first_byte), 1))
        }
    }

    /// Parse length from bytes (BER-TLV encoding), returns (length, bytes_consumed)
    fn parse_length(bytes: &[u8]) -> Result<(usize, usize), TlvParseError> {
        if bytes.is_empty() {
            return Err(TlvParseError::UnexpectedEndOfData);
        }

        let first_byte = bytes[0];

        if first_byte <= 0x7F {
            // Short form: length is just this byte
            Ok((first_byte as usize, 1))
        } else if first_byte == 0x81 {
            // Two-byte length form
            if bytes.len() < 2 {
                return Err(TlvParseError::UnexpectedEndOfData);
            }
            Ok((bytes[1] as usize, 2))
        } else if first_byte == 0x82 {
            // Three-byte length form
            if bytes.len() < 3 {
                return Err(TlvParseError::UnexpectedEndOfData);
            }
            let length = ((bytes[1] as usize) << 8) | (bytes[2] as usize);
            Ok((length, 3))
        } else if first_byte == 0x83 {
            // Four-byte length form
            if bytes.len() < 4 {
                return Err(TlvParseError::UnexpectedEndOfData);
            }
            let length =
                ((bytes[1] as usize) << 16) | ((bytes[2] as usize) << 8) | (bytes[3] as usize);
            Ok((length, 4))
        } else {
            // Invalid or unsupported length encoding
            Err(TlvParseError::InvalidLengthEncoding(first_byte))
        }
    }
}

/// Common EMV tags with their descriptions
pub struct EmvTags;

impl EmvTags {
    /// Get description for a known EMV tag
    pub fn get_description(tag: &str) -> &'static str {
        match tag {
            "4F" => "Application Identifier (AID)",
            "50" => "Application Label",
            "57" => "Track 2 Equivalent Data",
            "5A" => "Application Primary Account Number (PAN)",
            "5F20" => "Cardholder Name",
            "5F24" => "Application Expiration Date",
            "5F25" => "Application Effective Date",
            "5F28" => "Issuer Country Code",
            "5F2A" => "Transaction Currency Code",
            "5F2D" => "Language Preference",
            "5F30" => "Service Code",
            "5F34" => "PAN Sequence Number",
            "82" => "Application Interchange Profile (AIP)",
            "84" => "Dedicated File (DF) Name",
            "8C" => "Card Risk Management Data Object List 1 (CDOL1)",
            "8D" => "Card Risk Management Data Object List 2 (CDOL2)",
            "8E" => "Cardholder Verification Method (CVM) List",
            "94" => "Application File Locator (AFL)",
            "95" => "Terminal Verification Results (TVR)",
            "9A" => "Transaction Date",
            "9C" => "Transaction Type",
            "9F02" => "Amount, Authorized (Numeric)",
            "9F03" => "Amount, Other (Numeric)",
            "9F06" => "Application Identifier (AID) - Terminal",
            "9F07" => "Application Usage Control",
            "9F09" => "Application Version Number",
            "9F10" => "Issuer Application Data (IAD)",
            "9F11" => "Issuer Code Table Index",
            "9F12" => "Application Preferred Name",
            "9F1A" => "Terminal Country Code",
            "9F1E" => "Interface Device (IFD) Serial Number",
            "9F21" => "Transaction Time",
            "9F26" => "Application Cryptogram (AC)",
            "9F27" => "Cryptogram Information Data (CID)",
            "9F33" => "Terminal Capabilities",
            "9F34" => "Cardholder Verification Method (CVM) Results",
            "9F35" => "Terminal Type",
            "9F36" => "Application Transaction Counter (ATC)",
            "9F37" => "Unpredictable Number",
            "9F38" => "Processing Options Data Object List (PDOL)",
            "9F39" => "Point-of-Service (POS) Entry Mode",
            "9F40" => "Additional Terminal Capabilities",
            "9F41" => "Transaction Sequence Counter",
            "9F42" => "Application Currency Code",
            "9F53" => "Transaction Category Code",
            "9F66" => "Terminal Transaction Qualifiers (TTQ)",
            "DF8101" => "Online Response Data",
            _ => "Unknown Tag",
        }
    }
}

/// Parsed EMV data with convenient accessors
#[derive(Debug, Clone)]
pub struct ParsedEmvData {
    pub elements: HashMap<String, TlvElement>,
    pub ordered_elements: Vec<TlvElement>,
}

impl ParsedEmvData {
    /// Create from TLV parsed elements
    pub fn from_de55(hex_string: &str) -> Result<Self, TlvParseError> {
        let ordered_elements = TlvParser::parse_to_vec(hex_string)?;
        let elements = TlvParser::parse(hex_string)?;
        Ok(Self {
            elements,
            ordered_elements,
        })
    }

    /// Get PAN (Primary Account Number) - Tag 5A
    pub fn get_pan(&self) -> Option<String> {
        self.elements.get("5A").map(|e| e.value_bcd())
    }

    /// Get value by tag as hex string
    pub fn get_value(&self, tag: &str) -> Option<String> {
        self.elements.get(tag).map(|e| e.value_hex())
    }

    /// Get Cardholder Name - Tag 5F20
    pub fn get_cardholder_name(&self) -> Option<String> {
        self.elements.get("5F20").and_then(|e| e.value_ascii())
    }

    /// Get Expiry Date - Tag 5F24 (YYMMDD format)
    pub fn get_expiry_date(&self) -> Option<String> {
        self.elements.get("5F24").map(|e| e.value_bcd())
    }

    /// Get Application Identifier (AID) - Tag 4F
    pub fn get_aid(&self) -> Option<String> {
        self.elements.get("4F").map(|e| e.value_hex())
    }

    /// Get Amount Authorized - Tag 9F02
    pub fn get_amount(&self) -> Option<String> {
        self.elements.get("9F02").map(|e| e.value_bcd())
    }

    /// Get Transaction Date - Tag 9A (YYMMDD)
    pub fn get_transaction_date(&self) -> Option<String> {
        self.elements.get("9A").map(|e| e.value_bcd())
    }

    /// Get Transaction Time - Tag 9F21 (HHMMSS)
    pub fn get_transaction_time(&self) -> Option<String> {
        self.elements.get("9F21").map(|e| e.value_bcd())
    }

    /// Get Transaction Type - Tag 9C
    pub fn get_transaction_type(&self) -> Option<u8> {
        self.elements
            .get("9C")
            .and_then(|e| e.value.first().copied())
    }

    /// Get Currency Code - Tag 5F2A
    pub fn get_currency_code(&self) -> Option<String> {
        self.elements.get("5F2A").map(|e| e.value_bcd())
    }

    /// Get Application Transaction Counter - Tag 9F36
    pub fn get_atc(&self) -> Option<String> {
        self.elements.get("9F36").map(|e| e.value_hex())
    }

    /// Get Issuer Application Data - Tag 9F10
    pub fn get_iad(&self) -> Option<String> {
        self.elements.get("9F10").map(|e| e.value_hex())
    }

    /// Get Terminal ID - Tag 9F1E
    pub fn get_terminal_id(&self) -> Option<String> {
        self.elements.get("9F1E").and_then(|e| e.value_ascii())
    }

    /// Get a specific tag value as hex
    pub fn get_tag_hex(&self, tag: &str) -> Option<String> {
        self.elements.get(tag).map(|e| e.value_hex())
    }

    /// Get a specific tag value as ASCII
    pub fn get_tag_ascii(&self, tag: &str) -> Option<String> {
        self.elements.get(tag).and_then(|e| e.value_ascii())
    }

    /// Print all parsed elements with descriptions
    pub fn print_summary(&self) {
        info!("=== Parsed EMV Data Summary ===");
        info!("summary, {:?}", &self.elements);
        for elem in &self.ordered_elements {
            let desc = EmvTags::get_description(&elem.tag);
            info!(
                "  {} ({}): {} [len={}]",
                elem.tag,
                desc,
                elem.value_hex(),
                elem.length
            );
        }
        info!("================================");
    }
}

/// Errors that can occur during TLV parsing
#[derive(Debug)]
pub enum TlvParseError {
    InvalidHexLength,
    InvalidHexChar(String),
    UnexpectedEndOfData,
    InvalidLengthEncoding(u8),
    InvalidTag,
}

impl std::fmt::Display for TlvParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TlvParseError::InvalidHexLength => write!(f, "Hex string has odd length"),
            TlvParseError::InvalidHexChar(s) => write!(f, "Invalid hex characters: {}", s),
            TlvParseError::UnexpectedEndOfData => write!(f, "Unexpected end of TLV data"),
            TlvParseError::InvalidLengthEncoding(b) => {
                write!(f, "Invalid length encoding byte: 0x{:02X}", b)
            }
            TlvParseError::InvalidTag => write!(f, "Invalid TLV tag"),
        }
    }
}

impl std::error::Error for TlvParseError {}
