pub mod error;
pub mod resp_structs;

// Re-export commonly used types for convenience
pub use resp_structs::{TokenList, QuoteResponse, TransactionInfo, TxType};
pub use error::FogswapSdkError;

use std::collections::HashMap;
use reqwest::Client;
use serde_json::{json, Value};
use anyhow::Result;



#[derive(Debug, Clone)]
pub struct FogswapSdk {
    pub base_url: String,
    pub client: Client,
}

impl FogswapSdk {

    const BASE_URL: &str = "https://api.fogswap.io/v1";
    
    /// Create a new FogswapSdk instance
    /// # Examples
    /// ```
    /// use fogswap_sdk_rust::FogswapSdk;
    /// 
    /// let sdk = FogswapSdk::new();
    /// ```
    pub fn new() -> Self {
        let client = Client::builder()
            .build()
            .unwrap_or_default();
        Self { base_url: Self::BASE_URL.to_string(), client }
    }

    /// Send a request to the Fogswap API
    async fn send_request(
        &self,
        req_method: reqwest::Method,
        endpoint: &str,
        payload: Option<Value>,
    ) -> Result<Value> {

        let url = format!("{}{}", self.base_url, endpoint);
        
        let resp={
            match req_method {
                reqwest::Method::GET => {
                    match payload {
                        Some(payload) => {
                            let params: HashMap<&String, &Value> = payload
                                .as_object()
                                .unwrap()
                                .iter()
                                .flat_map(|(k, v)| {
                                    if v.is_null() {
                                        None
                                    } else {
                                        Some((k, v))
                                    }
                                })
                                .collect();
                            self.client.get(url).query(&params).send().await?
                        }
                        _=> self.client.get(url).send().await?
                    }
                },
                reqwest::Method::POST => {
                    match payload {
                        Some(payload) => {
                            self.client.post(url).header("Content-Type", "application/json").json(&payload).send().await?
                        }
                        _=> self.client.post(url).header("Content-Type", "application/json").send().await?
                    }   
                },
                _ => return Err(FogswapSdkError::UnsupportedMethod.into()),
            }
        };

        if resp.status() != 200 {   
            return Err(FogswapSdkError::SendRequestError.into());
        }

        let body = resp.json::<Value>().await?;
        Ok(body)

    }

    /// Get the list of available tokens
    /// # Returns
    /// * `Vec<TokenList>` - A vector of token lists grouped by network
    /// # Errors
    /// * `FogswapSdkError::GetAvailableCoinsError` - If the token list cannot be retrieved
    /// # Examples
    /// ```
    /// use fogswap_sdk_rust::FogswapSdk;
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let sdk = FogswapSdk::new();
    /// let tokens = sdk.get_token_list().await?;
    /// println!("Found {} networks", tokens.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_token_list(&self) -> Result<Vec<TokenList>> {
        let endpoint = "/market/tokens";

        let resp = self.send_request(reqwest::Method::GET, endpoint, None).await?;

        if let Some(e) = resp.get("error").unwrap().as_object() {
            let e=e.get("message").unwrap().as_str().unwrap();
            return Err(FogswapSdkError::GetAvailableCoinsError(e.to_string()).into());
        }
        
        let resp=resp.get("result").unwrap();
        let coins=serde_json::from_value::<Vec<TokenList>>(resp.to_owned())?;
        Ok(coins)
    }

    /// Get the quote for an swap
    /// # Arguments
    /// * `amount_from` - The amount of the token to swap
    /// * `network_from` - The network of the token to swap
    /// * `contract_address_from` - The contract address of the token to swap
    /// * `network_to` - The network of the token to swap
    /// * `contract_address_to` - The contract address of the token to swap
    /// * `tx_type` - The type of the transaction
    /// * `is_use_xmr` - Whether to use XMR for the transaction
    /// # Returns
    /// * `QuoteResponse` - The quote for the swap
    /// # Errors
    /// * `FogswapSdkError::GetEstimatedExchangeAmountError` - If the quote for the swap is not found
    /// # Examples
    /// ```
    /// use fogswap_sdk_rust::{FogswapSdk, TxType};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let sdk = FogswapSdk::new();
    /// let quote = sdk.get_quote(
    ///     1.0, 
    ///     "sol", 
    ///     "SOL", 
    ///     "sol", 
    ///     "SOL", 
    ///     Some(TxType::Private), 
    ///     Some(true)
    /// ).await?;
    /// println!("Amount to receive: {}", quote.amount_to);
    /// # Ok(())
    /// # }
    /// ```
    /// # Panics
    /// * If the quote for the swap is not found
    /// * If the request to the Fogswap API fails
    /// * If the response from the Fogswap API is not valid
    /// * If the response from the Fogswap API is not valid
    pub async fn get_quote(
        &self,
        amount_from: f64,
        network_from: &str,
        contract_address_from: &str,
        network_to: &str,
        contract_address_to: &str,
        tx_type: Option<TxType>,
        is_use_xmr: Option<bool>
    ) -> Result<QuoteResponse> {

        let endpoint = "/transaction/quote";

        let resp=self.send_request(
            reqwest::Method::GET, 
            endpoint, 
            Some(json!({
                "amount_from": amount_from,
                "network_from": network_from,
                "contract_address_from": contract_address_from,
                "network_to": network_to,
                "contract_address_to": contract_address_to,
                "tx_type": tx_type,
                "is_use_xmr":is_use_xmr,
            })),
        ).await?;

        if let Some(e) = resp.get("error").unwrap().as_object() {
            let e=e.get("message").unwrap().as_str().unwrap();
            return Err(FogswapSdkError::GetEstimatedExchangeAmountError(e.to_string()).into());
        }
        let resp=resp.get("result").unwrap();
        let estimated_exchange_amount=serde_json::from_value::<QuoteResponse>(resp.to_owned())?;
        Ok(estimated_exchange_amount)
   
    }

    /// Create a new transaction
    /// # Arguments
    /// * `network_from` - The network of the token to swap
    /// * `contract_address_from` - The contract address of the token to swap
    /// * `network_to` - The network of the token to swap
    /// * `contract_address_to` - The contract address of the token to swap
    /// * `amount_from` - The amount of the token to swap
    /// * `payout_address` - The address to receive the tokens
    /// * `payout_extra_id` - The extra id for the payout
    /// * `tx_type` - The type of the transaction
    /// * `is_use_xmr` - Whether to use XMR for the transaction
    /// # Returns
    /// * `TransactionInfo` - The information about the transaction
    /// # Errors
    /// * `FogswapSdkError::CreateTransactionError` - If the transaction is not created
    /// # Examples
    /// ```
    /// use fogswap_sdk_rust::{FogswapSdk, TxType};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let sdk = FogswapSdk::new();
    /// let tx_info = sdk.create_transaction(
    ///     "sol", 
    ///     "SOL", 
    ///     "sol",  
    ///     "SOL", 
    ///     0.5,
    ///     "ARBmhGy4ydx7ouUrLGhgsNJyDMBL25AuKWeh311k8cBP",
    ///     &None,
    ///     Some(TxType::Private), 
    ///     Some(true)
    /// ).await?;
    /// println!("Transaction ID: {}", tx_info.id);
    /// println!("Payin address: {}", tx_info.payin_address);
    /// # Ok(())
    /// # }
    /// ```
    /// # Panics
    /// * If the transaction is not created
    /// * If the request to the Fogswap API fails
    /// * If the response from the Fogswap API is not valid
    /// * If the response from the Fogswap API is not valid
    pub async fn create_transaction(
        &self,
        network_from: &str,
        contract_address_from: &str,
        network_to: &str,
        contract_address_to: &str,
        amount_from: f64,
        payout_address: &str,
        payout_extra_id: &Option<String>,
        tx_type: Option<TxType>,
        is_use_xmr: Option<bool>
    ) -> Result<TransactionInfo> {
        let endpoint = "/transaction/create";

        let resp=self.send_request(
            reqwest::Method::POST, 
            endpoint, 
            Some(json!({
                "network_from": network_from,
                "contract_address_from": contract_address_from,
                "amount_from": amount_from,
                "network_to": network_to,
                "contract_address_to": contract_address_to,
                "payout_address": payout_address,
                "payout_extra_id": payout_extra_id,
                "tx_type": tx_type,
                "is_use_xmr": is_use_xmr,
            }))
        ).await?;

        if let Some(e) = resp.get("error").unwrap().as_object() {
            let e=e.get("message").unwrap().as_str().unwrap();
            return Err(FogswapSdkError::CreateTransactionError(e.to_string()).into());
        }

        let resp=resp.get("result").unwrap();
        let tx_info=serde_json::from_value::<TransactionInfo>(resp.to_owned());
        Ok(tx_info?)
    }

    /// Get the information about a transaction
    /// # Arguments
    /// * `id` - The id of the transaction
    /// # Returns
    /// * `TransactionInfo` - The information about the transaction
    /// # Errors
    /// * `FogswapSdkError::GetTransactionInfoError` - If the transaction information is not found
    /// # Examples
    /// ```
    /// use fogswap_sdk_rust::FogswapSdk;
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let sdk = FogswapSdk::new();
    /// let tx_info = sdk.get_transaction_info("S7ZulO3j16").await?;
    /// println!("Transaction status: {}", tx_info.status);
    /// println!("Amount from: {}", tx_info.amount_from);
    /// println!("Amount to: {}", tx_info.amount_to);
    /// # Ok(())
    /// # }
    /// ```
    /// # Panics
    /// * If the transaction information is not found
    /// * If the request to the Fogswap API fails
    /// * If the response from the Fogswap API is not valid
    pub async fn get_transaction_info(
        &self,
        id: &str
    ) -> Result<TransactionInfo> {
        let endpoint = "/transaction/info";

        let resp=self.send_request(
            reqwest::Method::GET, 
            endpoint, 
            Some(json!({
                "tx_id": id
            }))
        ).await?;

        if let Some(e) = resp.get("error").unwrap().as_object() {
            let e=e.get("message").unwrap().as_str().unwrap();
            return Err(FogswapSdkError::GetTransactionInfoError(e.to_string()).into());
        }

        let resp=resp.get("result").unwrap();
        let tx_info=serde_json::from_value::<TransactionInfo>(resp.to_owned())?;
        Ok(tx_info)
    }

}
