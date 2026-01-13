use serde::{Deserialize, Serialize};

/// Represents the incoming TCP message from terminal
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CardRequest {
    pub msg_type: String,
    pub trm_id: String,
    pub transaction_id: String,
    pub amount: f64,
    pub transaction_type: String,
    #[serde(default)]
    pub merchant_id: Option<String>,
    #[serde(default)]
    pub card_data: Option<String>, // This is a JSON string that needs to be parsed
    #[serde(default)]
    pub qr_data: Option<String>,
    #[serde(default)]
    pub additional_data: Option<String>,
}

/// Parsed card data from the cardData field
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CardData {
    pub emv_data: EmvData,
}

/// EMV data containing DE55 TLV
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmvData {
    pub de55: String,
    pub de55_length: Option<u32>,
}

impl CardRequest {
    /// Parse the cardData JSON string into CardData struct
    pub fn parse_card_data(&self) -> Result<Option<CardData>, serde_json::Error> {
        match &self.card_data {
            Some(card_data_str) => {
                let parsed: CardData = serde_json::from_str(card_data_str)?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }

    /// Get DE55 hex string from nested cardData
    pub fn get_de55(&self) -> Result<Option<String>, serde_json::Error> {
        match self.parse_card_data()? {
            Some(card_data) => Ok(Some(card_data.emv_data.de55)),
            None => Ok(None),
        }
    }

    /// Get card data as string for processing
    pub fn get_card_data_string(&self) -> Result<Option<String>, serde_json::Error> {
        self.get_de55()
    }
}

