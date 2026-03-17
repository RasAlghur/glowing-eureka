// src/api_calls/mod.rs
use crate::models;
use crate::utils;
use reqwest::blocking::Client;
use colored::*;
use std::thread::sleep;
use std::time::Duration as StdDuration;
use chrono::{DateTime, Utc, Duration as ChronoDuration};



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

/// Fetch swap transactions but stop once we reach transactions older than `days` days.
/// - `days`: how many days back to fetch (1, 7, 30, etc.)
/// - returns an `AllSwapRelatedTxns` with `result` containing only txns within the requested window.
pub fn fetch_all_swap_related_txns_pages(
    addr: &str,
    api_key: &str,
    account_type: models::AccountType,
    days: u32,
) -> Result<models::AllSwapRelatedTxns, reqwest::Error> {
    let client = Client::new();
    let mut all_results: Vec<models::SwapResult> = Vec::new();
    let mut cursor: Option<String> = None;

    // Compute cutoff: transactions older than this (strictly less than) will NOT be included.
    let cutoff: DateTime<Utc> = Utc::now() - ChronoDuration::days(days as i64);

    const MAX_PAGES: usize = 1000;
    let mut page_count: usize = 0;
    let mut previous_cursor: Option<String> = None;
    let mut reached_cutoff = false;

    loop {
        page_count += 1;
        if page_count > MAX_PAGES {
            eprintln!("Reached MAX_PAGES ({}) — stopping pagination early.", MAX_PAGES);
            break;
        }

        let mut request_url = format!(
            "https://solana-gateway.moralis.io/{}/mainnet/{}/swaps?limit=100&order=DESC&transactionTypes=buy%2Csell",
            account_type, addr
        );

        if let Some(ref c) = cursor {
            request_url.push_str(&format!("&cursor={}", c));
        }

        println!("{}", format!("fetch page {} -> {}", page_count, request_url).bright_black());

        let response = client
            .get(&request_url)
            .header("X-API-Key", api_key)
            .header("accept", "application/json")
            .send()?;

        utils::display_reponse_status(response.status().as_str());

        // parse page into typed model
        let page: models::AllSwapRelatedTxns = response.json::<models::AllSwapRelatedTxns>()?;

        // Defensive: if page.result is empty, stop
        if page.result.is_empty() {
            println!("Page {} returned zero results — stopping.", page_count);
            break;
        }

        // Iterate results (they should be sorted newest -> oldest)
        for r in page.result.iter() {
            // Try parse the block timestamp. If parsing fails, include the tx (fail-open).
            let parsed_ts = DateTime::parse_from_rfc3339(&r.block_timestamp)
                .map(|dt_fixed| dt_fixed.with_timezone(&Utc));

            match parsed_ts {
                Ok(ts) => {
                    if ts < cutoff {
                        // we've hit older-than-cutoff txns — stop fetching any more.
                        reached_cutoff = true;
                        break;
                    } else {
                        all_results.push(r.clone());
                    }
                }
                Err(_) => {
                    // If parse fails, include the txn (don't stop). You can change to skip if you prefer.
                    all_results.push(r.clone());
                }
            }

            // safety check for MAX_RESULTS if you want (not strictly required)
            if all_results.len() >= 50_000 {
                eprintln!("Reached MAX_RESULTS (50_000) — stopping early.");
                reached_cutoff = true;
                break;
            }
        }

        if reached_cutoff {
            println!("Reached cutoff ({} days) on page {} - stopping.", days, page_count);
            break;
        }

        // update cursor and detect repeats (defensive)
        let next_cursor = match page.cursor {
            Some(ref c) if !c.is_empty() => Some(c.clone()),
            _ => None,
        };

        if next_cursor.is_some() && previous_cursor.is_some() && next_cursor == previous_cursor {
            eprintln!("Cursor did not advance (repeated) — stopping to avoid loop.");
            break;
        }

        previous_cursor = cursor;
        cursor = next_cursor;

        if cursor.is_none() {
            println!("No next cursor — finished pagination after {} pages.", page_count);
            break;
        }

        // polite sleep (reduce 429s)
        sleep(StdDuration::from_millis(300));
    }

    // return aggregated typed struct compatible with existing refiners
    let aggregated = models::AllSwapRelatedTxns {
        cursor: None,
        page_size: all_results.len() as u32,
        page: 1,
        result: all_results,
    };

    Ok(aggregated)
}