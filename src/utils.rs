use colored::*;

pub fn display_reponse_status (response_status: &str) {
    let _prompt_text: String = format!("Response status: {}", response_status);
    if response_status == "200" {
        println!("{}", _prompt_text.green());
    } else {
        println!("{}", _prompt_text.red());
    }
}
