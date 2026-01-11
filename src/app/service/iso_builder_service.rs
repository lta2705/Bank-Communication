use std::{collections::HashMap, iter::Map};

use crate::app::service::transaction_profile::TransactionType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TcpTransactionType {
    Sale,
    Void,
    Reversal,
    Qr,
}

impl TryFrom<&str> for TcpTransactionType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "SALE" => Ok(TcpTransactionType::Sale),
            "VOID" => Ok(TcpTransactionType::Void),
            "REVERSAL" => Ok(TcpTransactionType::Reversal),
            "QR" => Ok(TcpTransactionType::Qr),
            _ => Err(format!("Unsupported TCP transactionType: {}", value)),
        }
    }
}

impl TcpTransactionType {
    pub fn to_internal(self) -> TransactionType {
        match self {
            TcpTransactionType::Sale => TransactionType::Purchase,
            TcpTransactionType::Void => TransactionType::Void,
            TcpTransactionType::Reversal => TransactionType::Reversal,
            TcpTransactionType::Qr => TransactionType::QrPayment,
        }
    }
}