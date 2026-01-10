use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

/// Transaction types supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransactionType {
    /// Purchase transaction (MTI 0200)
    Purchase,
    /// Cash withdrawal (MTI 0200)
    CashWithdrawal,
    /// Balance inquiry (MTI 0200)
    BalanceInquiry,
    /// Refund/Return (MTI 0200)
    Refund,
    /// Pre-authorization (MTI 0100)
    PreAuth,
    /// Pre-auth completion (MTI 0200)
    PreAuthCompletion,
    /// Void/Reversal (MTI 0400)
    Void,
    /// Reversal (MTI 0400)
    Reversal,
    /// Cash advance (MTI 0200)
    CashAdvance,
    /// QR Payment (MTI 0200)
    QrPayment,
}

impl TransactionType {
    /// Get ISO8583 MTI for this transaction type
    pub fn get_mti(&self) -> &'static str {
        match self {
            TransactionType::Purchase => "0200",
            TransactionType::CashWithdrawal => "0200",
            TransactionType::BalanceInquiry => "0200",
            TransactionType::Refund => "0200",
            TransactionType::PreAuth => "0100",
            TransactionType::PreAuthCompletion => "0200",
            TransactionType::Void => "0400",
            TransactionType::Reversal => "0400",
            TransactionType::CashAdvance => "0200",
            TransactionType::QrPayment => "0200",
        }
    }

    /// Get Processing Code (DE3) for this transaction type
    pub fn get_processing_code(&self) -> &'static str {
        match self {
            TransactionType::Purchase => "000000",
            TransactionType::CashWithdrawal => "010000",
            TransactionType::BalanceInquiry => "310000",
            TransactionType::Refund => "200000",
            TransactionType::PreAuth => "000000",
            TransactionType::PreAuthCompletion => "000000",
            TransactionType::Void => "000000",
            TransactionType::Reversal => "000000",
            TransactionType::CashAdvance => "010000",
            TransactionType::QrPayment => "000000",
        }
    }

    /// Get EMV Transaction Type (Tag 9C) value
    pub fn get_emv_transaction_type(&self) -> u8 {
        match self {
            TransactionType::Purchase => 0x00,
            TransactionType::CashWithdrawal => 0x01,
            TransactionType::BalanceInquiry => 0x31,
            TransactionType::Refund => 0x20,
            TransactionType::PreAuth => 0x00,
            TransactionType::PreAuthCompletion => 0x00,
            TransactionType::Void => 0x00,
            TransactionType::Reversal => 0x00,
            TransactionType::CashAdvance => 0x01,
            TransactionType::QrPayment => 0x00,
        }
    }
}

/// Profile defining required and optional fields for a transaction
#[derive(Debug, Clone)]
pub struct TransactionProfile {
    pub transaction_type: TransactionType,
    pub name: &'static str,
    pub description: &'static str,
    /// Required ISO8583 Data Elements
    pub required_iso_des: HashSet<u8>,
    /// Optional ISO8583 Data Elements
    pub optional_iso_des: HashSet<u8>,
    /// Required EMV tags (for chip transactions)
    pub required_emv_tags: HashSet<&'static str>,
    /// Optional EMV tags
    pub optional_emv_tags: HashSet<&'static str>,
    /// Tags that must be present in DE55 for online authorization
    pub de55_required_tags: HashSet<&'static str>,
}

/// Create a HashSet from a list of items
macro_rules! hashset {
    ($($x:expr),* $(,)?) => {{
        let mut set = HashSet::new();
        $(set.insert($x);)*
        set
    }};
}

/// All transaction profiles
pub static TRANSACTION_PROFILES: Lazy<HashMap<TransactionType, TransactionProfile>> =
    Lazy::new(|| {
        let mut profiles = HashMap::new();

        // ========================================
        // PURCHASE TRANSACTION PROFILE
        // ========================================
        profiles.insert(
            TransactionType::Purchase,
            TransactionProfile {
                transaction_type: TransactionType::Purchase,
                name: "Purchase",
                description: "Standard purchase transaction",
                required_iso_des: hashset![
                    2,  // PAN
                    3,  // Processing Code
                    4,  // Transaction Amount
                    11, // STAN
                    12, // Local Transaction Time
                    13, // Local Transaction Date
                    14, // Expiration Date
                    22, // POS Entry Mode
                    23, // Card Sequence Number
                    25, // POS Condition Code
                    26, // POS PIN Capture Code
                    35, // Track 2 Data
                    41, // Terminal ID
                    42, // Merchant ID
                    49, // Currency Code
                    55, // EMV Data (ICC)
                ],
                optional_iso_des: hashset![
                    32, // Acquiring Institution ID
                    37, // Retrieval Reference Number
                    38, // Authorization ID Response
                    39, // Response Code
                    43, // Card Acceptor Name/Location
                    52, // PIN Data
                    54, // Additional Amounts
                ],
                required_emv_tags: hashset![
                    "5A",   // PAN
                    "5F24", // Expiry Date
                    "9F26", // Application Cryptogram
                    "9F27", // CID
                    "9F10", // IAD
                    "9F36", // ATC
                    "9F37", // Unpredictable Number
                    "95",   // TVR
                ],
                optional_emv_tags: hashset![
                    "5F20", // Cardholder Name
                    "5F34", // PAN Sequence Number
                    "9F33", // Terminal Capabilities
                    "9F34", // CVM Results
                    "9F35", // Terminal Type
                ],
                de55_required_tags: hashset![
                    "9F26", // Application Cryptogram (ARQC)
                    "9F27", // CID
                    "9F10", // IAD
                    "9F37", // Unpredictable Number
                    "9F36", // ATC
                    "95",   // TVR
                    "9A",   // Transaction Date
                    "9C",   // Transaction Type
                    "9F02", // Amount Authorized
                    "5F2A", // Transaction Currency Code
                    "82",   // AIP
                    "9F1A", // Terminal Country Code
                    "9F34", // CVM Results
                    "9F33", // Terminal Capabilities
                    "9F35", // Terminal Type
                    "4F",   // AID
                    "84",   // DF Name
                ],
            },
        );

        // ========================================
        // CASH WITHDRAWAL PROFILE
        // ========================================
        profiles.insert(
            TransactionType::CashWithdrawal,
            TransactionProfile {
                transaction_type: TransactionType::CashWithdrawal,
                name: "Cash Withdrawal",
                description: "ATM cash withdrawal transaction",
                required_iso_des: hashset![
                    2,  // PAN
                    3,  // Processing Code (010000)
                    4,  // Transaction Amount
                    11, // STAN
                    12, // Time
                    13, // Date
                    14, // Expiration Date
                    22, // POS Entry Mode
                    23, // Card Sequence Number
                    25, // POS Condition Code
                    35, // Track 2
                    41, // Terminal ID
                    42, // Merchant ID
                    49, // Currency Code
                    52, // PIN Data (required for ATM)
                    55, // EMV Data
                ],
                optional_iso_des: hashset![32, 37, 38, 39, 43, 54,],
                required_emv_tags: hashset![
                    "5A", "5F24", "9F26", "9F27", "9F10", "9F36", "9F37", "95",
                ],
                optional_emv_tags: hashset!["5F20", "5F34", "9F33", "9F34", "9F35",],
                de55_required_tags: hashset![
                    "9F26", "9F27", "9F10", "9F37", "9F36", "95", "9A", "9C", "9F02", "5F2A", "82",
                    "9F1A", "9F34", "9F33", "9F35", "4F", "84",
                ],
            },
        );

        // ========================================
        // BALANCE INQUIRY PROFILE
        // ========================================
        profiles.insert(
            TransactionType::BalanceInquiry,
            TransactionProfile {
                transaction_type: TransactionType::BalanceInquiry,
                name: "Balance Inquiry",
                description: "Balance inquiry transaction",
                required_iso_des: hashset![
                    2,  // PAN
                    3,  // Processing Code (310000)
                    11, // STAN
                    12, // Time
                    13, // Date
                    14, // Expiration
                    22, // POS Entry Mode
                    35, // Track 2
                    41, // Terminal ID
                    42, // Merchant ID
                    49, // Currency
                ],
                optional_iso_des: hashset![23, 25, 32, 37, 38, 39, 43, 52, 54, 55,],
                required_emv_tags: hashset!["5A", "5F24",],
                optional_emv_tags: hashset!["9F26", "9F27", "9F10", "9F36", "95",],
                de55_required_tags: hashset!["4F", "9A", "9C", "5F2A", "9F1A",],
            },
        );

        // ========================================
        // REFUND PROFILE
        // ========================================
        profiles.insert(
            TransactionType::Refund,
            TransactionProfile {
                transaction_type: TransactionType::Refund,
                name: "Refund",
                description: "Refund/Return transaction",
                required_iso_des: hashset![
                    2,  // PAN
                    3,  // Processing Code (200000)
                    4,  // Amount
                    11, // STAN
                    12, // Time
                    13, // Date
                    14, // Expiration
                    22, // POS Entry Mode
                    25, // POS Condition Code
                    35, // Track 2
                    37, // Original RRN (reference to original transaction)
                    41, // Terminal ID
                    42, // Merchant ID
                    49, // Currency
                ],
                optional_iso_des: hashset![23, 32, 38, 39, 43, 55,],
                required_emv_tags: hashset!["5A", "5F24",],
                optional_emv_tags: hashset!["9F26", "9F27", "9F10", "9F36", "95",],
                de55_required_tags: hashset!["4F", "9A", "9C", "9F02", "5F2A", "9F1A",],
            },
        );

        // ========================================
        // PRE-AUTHORIZATION PROFILE
        // ========================================
        profiles.insert(
            TransactionType::PreAuth,
            TransactionProfile {
                transaction_type: TransactionType::PreAuth,
                name: "Pre-Authorization",
                description: "Pre-authorization hold transaction",
                required_iso_des: hashset![
                    2,  // PAN
                    3,  // Processing Code
                    4,  // Amount
                    11, // STAN
                    12, // Time
                    13, // Date
                    14, // Expiration
                    22, // POS Entry Mode
                    23, // Card Sequence Number
                    25, // POS Condition Code
                    35, // Track 2
                    41, // Terminal ID
                    42, // Merchant ID
                    49, // Currency
                    55, // EMV Data
                ],
                optional_iso_des: hashset![32, 37, 38, 39, 43, 52, 54,],
                required_emv_tags: hashset![
                    "5A", "5F24", "9F26", "9F27", "9F10", "9F36", "9F37", "95",
                ],
                optional_emv_tags: hashset!["5F20", "5F34", "9F33", "9F34", "9F35",],
                de55_required_tags: hashset![
                    "9F26", "9F27", "9F10", "9F37", "9F36", "95", "9A", "9C", "9F02", "5F2A", "82",
                    "9F1A", "9F34", "9F33", "4F", "84",
                ],
            },
        );

        // ========================================
        // VOID/REVERSAL PROFILE
        // ========================================
        profiles.insert(
            TransactionType::Void,
            TransactionProfile {
                transaction_type: TransactionType::Void,
                name: "Void",
                description: "Void/Cancel transaction",
                required_iso_des: hashset![
                    2,  // PAN
                    3,  // Processing Code
                    4,  // Amount
                    11, // STAN
                    12, // Time
                    13, // Date
                    22, // POS Entry Mode
                    25, // POS Condition Code
                    37, // Original RRN
                    38, // Original Auth Code
                    41, // Terminal ID
                    42, // Merchant ID
                    49, // Currency
                ],
                optional_iso_des: hashset![14, 23, 32, 35, 39, 43, 55,],
                required_emv_tags: hashset!["5A",],
                optional_emv_tags: hashset!["5F24", "9F26", "9F27", "9F10", "9F36", "95",],
                de55_required_tags: hashset!["4F", "9A", "9C", "9F02", "5F2A", "9F1A",],
            },
        );

        // ========================================
        // QR PAYMENT PROFILE
        // ========================================
        profiles.insert(
            TransactionType::QrPayment,
            TransactionProfile {
                transaction_type: TransactionType::QrPayment,
                name: "QR Payment",
                description: "QR code based payment (VietQR, etc.)",
                required_iso_des: hashset![
                    3,  // Processing Code
                    4,  // Amount
                    11, // STAN
                    12, // Time
                    13, // Date
                    25, // POS Condition Code
                    41, // Terminal ID
                    42, // Merchant ID
                    49, // Currency
                ],
                optional_iso_des: hashset![
                    2,   // PAN (may be present)
                    32,  // Acquiring Institution
                    37,  // RRN
                    38,  // Auth Code
                    39,  // Response Code
                    43,  // Merchant Name/Location
                    102, // Account ID 1
                    103, // Account ID 2
                ],
                required_emv_tags: hashset![], // QR typically doesn't have EMV data
                optional_emv_tags: hashset![],
                de55_required_tags: hashset![],
            },
        );

        profiles
    });

/// Get transaction profile by type
pub fn get_profile(tx_type: TransactionType) -> Option<&'static TransactionProfile> {
    TRANSACTION_PROFILES.get(&tx_type)
}

/// Get all available transaction profiles
pub fn get_all_profiles() -> &'static HashMap<TransactionType, TransactionProfile> {
    &TRANSACTION_PROFILES
}

/// Validate if all required fields are present for a transaction
pub fn validate_transaction_fields(
    tx_type: TransactionType,
    present_iso_des: &HashSet<u8>,
    present_emv_tags: &HashSet<&str>,
) -> ValidationResult {
    let profile = match get_profile(tx_type) {
        Some(p) => p,
        None => {
            return ValidationResult {
                is_valid: false,
                missing_iso_des: vec![],
                missing_emv_tags: vec![],
                warnings: vec!["Unknown transaction type".to_string()],
            };
        }
    };

    let missing_iso: Vec<u8> = profile
        .required_iso_des
        .iter()
        .filter(|de| !present_iso_des.contains(*de))
        .copied()
        .collect();

    let missing_emv: Vec<&str> = profile
        .required_emv_tags
        .iter()
        .filter(|tag| !present_emv_tags.contains(*tag))
        .copied()
        .collect();

    let mut warnings = Vec::new();

    // Check optional but recommended fields
    for de in &profile.optional_iso_des {
        if !present_iso_des.contains(de) {
            warnings.push(format!("Optional DE{} is missing", de));
        }
    }

    ValidationResult {
        is_valid: missing_iso.is_empty() && missing_emv.is_empty(),
        missing_iso_des: missing_iso,
        missing_emv_tags: missing_emv.iter().map(|s| s.to_string()).collect(),
        warnings,
    }
}

/// Result of field validation
#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub missing_iso_des: Vec<u8>,
    pub missing_emv_tags: Vec<String>,
    pub warnings: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_purchase_profile() {
        let profile = get_profile(TransactionType::Purchase).unwrap();
        assert_eq!(profile.name, "Purchase");
        assert!(profile.required_iso_des.contains(&2)); // PAN
        assert!(profile.required_iso_des.contains(&55)); // EMV Data
    }

    #[test]
    fn test_transaction_mti() {
        assert_eq!(TransactionType::Purchase.get_mti(), "0200");
        assert_eq!(TransactionType::PreAuth.get_mti(), "0100");
        assert_eq!(TransactionType::Void.get_mti(), "0400");
    }

    #[test]
    fn test_processing_codes() {
        assert_eq!(TransactionType::Purchase.get_processing_code(), "000000");
        assert_eq!(
            TransactionType::CashWithdrawal.get_processing_code(),
            "010000"
        );
        assert_eq!(
            TransactionType::BalanceInquiry.get_processing_code(),
            "310000"
        );
        assert_eq!(TransactionType::Refund.get_processing_code(), "200000");
    }

    #[test]
    fn test_validate_transaction() {
        let present_des: HashSet<u8> =
            hashset![2, 3, 4, 11, 12, 13, 14, 22, 23, 25, 26, 35, 41, 42, 49, 55];
        let present_tags: HashSet<&str> =
            hashset!["5A", "5F24", "9F26", "9F27", "9F10", "9F36", "9F37", "95"];

        let result =
            validate_transaction_fields(TransactionType::Purchase, &present_des, &present_tags);

        assert!(result.is_valid);
    }

    #[test]
    fn test_missing_fields_detection() {
        let present_des: HashSet<u8> = hashset![2, 3, 4, 11]; // Missing many required fields
        let present_tags: HashSet<&str> = hashset!["5A"];

        let result =
            validate_transaction_fields(TransactionType::Purchase, &present_des, &present_tags);

        assert!(!result.is_valid);
        assert!(!result.missing_iso_des.is_empty());
        assert!(!result.missing_emv_tags.is_empty());
    }

    #[test]
    fn test_de55_required_tags() {
        let profile = get_profile(TransactionType::Purchase).unwrap();

        // These tags must be in DE55 for online authorization
        assert!(profile.de55_required_tags.contains("9F26")); // Cryptogram
        assert!(profile.de55_required_tags.contains("9F27")); // CID
        assert!(profile.de55_required_tags.contains("9F10")); // IAD
        assert!(profile.de55_required_tags.contains("9F36")); // ATC
    }
}
