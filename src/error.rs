use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum FogswapSdkError {
    
    #[error("Unsupported method")]
    UnsupportedMethod,

    #[error("send request error")]
    SendRequestError,

    #[error("Get Available Coins Error : {0}")]
    GetAvailableCoinsError(String),

    #[error("Get Estimated Exchange Amount Error : {0}")]
    GetEstimatedExchangeAmountError(String),

    #[error("Create Transaction Error : {0}")]
    CreateTransactionError(String),

    #[error("Get Transaction Info Error : {0}")]
    GetTransactionInfoError(String),

}