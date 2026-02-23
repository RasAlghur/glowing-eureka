use dotenv::dotenv;
use std::env;
use std::io::stdin;
use serde::{Deserialize};
use anyhow::Result;
use colored::*;
use reqwest::blocking::Client;

#[derive(Deserialize, Debug)]
struct TokenPairByAddress {
    pairs: Vec<Pairs>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Pairs {
    exchange_address: String,
    pair_address: String,
    pair: Vec<Pair>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Pair {
    token_address: String,
    token_name: String,
    pair_token_type: String,
    token_symbol: String
}

#[derive(Deserialize, Debug)]
struct RefinedTokenPairInfo {
    exchange_address: String,
    pair_address: String,
    token0_address: String,
    token0_name: String,
    token0_symbol: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AllSwapRelatedTxnsForToken {
    cursor: String,
    page_size: u32,
    page: u32,
    result: Vec<SwapResult>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct SwapResult {
    transaction_hash: String,
    wallet_address: String,
    block_timestamp: String,
    pair_address: String,
    exchange_address: String,
    bought: Bought,
    sold: Sold,
    total_value_usd: f64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Bought {
    address: String,
    symbol: String,
    amount: String,
    usd_price: f64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Sold {
    address: String,
    symbol: String,
    amount: String,
    usd_price: f64,
}

// curl -X 'GET' \
//   'https://solana-gateway.moralis.io/token/mainnet/So11111111111111111111111111111111111111112/pairs?limit=50' \
//   -H 'accept: application/json' \
//   -H 'X-Api-Key: grtyddertgvfdrtgfder'
fn get_token_pair_by_address(token_addr: &str, api_key: &str) -> Result<TokenPairByAddress, reqwest::Error> {
    let _prompt_text: String = format!("fetching pair information for token: {}", token_addr);
    println!("{}", _prompt_text.green());
    let request_url: String = format!("https://solana-gateway.moralis.io/token/mainnet/{}/pairs?limit=50", token_addr);
        let client = Client::new();
    let response = client
        .get(&request_url)
        .header("X-API-Key", api_key)
        .header("accept", "application/json")
        .send()?;
    display_reponse_status(response.status().as_str());
    let response_text:TokenPairByAddress = response.json::<TokenPairByAddress>()?;
    // println!("Response text: {:?}", response_text);
    Ok(response_text)
}

fn display_reponse_status (response_status: &str) {
    let _prompt_text: String = format!("Response status: {}", response_status);
    if response_status == "200" {
        println!("{}", _prompt_text.green());
    } else {
        println!("{}", _prompt_text.red());
    }
}

fn refined_get_token_pair_by_address(response: &TokenPairByAddress) -> Result<RefinedTokenPairInfo, String> {
    println!("{}", "Token pair information:".green());

    // Get the first pair, if any
    if let Some(first_pair) = response.pairs.first() {
        println!("Exchange Address: {}", first_pair.exchange_address);
        println!("Pair Address: {}", first_pair.pair_address);

        // Find the token with pair_token_type == "token0" (adjust the string as needed)
        let token0 = first_pair.pair.iter().find(|t| t.pair_token_type == "token0");

        match token0 {
            Some(token) => {
                println!("  Token Address: {}", token.token_address);
                println!("  Token Name: {}", token.token_name);
                println!("  Token Symbol: {}", token.token_symbol);
            }
            None => println!("  No token found in this pair."),
        }
    } else {
        println!("No pairs found.");
    }

    Ok(RefinedTokenPairInfo {
        exchange_address: response.pairs.first().unwrap().exchange_address.clone(),
        pair_address: response.pairs.first().unwrap().pair_address.clone(),
        token0_address: response.pairs.first().unwrap().pair.first().unwrap().token_address.clone(),
        token0_name: response.pairs.first().unwrap().pair.first().unwrap().token_name.clone(),
        token0_symbol: response.pairs.first().unwrap().pair.first().unwrap().token_symbol.clone(),
    })
}

// curl -X 'GET' \
//   'https://solana-gateway.moralis.io/token/mainnet/So11111111111111111111111111111111111111112/swaps?limit=100&order=DESC&transactionTypes=buy%2Csell' \
//   -H 'accept: application/json' \
//    -H 'X-Api-Key: grtyddertgvfdrtgfder'
fn get_all_swap_related_txns_for_token(token_addr: &str, api_key: &str) -> Result<AllSwapRelatedTxnsForToken, reqwest::Error> {
    let _prompt_text: String = format!("fetching swap related transactions for {}", token_addr);
    println!("{}", _prompt_text.green());

    let request_url: String = format!("https://solana-gateway.moralis.io/token/mainnet/{}/swaps?limit=100&order=DESC&transactionTypes=buy%2Csell", token_addr);
        let client = Client::new();
    let response = client
        .get(&request_url)
        .header("X-API-Key", api_key)
        .header("accept", "application/json")
        .send()?;
    display_reponse_status(response.status().as_str());
    let response_text:AllSwapRelatedTxnsForToken = response.json::<AllSwapRelatedTxnsForToken>()?;
    Ok(response_text)
}

fn refined_get_all_swap_related_txns_for_token(response: &AllSwapRelatedTxnsForToken) -> Vec<SwapResult> {
    println!("{}", "Swap related transactions:".green());
    for swap in &response.result {
        println!("Transaction Hash: {}", swap.transaction_hash);
        println!("Wallet Address: {}", swap.wallet_address);
        println!("Block Timestamp: {}", swap.block_timestamp);
        println!("Pair Address: {}", swap.pair_address);
        println!("Exchange Address: {}", swap.exchange_address);
        println!("Bought: {:.2} {} (USD Price: {:.2} per {})", swap.bought.amount, swap.bought.symbol, swap.bought.usd_price, swap.bought.symbol);
        println!("Sold: {:.2} {} (USD Price: {:.2} per {})", swap.sold.amount, swap.sold.symbol, swap.sold.usd_price, swap.sold.symbol);
        println!("Total Value USD: {:.2}", swap.total_value_usd);
        println!("-----------------------------------");
    }
    response.result.clone()
}

// curl --request GET \
//      --url 'https://solana-gateway.moralis.io/account/mainnet/kXB7FfzdrfZpAZEW3TZcp8a8CwQbsowa6BdfAHZ4gVs/swaps?limit=25&order=DESC&transactionTypes=buy%2Csell' \
//      --header 'accept: application/json' \
//      --header 'X-API-Key: grtyddertgvfdrtgfder' 
fn get_all_swap_related_txns_for_wallet(wallet_addr: &str) {
    let _prompt_text: String = format!("fetching swap related transactions for wallet {}", wallet_addr);
    println!("{}", _prompt_text.green());
}

fn main() {
    dotenv().ok();

    // Read the API key
    let api_key: String = env::var("MORALIS_API_KEY")
        .expect("MORALIS_API_KEY must be set in .env or environment");

    let get_token_pair_by_address_result = get_token_pair_by_address("AVF9F4C4j8b1Kh4BmNHqybDaHgnZpJ7W7yLvL7hUpump", &api_key);
    match get_token_pair_by_address_result {
        Ok(data) => match refined_get_token_pair_by_address(&data) {
            Ok(refined_data) => match get_all_swap_related_txns_for_token(&refined_data.token0_address, &api_key) {
                Ok(_swap_txns) => match refined_get_all_swap_related_txns_for_token(&_swap_txns) {
                    _swap_results => {
                        println!("Successfully fetched and refined swap related transactions for token: {}", refined_data.token0_name);
                    }
                },
                Err(e) => println!("Error fetching swap related transactions: {}", e),
            },
            Err(e) => println!("Error refining token pair info: {}", e),
        },
        Err(e) => println!("{:?}", e),
    }

    println!("Hello, world!");
}
