use std::str::FromStr;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct TokenList{
    pub network: String,
    pub network_image: String,
    pub tokens: Vec<TokenInfo>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct TokenInfo {
    pub token: String,
    pub network: String,
    pub contract_address: String,
    pub image: String,
    pub is_native: bool,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct QuoteResponse {
    pub network_from: String,
    pub contract_address_from: String,
    pub amount_from: f64,
    pub network_to: String,
    pub contract_address_to: String,
    pub amount_to: f64,
    pub convert_usd: ConvertUsd,
    pub tx_type: TxType,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ConvertUsd {
    pub from: Option<f64>,
    pub to: Option<f64>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub id: String,
    pub created_at: i64,
    pub tx_type: TxType,

    pub network_from: String,
    pub contract_address_from: String,
    
    pub contract_address_to: String,
    pub network_to: String,

    pub amount_from: f64,
    pub amount_to: f64,

    pub payin_address: String,
    pub payin_extra_id: Option<String>,
    pub payin_hash: Option<String>,

    pub payout_address: String,
    pub payout_extra_id: Option<String>,
    pub payout_hash: Option<String>,

    pub convert_usd: Option<f64>,
    
    pub status: String,
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum TxType {
    Standard,
    Private,
}

impl ToString for TxType {
    fn to_string(&self) -> String {
        match self {
            TxType::Standard => "standard".to_string(),
            TxType::Private => "private".to_string(),
        }
    }
}

impl FromStr for TxType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(TxType::Standard),
            "private" => Ok(TxType::Private),
            _ => Err(anyhow::anyhow!("Invalid tx type")),
        }
    }
}