use serde::{Deserialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountType {
    Account,
    Token,
}

#[derive(Deserialize, Debug)]
pub struct TokenPairByAddress {
    pub pairs: Vec<Pairs>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pairs {
    pub exchange_address: String,
    pub pair_address: String,
    pub pair: Vec<Pair>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub token_address: String,
    pub token_name: String,
    pub pair_token_type: String,
    pub token_symbol: String
}

#[derive(Deserialize, Debug)]
pub struct RefinedTokenPairInfo {
    pub exchange_address: String,
    pub pair_address: String,
    pub token0_address: String,
    pub token0_name: String,
    pub token0_symbol: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AllSwapRelatedTxns{
    pub cursor: Option<String>,
    pub page_size: u32,
    pub page: u32,
    pub result: Vec<SwapResult>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwapResult {
    pub transaction_hash: String,
    pub wallet_address: String,
    pub block_timestamp: String,
    pub pair_address: String,
    pub exchange_address: String,
    pub bought: Bought,
    pub sold: Sold,
    pub total_value_usd: f64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bought {
    pub address: String,
    pub symbol: String,
    pub amount: String,
    pub usd_price: f64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sold {
    pub address: String,
    pub symbol: String,
    pub amount: String,
    pub usd_price: f64,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Token => write!(f, "token"),
            AccountType::Account => write!(f, "account"),
        }
    }
}
