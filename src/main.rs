use colored::*;
use anyhow::Result;
use std::io::{self, Write};

pub mod models;
pub mod utils;
pub mod api_calls;
pub mod refiners;
pub mod processors;
pub mod commands;

fn read_trimmed_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?; // ensure prompt appears before reading
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn choose_account_type_from_input(s: &str) -> Option<models::AccountType> {
    match s.to_lowercase().as_str() {
        "1" | "t" | "token" => Some(models::AccountType::Token),
        "2" | "a" | "account" => Some(models::AccountType::Account),
        _ => None,
    }
}

fn show_menu() {
    println!();
    println!("{}", "Please choose an option:".yellow());
    println!("  {}  - Scan a Token (token swaps)", "1".green());
    println!("  {}  - Scan an Account/Wallet (account swaps)", "2".green());
    println!("  {}  - Quit", "q".red());
}

fn run_cli() -> Result<()> {
    let welcome_text = format!("Welcome to the Bagscan Wallet Analyser CLI");
    println!("{}", welcome_text.green());
    println!("{}", "Interactive mode — choose an option to begin.".bright_black());

    loop {
        show_menu();

        let choice = read_trimmed_line("> ")?;
        if choice.to_lowercase() == "q" {
            println!("{}", "Goodbye!".cyan());
            break;
        }

        let account_type = match choose_account_type_from_input(&choice) {
            Some(t) => t,
            None => {
                println!("{}", "Invalid choice — please enter 1, 2, or q.".red());
                continue;
            }
        };

        // ask for address
        let prompt = match account_type {
            models::AccountType::Token => "Enter token address: ",
            models::AccountType::Account => "Enter wallet/account address: ",
        };
        let address = read_trimmed_line(prompt)?;
        if address.is_empty() {
            println!("{}", "Address cannot be empty. Try again.".red());
            continue;
        }

        // Minimal validation: optionally you can add more (length, prefix checks, etc.)
        println!("{}", format!("Scanning {} ...", address).bright_black());

        // ask for duration
        println!();
        println!("{}", "Select fetch duration:".yellow());
        println!("  {} - last 1 day", "1".green());
        println!("  {} - last 7 days", "7".green());
        println!("  {} - last 30 days", "30".green());
        println!("  {} - cancel", "c".red());

        let duration_input = read_trimmed_line("Duration (1/7/30): ")?;
        if duration_input.to_lowercase() == "c" {
            println!("{}", "Cancelled scan.".cyan());
            continue;
        }

        let fetch_duration: u32 = match duration_input.as_str() {
            "1" => 1,
            "7" => 7,
            "30" => 30,
            other => {
                // try to parse numeric fallback (optional)
                match other.parse::<u32>() {
                    Ok(n) if n > 0 => n,
                    _ => {
                        println!("{}", "Invalid duration — please enter 1, 7, 30 or c.".red());
                        continue;
                    }
                }
            }
        };

        println!(
            "{}",
            format!("Scanning {} for the last {} day(s)...", address, fetch_duration).bright_black()
        );

        // Call your existing command (propagate error if it fails)
        // NOTE: commands::scan_account signature must accept (address, account_type, fetch_duration)
        if let Err(e) = commands::scan_account(&address, account_type, fetch_duration) {
            println!("{} {}", "Scan failed:".red(), e);
        } else {
            println!("{}", "Scan finished.".green());
        }

        // after scan, allow user to continue or quit
        let again = read_trimmed_line("Scan again? (y/n) ")?;
        if again.to_lowercase().starts_with('n') {
            println!("{}", "Exiting interactive mode.".cyan());
            break;
        }
    } // end loop

    Ok(())
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    run_cli()?;
    Ok(())
}