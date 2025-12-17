use thiserror::Error;

#[derive(Debug, Error)]
pub enum CardError {
    #[error("invalid last4 digits")]
    InvalidLast4,

    #[error("invalid card bin")]
    InvalidBin,

    #[error("invalid expiration date")]
    InvalidExpiry,
}