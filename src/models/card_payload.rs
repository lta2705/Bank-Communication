use serde::{Serialize, Deserialize};
use zeroize;

#[derive(Serialize,Deserialize,zeroize)]
#[zeroize(drop)]
pub struct Card {
    pub pan: String,
    pub exp_month: u8,
    pub exp_year: u16,
    pub cvv: String,
    pub cardholder_name: Option<String>
}

