use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Represents the mapping between an EMV Tag and ISO8583 Data Element
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EmvIsoMapping {
    pub emv_tag: &'static str,
    pub emv_name: &'static str,
    pub iso_de: u8,
    pub iso_subfield: Option<u8>,
    pub iso_de_name: &'static str,
    pub format: DataFormat,
    pub max_length: usize,
}

/// Data format types for encoding/decoding
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum DataFormat {
    /// Binary data (hex)
    Binary,
    /// Numeric (BCD encoded)
    Numeric,
    /// Alphanumeric (ASCII)
    Alphanumeric,
    /// Compressed Numeric (2 digits per byte)
    CompressedNumeric,
    /// Track 2 equivalent data
    Track2,
}

/// Static mapping table: EMV Tag -> ISO8583 DE
#[allow(dead_code)]
pub static EMV_TO_ISO_MAP: Lazy<HashMap<&'static str, EmvIsoMapping>> = Lazy::new(|| {
    let mappings = vec![
        // DE2 - Primary Account Number (PAN)
        EmvIsoMapping {
            emv_tag: "5A",
            emv_name: "Application PAN",
            iso_de: 2,
            iso_subfield: None,
            iso_de_name: "Primary Account Number",
            format: DataFormat::Numeric,
            max_length: 19,
        },
        // DE14 - Expiration Date
        EmvIsoMapping {
            emv_tag: "5F24",
            emv_name: "Application Expiration Date",
            iso_de: 14,
            iso_subfield: None,
            iso_de_name: "Date, Expiration",
            format: DataFormat::Numeric,
            max_length: 4, // YYMM
        },
        // DE22 - Point of Service Entry Mode
        EmvIsoMapping {
            emv_tag: "9F39",
            emv_name: "POS Entry Mode",
            iso_de: 22,
            iso_subfield: None,
            iso_de_name: "Point of Service Entry Mode",
            format: DataFormat::Numeric,
            max_length: 3,
        },
        // DE23 - Card Sequence Number
        EmvIsoMapping {
            emv_tag: "5F34",
            emv_name: "PAN Sequence Number",
            iso_de: 23,
            iso_subfield: None,
            iso_de_name: "Card Sequence Number",
            format: DataFormat::Numeric,
            max_length: 3,
        },
        // DE35 - Track 2 Equivalent Data
        EmvIsoMapping {
            emv_tag: "57",
            emv_name: "Track 2 Equivalent Data",
            iso_de: 35,
            iso_subfield: None,
            iso_de_name: "Track 2 Data",
            format: DataFormat::Track2,
            max_length: 37,
        },
        // DE41 - Card Acceptor Terminal ID
        EmvIsoMapping {
            emv_tag: "9F1E",
            emv_name: "IFD Serial Number",
            iso_de: 41,
            iso_subfield: None,
            iso_de_name: "Card Acceptor Terminal ID",
            format: DataFormat::Alphanumeric,
            max_length: 8,
        },
        // DE49 - Currency Code, Transaction
        EmvIsoMapping {
            emv_tag: "5F2A",
            emv_name: "Transaction Currency Code",
            iso_de: 49,
            iso_subfield: None,
            iso_de_name: "Currency Code, Transaction",
            format: DataFormat::Numeric,
            max_length: 3,
        },
        // DE55 - ICC Data (contains multiple EMV tags)
        // This is a container for chip data sent to issuer

        // === DE55 Subfields (EMV tags that go into DE55) ===

        // DE55.1 - Application Identifier (AID)
        EmvIsoMapping {
            emv_tag: "4F",
            emv_name: "Application Identifier (AID)",
            iso_de: 55,
            iso_subfield: Some(1),
            iso_de_name: "DE55 - AID",
            format: DataFormat::Binary,
            max_length: 16,
        },
        // DE55.2 - Application Interchange Profile
        EmvIsoMapping {
            emv_tag: "82",
            emv_name: "Application Interchange Profile",
            iso_de: 55,
            iso_subfield: Some(2),
            iso_de_name: "DE55 - AIP",
            format: DataFormat::Binary,
            max_length: 2,
        },
        // DE55.3 - Application Transaction Counter
        EmvIsoMapping {
            emv_tag: "9F36",
            emv_name: "Application Transaction Counter",
            iso_de: 55,
            iso_subfield: Some(3),
            iso_de_name: "DE55 - ATC",
            format: DataFormat::Binary,
            max_length: 2,
        },
        // DE55.4 - Application Cryptogram
        EmvIsoMapping {
            emv_tag: "9F26",
            emv_name: "Application Cryptogram",
            iso_de: 55,
            iso_subfield: Some(4),
            iso_de_name: "DE55 - Cryptogram",
            format: DataFormat::Binary,
            max_length: 8,
        },
        // DE55.5 - Cryptogram Information Data
        EmvIsoMapping {
            emv_tag: "9F27",
            emv_name: "Cryptogram Information Data",
            iso_de: 55,
            iso_subfield: Some(5),
            iso_de_name: "DE55 - CID",
            format: DataFormat::Binary,
            max_length: 1,
        },
        // DE55.6 - Issuer Application Data
        EmvIsoMapping {
            emv_tag: "9F10",
            emv_name: "Issuer Application Data",
            iso_de: 55,
            iso_subfield: Some(6),
            iso_de_name: "DE55 - IAD",
            format: DataFormat::Binary,
            max_length: 32,
        },
        // DE55.7 - Terminal Verification Results
        EmvIsoMapping {
            emv_tag: "95",
            emv_name: "Terminal Verification Results",
            iso_de: 55,
            iso_subfield: Some(7),
            iso_de_name: "DE55 - TVR",
            format: DataFormat::Binary,
            max_length: 5,
        },
        // DE55.8 - Transaction Date
        EmvIsoMapping {
            emv_tag: "9A",
            emv_name: "Transaction Date",
            iso_de: 55,
            iso_subfield: Some(8),
            iso_de_name: "DE55 - Transaction Date",
            format: DataFormat::Numeric,
            max_length: 3,
        },
        // DE55.9 - Transaction Type
        EmvIsoMapping {
            emv_tag: "9C",
            emv_name: "Transaction Type",
            iso_de: 55,
            iso_subfield: Some(9),
            iso_de_name: "DE55 - Transaction Type",
            format: DataFormat::Numeric,
            max_length: 1,
        },
        // DE55.10 - Amount Authorized
        EmvIsoMapping {
            emv_tag: "9F02",
            emv_name: "Amount, Authorized",
            iso_de: 55,
            iso_subfield: Some(10),
            iso_de_name: "DE55 - Amount Authorized",
            format: DataFormat::Numeric,
            max_length: 6,
        },
        // DE55.11 - Amount Other
        EmvIsoMapping {
            emv_tag: "9F03",
            emv_name: "Amount, Other",
            iso_de: 55,
            iso_subfield: Some(11),
            iso_de_name: "DE55 - Amount Other",
            format: DataFormat::Numeric,
            max_length: 6,
        },
        // DE55.12 - Terminal Country Code
        EmvIsoMapping {
            emv_tag: "9F1A",
            emv_name: "Terminal Country Code",
            iso_de: 55,
            iso_subfield: Some(12),
            iso_de_name: "DE55 - Terminal Country Code",
            format: DataFormat::Numeric,
            max_length: 2,
        },
        // DE55.13 - Unpredictable Number
        EmvIsoMapping {
            emv_tag: "9F37",
            emv_name: "Unpredictable Number",
            iso_de: 55,
            iso_subfield: Some(13),
            iso_de_name: "DE55 - UN",
            format: DataFormat::Binary,
            max_length: 4,
        },
        // DE55.14 - Terminal Capabilities
        EmvIsoMapping {
            emv_tag: "9F33",
            emv_name: "Terminal Capabilities",
            iso_de: 55,
            iso_subfield: Some(14),
            iso_de_name: "DE55 - Terminal Capabilities",
            format: DataFormat::Binary,
            max_length: 3,
        },
        // DE55.15 - CVM Results
        EmvIsoMapping {
            emv_tag: "9F34",
            emv_name: "CVM Results",
            iso_de: 55,
            iso_subfield: Some(15),
            iso_de_name: "DE55 - CVM Results",
            format: DataFormat::Binary,
            max_length: 3,
        },
        // DE55.16 - Terminal Type
        EmvIsoMapping {
            emv_tag: "9F35",
            emv_name: "Terminal Type",
            iso_de: 55,
            iso_subfield: Some(16),
            iso_de_name: "DE55 - Terminal Type",
            format: DataFormat::Numeric,
            max_length: 1,
        },
        // DE55.17 - Application Version Number
        EmvIsoMapping {
            emv_tag: "9F09",
            emv_name: "Application Version Number",
            iso_de: 55,
            iso_subfield: Some(17),
            iso_de_name: "DE55 - App Version",
            format: DataFormat::Binary,
            max_length: 2,
        },
        // DE55.18 - Dedicated File Name
        EmvIsoMapping {
            emv_tag: "84",
            emv_name: "Dedicated File Name",
            iso_de: 55,
            iso_subfield: Some(18),
            iso_de_name: "DE55 - DF Name",
            format: DataFormat::Binary,
            max_length: 16,
        },
    ];

    let mut map = HashMap::new();
    for mapping in mappings {
        map.insert(mapping.emv_tag, mapping);
    }
    map
});

/// Reverse mapping: ISO DE -> List of EMV Tags
#[allow(dead_code)]
pub static ISO_TO_EMV_MAP: Lazy<HashMap<u8, Vec<&'static str>>> = Lazy::new(|| {
    let mut map: HashMap<u8, Vec<&'static str>> = HashMap::new();

    for (emv_tag, mapping) in EMV_TO_ISO_MAP.iter() {
        map.entry(mapping.iso_de).or_default().push(*emv_tag);
    }

    map
});

/// Get ISO8583 DE for an EMV tag
#[allow(dead_code)]
pub fn get_iso_de_for_emv(emv_tag: &str) -> Option<&'static EmvIsoMapping> {
    EMV_TO_ISO_MAP.get(emv_tag)
}

/// Get all EMV tags that map to a specific ISO DE
#[allow(dead_code)]
pub fn get_emv_tags_for_iso_de(iso_de: u8) -> Option<&'static Vec<&'static str>> {
    ISO_TO_EMV_MAP.get(&iso_de)
}

/// Check if an EMV tag should be included in DE55
#[allow(dead_code)]
pub fn is_de55_tag(emv_tag: &str) -> bool {
    EMV_TO_ISO_MAP
        .get(emv_tag)
        .map(|m| m.iso_de == 55)
        .unwrap_or(false)
}

/// Get all EMV tags that go into DE55
#[allow(dead_code)]
pub fn get_de55_tags() -> Vec<&'static str> {
    EMV_TO_ISO_MAP
        .iter()
        .filter(|(_, m)| m.iso_de == 55)
        .map(|(tag, _)| *tag)
        .collect()
}
