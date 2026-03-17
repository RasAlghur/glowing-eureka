// src/processors/mod.rs
use crate::api_calls;
use crate::refiners;
use crate::models;
use dotenv::dotenv;
use std::env;
use anyhow::{Result, anyhow};

pub fn process_account(account_address: &str, account_type: models::AccountType, fetch_duration: u32) -> Result<()> {
    dotenv().ok();

    let api_key = env::var("MORALIS_API_KEY")
        .map_err(|_| anyhow!("MORALIS_API_KEY must be set in .env or environment"))?;

    // PASS `account_type` by value (no `&`)
    // let swap_txns = api_calls::fetch_swap_related_txns(account_address, &api_key, account_type)
    let swap_txns = api_calls::fetch_all_swap_related_txns_pages(account_address, &api_key, account_type, fetch_duration)
        .map_err(|e| anyhow!("Fetching swap txns failed: {}", e))?;

    // PASS `account_type` by value (no `&`)
    let _swap_results = refiners::refined_get_all_swap_related_txns(&swap_txns, account_type);

    println!("Successfully fetched and refined swap related transactions for account: {}", account_address);
    Ok(())
}

pub fn process_tokenPair() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("MORALIS_API_KEY")
        .map_err(|_| anyhow!("MORALIS_API_KEY must be set in .env or environment"))?;

    let data = api_calls::get_token_pair_by_address(
        "AVF9F4C4j8b1Kh4BmNHqybDaHgnZpJ7W7yLvL7hUpump",
        &api_key,
    )
    .map_err(|e| anyhow!("API call failed: {}", e))?;

    let refined = refiners::refined_get_token_pair_by_address(&data)
        .map_err(|e| anyhow!("Refining token pair failed: {}", e))?;

    println!(
        "Refined pair: {} ({}) on exchange {}",
        refined.token0_name, refined.token0_address, refined.exchange_address
    );

    Ok(())
}