use std::ptr::null;

use std::io;
use tracing::{info, debug};

pub async fn handle_message(raw_emv: &str) -> io::Result<Vec<u8>> {
    info!("data:{:?} ", raw_emv);
    // parse_tlv_vec(raw_emv)
    // build ISO 8583
    // send to bank
    // receive response
    // map response → terminal response
    Ok(Vec::<u8>::new()) // Resolve this
}
