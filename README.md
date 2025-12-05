# FogSwap SDK Rust

A Rust implementation of the FogSwap SDK, providing a convenient interface for interacting with the FogSwap API. Supports token swapping, quote queries, transaction creation and querying, and more.

## Features

- 🔄 **Token Swapping** - Support for cross-chain token swapping
- 💰 **Quote Queries** - Get real-time swap quotes
- 📋 **Token Lists** - Get lists of supported tokens and networks
- 🔒 **Private Transactions** - Support for both standard and private transactions
- 📊 **Transaction Queries** - Query transaction status and details
- 🛡️ **Error Handling** - Comprehensive error handling mechanism


## Quick Start

### Basic Usage

```rust
use fogswap_sdk_rust::FogswapSdk;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create SDK instance
    let sdk = FogswapSdk::new();
    
    // Get token list
    let tokens = sdk.get_token_list().await?;
    println!("Number of supported networks: {}", tokens.len());
    
    Ok(())
}
```

## API Documentation

### FogswapSdk

The main SDK struct that provides all API methods.

#### Creating an Instance

```rust
use fogswap_sdk_rust::FogswapSdk;

let sdk = FogswapSdk::new();
```

### Methods

#### `get_token_list()`

Get a list of all supported tokens, grouped by network.

**Returns**: `Result<Vec<TokenList>>`

**Example**:

```rust
use fogswap_sdk_rust::FogswapSdk;

let sdk = FogswapSdk::new();
let tokens = sdk.get_token_list().await?;

for token_list in tokens {
    println!("Network: {}", token_list.network);
    for token in token_list.tokens {
        println!("  - {} ({})", token.token, token.contract_address);
    }
}
```

#### `get_quote()`

Get a quote for a token swap.

**Parameters**:
- `amount_from: f64` - The amount of tokens to swap
- `network_from: &str` - Source network (e.g., "sol", "eth")
- `contract_address_from: &str` - Source token contract address (use "SOL", "ETH" for native tokens)
- `network_to: &str` - Target network
- `contract_address_to: &str` - Target token contract address
- `tx_type: Option<TxType>` - Transaction type (`Standard` or `Private`)
- `is_use_xmr: Option<bool>` - Whether to use XMR

**Returns**: `Result<QuoteResponse>`

**Example**:

```rust
use fogswap_sdk_rust::{FogswapSdk, TxType};

let sdk = FogswapSdk::new();
let quote = sdk.get_quote(
    1.0,           // Swap 1.0 tokens
    "sol",         // From Solana network
    "SOL",         // SOL token
    "sol",         // To Solana network
    "SOL",         // SOL token
    Some(TxType::Private),  // Use private transaction
    Some(true)     // Use XMR
).await?;

println!("Will receive: {} {}", quote.amount_to, quote.contract_address_to);
println!("USD value: ${:?}", quote.convert_usd);
```

#### `create_transaction()`

Create a new swap transaction.

**Parameters**:
- `network_from: &str` - Source network
- `contract_address_from: &str` - Source token contract address
- `network_to: &str` - Target network
- `contract_address_to: &str` - Target token contract address
- `amount_from: f64` - The amount of tokens to swap
- `payout_address: &str` - Address to receive tokens
- `payout_extra_id: &Option<String>` - Additional payout ID (required for some networks)
- `tx_type: Option<TxType>` - Transaction type
- `is_use_xmr: Option<bool>` - Whether to use XMR

**Returns**: `Result<TransactionInfo>`

**Example**:

```rust
use fogswap_sdk_rust::{FogswapSdk, TxType};

let sdk = FogswapSdk::new();
let tx_info = sdk.create_transaction(
    "sol",
    "SOL",
    "sol",
    "SOL",
    0.5,                                    // Swap 0.5 tokens
    "YOUR_RECEIVE_ADDRESS_HERE",  // Receive address
    &None,                                  // No extra ID
    Some(TxType::Private),
    Some(true)
).await?;

println!("Transaction ID: {}", tx_info.id);
println!("Payin address: {}", tx_info.payin_address);
println!("Status: {}", tx_info.status);
```

#### `get_transaction_info()`

Query transaction information by transaction ID.

**Parameters**:
- `id: &str` - Transaction ID

**Returns**: `Result<TransactionInfo>`

**Example**:

```rust
use fogswap_sdk_rust::FogswapSdk;

let sdk = FogswapSdk::new();
let tx_info = sdk.get_transaction_info("S7ZulO3j16").await?;

println!("Transaction status: {}", tx_info.status);
println!("Send {} from {}", tx_info.amount_from, tx_info.network_from);
println!("Receive {} to {}", tx_info.amount_to, tx_info.network_to);

if let Some(payin_hash) = &tx_info.payin_hash {
    println!("Payin hash: {}", payin_hash);
}

if let Some(payout_hash) = &tx_info.payout_hash {
    println!("Payout hash: {}", payout_hash);
}
```

## Type Reference

### TokenList

Token list grouped by network.

```rust
pub struct TokenList {
    pub network: String,           // Network name
    pub network_image: String,     // Network icon URL
    pub tokens: Vec<TokenInfo>,    // List of tokens on this network
}
```

### TokenInfo

Individual token information.

```rust
pub struct TokenInfo {
    pub token: String,              // Token name
    pub network: String,            // Network it belongs to
    pub contract_address: String,   // Contract address
    pub image: String,              // Token icon URL
    pub is_native: bool,            // Whether it's a native token
}
```

### QuoteResponse

Quote response.

```rust
pub struct QuoteResponse {
    pub network_from: String,
    pub contract_address_from: String,
    pub amount_from: f64,
    pub network_to: String,
    pub contract_address_to: String,
    pub amount_to: f64,
    pub convert_usd: ConvertUsd,   // USD value conversion
    pub tx_type: TxType,
}
```

### TransactionInfo

Transaction information.

```rust
pub struct TransactionInfo {
    pub id: String,                    // Transaction ID
    pub created_at: i64,                // Creation timestamp
    pub tx_type: TxType,                // Transaction type
    pub network_from: String,           // Source network
    pub contract_address_from: String,  // Source token contract address
    pub contract_address_to: String,    // Target token contract address
    pub network_to: String,             // Target network
    pub amount_from: f64,               // Amount to send
    pub amount_to: f64,                 // Amount to receive
    pub payin_address: String,         // Payin address
    pub payin_extra_id: Option<String>, // Payin extra ID
    pub payin_hash: Option<String>,     // Payin transaction hash
    pub payout_address: String,         // Payout address
    pub payout_extra_id: Option<String>, // Payout extra ID
    pub payout_hash: Option<String>,     // Payout transaction hash
    pub convert_usd: Option<f64>,        // USD value
    pub status: String,                  // Transaction status
}
```

### TxType

Transaction type enumeration.

```rust
pub enum TxType {
    Standard,  // Standard transaction
    Private,   // Private transaction
}
```

## Error Handling

The SDK uses `anyhow::Result` for error handling. All methods return `Result<T, anyhow::Error>`.

### FogswapSdkError

SDK-defined error types:

```rust
pub enum FogswapSdkError {
    UnsupportedMethod,                          // Unsupported method
    SendRequestError,                           // Request sending error
    GetAvailableCoinsError(String),            // Get token list error
    GetEstimatedExchangeAmountError(String),   // Get quote error
    CreateTransactionError(String),            // Create transaction error
    GetTransactionInfoError(String),          // Get transaction info error
}
```

### Error Handling Example

```rust
use fogswap_sdk_rust::{FogswapSdk, FogswapSdkError};

match sdk.get_token_list().await {
    Ok(tokens) => {
        println!("Successfully retrieved {} networks", tokens.len());
    }
    Err(e) => {
        eprintln!("Failed to get token list: {}", e);
        // Can further handle specific errors
        if let Some(sdk_error) = e.downcast_ref::<FogswapSdkError>() {
            match sdk_error {
                FogswapSdkError::GetAvailableCoinsError(msg) => {
                    eprintln!("API error: {}", msg);
                }
                _ => {}
            }
        }
    }
}
```

## Complete Example

### Complete Swap Flow

```rust
use fogswap_sdk_rust::{FogswapSdk, TxType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = FogswapSdk::new();
    
    // 1. Get token list
    println!("Getting token list...");
    let tokens = sdk.get_token_list().await?;
    println!("Found {} networks\n", tokens.len());
    
    // 2. Get quote
    println!("Getting swap quote...");
    let quote = sdk.get_quote(
        1.0,
        "sol",
        "SOL",
        "sol",
        "SOL",
        Some(TxType::Private),
        Some(true)
    ).await?;
    
    println!("Quote details:");
    println!("  Send: {} {}", quote.amount_from, quote.contract_address_from);
    println!("  Receive: {} {}", quote.amount_to, quote.contract_address_to);
    println!("  USD value: ${:?}\n", quote.convert_usd);
    
    // 3. Create transaction
    println!("Creating transaction...");
    let tx_info = sdk.create_transaction(
        "sol",
        "SOL",
        "sol",
        "SOL",
        1.0,
        "YOUR_RECEIVE_ADDRESS_HERE",
        &None,
        Some(TxType::Private),
        Some(true)
    ).await?;
    
    println!("Transaction created:");
    println!("  Transaction ID: {}", tx_info.id);
    println!("  Payin address: {}", tx_info.payin_address);
    println!("  Status: {}\n", tx_info.status);
    
    // 4. Query transaction status
    println!("Querying transaction status...");
    let tx_status = sdk.get_transaction_info(&tx_info.id).await?;
    println!("Current status: {}", tx_status.status);
    
    Ok(())
}
```


## Dependencies

- `tokio` - Async runtime
- `reqwest` - HTTP client
- `serde` / `serde_json` - JSON serialization/deserialization
- `anyhow` - Error handling
- `thiserror` - Error type definitions


## Contributing

Issues and Pull Requests are welcome!

## Related Links

- [FogSwap Website](https://www.fogswap.com)
- [Cooperate](mailto:partner@fogswap.com)
