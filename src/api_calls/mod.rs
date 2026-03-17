// src/api_calls/mod.rs

use crate::models;
use crate::utils;
use reqwest::blocking::Client;
use colored::*;

// 
// get_all_swap_related_txns
// AccountType: Token or Account
// 'https://solana-gateway.moralis.io/account/mainnet/{walletAdd}/swaps?limit=25&order=DESC&transactionTypes=buy%2Csell' \
// 'https://solana-gateway.moralis.io/token/mainnet/{tokenAdd}/swaps?limit=100&order=DESC&transactionTypes=buy%2Csell' \

pub fn fetch_swap_related_txns(addr: &str, api_key: &str, account_type: models::AccountType) -> Result<models::AllSwapRelatedTxns, reqwest::Error> {
    let _prompt_text: String = format!("fetching swap related transactions for {}", addr);
    println!("{}", _prompt_text.green());
    
    let request_url: String = format!("https://solana-gateway.moralis.io/{}/mainnet/{}/swaps?limit=100&order=DESC&transactionTypes=buy%2Csell", account_type, addr);
    println!("{}", request_url.green());
        let client = Client::new();
    let response = client
        .get(&request_url)
        .header("X-API-Key", api_key)
        .header("accept", "application/json")
        .send()?;
    utils::display_reponse_status(response.status().as_str());
    let response_text:models::AllSwapRelatedTxns = response.json::<models::AllSwapRelatedTxns>()?;
    Ok(response_text)
}

// curl -X 'GET' \
//   'https://solana-gateway.moralis.io/token/mainnet/So11111111111111111111111111111111111111112/pairs?limit=50' \
//   -H 'accept: application/json' \
//   -H 'X-Api-Key: grtyddertgvfdrtgfder'
pub fn get_token_pair_by_address(token_addr: &str, api_key: &str) -> Result<models::TokenPairByAddress, reqwest::Error> {
    let _prompt_text: String = format!("fetching pair information for token: {}", token_addr);
    println!("{}", _prompt_text.green());
    let request_url: String = format!("https://solana-gateway.moralis.io/token/mainnet/{}/pairs?limit=50", token_addr);
        let client = Client::new();
    let response = client
        .get(&request_url)
        .header("X-API-Key", api_key)
        .header("accept", "application/json")
        .send()?;
    utils::display_reponse_status(response.status().as_str());
    let response_text:models::TokenPairByAddress = response.json::<models::TokenPairByAddress>()?;
    // println!("Response text: {:?}", response_text);
    Ok(response_text)
}