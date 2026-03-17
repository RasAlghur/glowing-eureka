// src/commands/mod.rs
use crate::processors;
use crate::models;
use anyhow::Result; // <- add this

pub fn scan_account(account_address: &str, account_type: models::AccountType, fetch_duration: u32) -> Result<()> {
    match account_type {
        models::AccountType::Token => println!("Account Type is Token"),
        models::AccountType::Account => println!("Account Type is Account"),
    }

    // forward the actual work to processors::process_account
    processors::process_account(account_address, account_type, fetch_duration)?;
    Ok(())
}