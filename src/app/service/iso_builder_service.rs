use std::{collections::HashMap, iter::Map};

#[derive(Debug, Copy, Clone)]
pub enum IsoMsgType {
    Purchase, // 0200
    PurchaseAdvice, // 0220
    Void, // 0200 with another processing code
    Reversal, // 0400
    Network, // 0800
}

impl IsoMsgType {
    pub fn make_mti(transaction_type: &str) -> String {
        match transaction_type {
            "SALE" => "0200".to_string(),
            "VOID" => "0200".to_string(),
            "REVERSAL" => "0400".to_string(),
            "NETWORK" => "0800".to_string(),
            _ => "0000".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Iso8583Message {
    pub mti: String,
    pub bitmap: String,
    pub fields: HashMap<u8, String>,
}

impl Iso8583Message {
    pub fn new() -> Self {
        Iso8583Message {
            mti: String::new(),
            bitmap: String::new(),
            fields: HashMap::new(),
        }
    }
}