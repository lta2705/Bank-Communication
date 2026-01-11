//! ISO8583 Message Profile definitions
//! This module defines the structure for ISO8583 message profiles

/// Profile for an ISO8583 message type
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IsoMessageProfile {
    pub name: &'static str,
    pub mti: &'static str,
    pub tr_type: &'static str,
    pub required_fields: &'static [u16],
    pub optional_fields: &'static [u16],
    pub emv_profile: Option<&'static EmvProfile>,
}

/// EMV Profile containing allowed and mandatory tags
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EmvProfile {
    pub allowed_tags: &'static [&'static str],
    pub mandatory_tags: &'static [&'static str],
}

// ========================================
// PURCHASE PROFILE
// ========================================
#[allow(dead_code)]
pub static PURCHASE_EMV_PROFILE: EmvProfile = EmvProfile {
    allowed_tags: &[
        "4F", "50", "57", "5A", "5F20", "5F24", "5F2A", "5F34", "82", "84", "8C", "8D", "8E", "94",
        "95", "9A", "9C", "9F02", "9F03", "9F06", "9F09", "9F10", "9F1A", "9F1E", "9F26", "9F27",
        "9F33", "9F34", "9F35", "9F36", "9F37",
    ],
    mandatory_tags: &[
        "9F26", // Application Cryptogram
        "9F27", // CID
        "9F10", // IAD
        "9F36", // ATC
        "9F37", // Unpredictable Number
        "95",   // TVR
        "9A",   // Transaction Date
        "9C",   // Transaction Type
        "9F02", // Amount Authorized
        "5F2A", // Transaction Currency
        "9F1A", // Terminal Country Code
    ],
};

#[allow(dead_code)]
pub static PURCHASE_PROFILE: IsoMessageProfile = IsoMessageProfile {
    name: "PURCHASE",
    mti: "0200",
    tr_type: "00",
    required_fields: &[2, 3, 4, 11, 12, 13, 14, 22, 23, 25, 35, 41, 42, 49, 55],
    optional_fields: &[32, 37, 38, 39, 43, 52, 54],
    emv_profile: Some(&PURCHASE_EMV_PROFILE),
};

// ========================================
// CASH WITHDRAWAL PROFILE
// ========================================
#[allow(dead_code)]
pub static CASH_WITHDRAWAL_EMV_PROFILE: EmvProfile = EmvProfile {
    allowed_tags: &[
        "4F", "57", "5A", "5F24", "5F2A", "5F34", "82", "84", "95", "9A", "9C", "9F02", "9F10",
        "9F1A", "9F26", "9F27", "9F33", "9F34", "9F35", "9F36", "9F37",
    ],
    mandatory_tags: &[
        "9F26", "9F27", "9F10", "9F36", "9F37", "95", "9A", "9C", "9F02", "5F2A", "9F1A",
    ],
};

#[allow(dead_code)]
pub static CASH_WITHDRAWAL_PROFILE: IsoMessageProfile = IsoMessageProfile {
    name: "CASH_WITHDRAWAL",
    mti: "0200",
    tr_type: "01",
    required_fields: &[2, 3, 4, 11, 12, 13, 14, 22, 23, 25, 35, 41, 42, 49, 52, 55],
    optional_fields: &[32, 37, 38, 39, 43, 54],
    emv_profile: Some(&CASH_WITHDRAWAL_EMV_PROFILE),
};

// ========================================
// BALANCE INQUIRY PROFILE
// ========================================
#[allow(dead_code)]
pub static BALANCE_INQUIRY_PROFILE: IsoMessageProfile = IsoMessageProfile {
    name: "BALANCE_INQUIRY",
    mti: "0200",
    tr_type: "31",
    required_fields: &[2, 3, 11, 12, 13, 14, 22, 35, 41, 42, 49],
    optional_fields: &[23, 25, 32, 37, 38, 39, 43, 52, 54, 55],
    emv_profile: None,
};

// ========================================
// REFUND PROFILE
// ========================================
#[allow(dead_code)]
pub static REFUND_PROFILE: IsoMessageProfile = IsoMessageProfile {
    name: "REFUND",
    mti: "0200",
    tr_type: "20",
    required_fields: &[2, 3, 4, 11, 12, 13, 14, 22, 25, 35, 37, 41, 42, 49],
    optional_fields: &[23, 32, 38, 39, 43, 55],
    emv_profile: None,
};

// ========================================
// PRE-AUTH PROFILE
// ========================================
#[allow(dead_code)]
pub static PREAUTH_EMV_PROFILE: EmvProfile = EmvProfile {
    allowed_tags: &[
        "4F", "57", "5A", "5F24", "5F2A", "5F34", "82", "84", "95", "9A", "9C", "9F02", "9F10",
        "9F1A", "9F26", "9F27", "9F33", "9F34", "9F35", "9F36", "9F37",
    ],
    mandatory_tags: &[
        "9F26", "9F27", "9F10", "9F36", "9F37", "95", "9A", "9C", "9F02", "5F2A", "9F1A",
    ],
};

#[allow(dead_code)]
pub static PREAUTH_PROFILE: IsoMessageProfile = IsoMessageProfile {
    name: "PRE_AUTH",
    mti: "0100",
    tr_type: "00",
    required_fields: &[2, 3, 4, 11, 12, 13, 14, 22, 23, 25, 35, 41, 42, 49, 55],
    optional_fields: &[32, 37, 38, 39, 43, 52, 54],
    emv_profile: Some(&PREAUTH_EMV_PROFILE),
};

// ========================================
// VOID/REVERSAL PROFILE
// ========================================
#[allow(dead_code)]
pub static VOID_PROFILE: IsoMessageProfile = IsoMessageProfile {
    name: "VOID",
    mti: "0400",
    tr_type: "00",
    required_fields: &[2, 3, 4, 11, 12, 13, 22, 25, 37, 38, 41, 42, 49],
    optional_fields: &[14, 23, 32, 35, 39, 43, 55],
    emv_profile: None,
};

// ========================================
// QR PAYMENT PROFILE
// ========================================
#[allow(dead_code)]
pub static QR_PAYMENT_PROFILE: IsoMessageProfile = IsoMessageProfile {
    name: "QR_PAYMENT",
    mti: "0200",
    tr_type: "00",
    required_fields: &[3, 4, 11, 12, 13, 25, 41, 42, 49],
    optional_fields: &[2, 32, 37, 38, 39, 43, 102, 103],
    emv_profile: None,
};

/// Get profile by transaction type string
#[allow(dead_code)]
pub fn get_profile_by_type(tr_type: &str) -> Option<&'static IsoMessageProfile> {
    match tr_type.to_uppercase().as_str() {
        "PURCHASE" | "00" => Some(&PURCHASE_PROFILE),
        "CASH_WITHDRAWAL" | "WITHDRAWAL" | "01" => Some(&CASH_WITHDRAWAL_PROFILE),
        "BALANCE_INQUIRY" | "BALANCE" | "31" => Some(&BALANCE_INQUIRY_PROFILE),
        "REFUND" | "RETURN" | "20" => Some(&REFUND_PROFILE),
        "PRE_AUTH" | "PREAUTH" | "AUTH" => Some(&PREAUTH_PROFILE),
        "VOID" | "REVERSAL" | "CANCEL" => Some(&VOID_PROFILE),
        "QR_PAYMENT" | "QR" | "VIETQR" => Some(&QR_PAYMENT_PROFILE),
        _ => None,
    }
}

/// List of all available profiles
#[allow(dead_code)]
pub static ALL_PROFILES: &[&IsoMessageProfile] = &[
    &PURCHASE_PROFILE,
    &CASH_WITHDRAWAL_PROFILE,
    &BALANCE_INQUIRY_PROFILE,
    &REFUND_PROFILE,
    &PREAUTH_PROFILE,
    &VOID_PROFILE,
    &QR_PAYMENT_PROFILE,
];
