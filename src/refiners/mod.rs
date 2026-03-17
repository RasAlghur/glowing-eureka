// src/refiners/mod.rs
use crate::models;
use colored::*;

pub fn refined_get_token_pair_by_address(response: &models::TokenPairByAddress) -> Result<models::RefinedTokenPairInfo, String> {
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

    Ok(models::RefinedTokenPairInfo {
        exchange_address: response.pairs.first().unwrap().exchange_address.clone(),
        pair_address: response.pairs.first().unwrap().pair_address.clone(),
        token0_address: response.pairs.first().unwrap().pair.first().unwrap().token_address.clone(),
        token0_name: response.pairs.first().unwrap().pair.first().unwrap().token_name.clone(),
        token0_symbol: response.pairs.first().unwrap().pair.first().unwrap().token_symbol.clone(),
    })
}

pub fn refined_get_all_swap_related_txns(response: &models::AllSwapRelatedTxns, account_type: models::AccountType) -> Vec<models::SwapResult> {
    println!("{}", "Swap related transactions:".green());
    for swap in &response.result {
        println!("Transaction Hash: {}", swap.transaction_hash);
        match account_type {
            models::AccountType::Account => {},
            models::AccountType::Token => println!("Wallet Address: {}", swap.wallet_address),
        }
        println!("Block Timestamp: {}", swap.block_timestamp);
        println!("Pair Address: {}", swap.pair_address);
        println!("Exchange Address: {}", swap.exchange_address);
        println!("Token Bought: {} ({}) ", swap.bought.address, swap.bought.symbol);
        println!("Bought: {} {} (USD Price: {:.2} per {})", swap.bought.amount, swap.bought.symbol, swap.bought.usd_price, swap.bought.symbol);
        println!("Token Sold: {} ({}) ", swap.sold.address, swap.sold.symbol);
        println!("Sold: {} {} (USD Price: {:.2} per {})", swap.sold.amount, swap.sold.symbol, swap.sold.usd_price, swap.sold.symbol);
        println!("Total Value USD: {:.2}", swap.total_value_usd);
        println!("-----------------------------------");
    }
    response.result.clone()
}