// src/mains.rs

use dotenv::dotenv;
use std::env;

pub mod models;
pub mod utils;
pub mod api_calls;
pub mod processors;

fn main() {
    dotenv().ok();

    let api_key: String = env::var("MORALIS_API_KEY")
        .expect("MORALIS_API_KEY must be set in .env or environment");

    let get_token_pair_by_address_result = api_calls::get_token_pair_by_address("AVF9F4C4j8b1Kh4BmNHqybDaHgnZpJ7W7yLvL7hUpump", &api_key);
    match get_token_pair_by_address_result {
        Ok(data) => match processors::refined_get_token_pair_by_address(&data) {
            Ok(refined_data) => match api_calls::fetch_swap_related_txns(&refined_data.token0_address, &api_key, models::AccountType::Token) {
            // Ok(refined_data) => match api_calls::fetch_swap_related_txns("2U9W9kBvnjisfch6Dp8T4sXG8v2ppuyEp2hVSbm3gxhb", &api_key, models::AccountType::Account) {
                Ok(_swap_txns) => match processors::refined_get_all_swap_related_txns(&_swap_txns) {
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
