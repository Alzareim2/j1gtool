use serde_json;
use crossterm::{style::{Color, Print, ResetColor, SetForegroundColor}, execute};
use std::io::stdout;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub product: String,
    pub user: String,
    pub plan: String,
    pub email: String,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub status: String,
    pub valid: bool,
    pub cancel_at_period_end: bool,
    pub payment_processor: String,
    pub license_key: String,
    pub metadata: serde_json::Value,
    pub quantity: i32,
    pub wallet_address: Option<String>,
    pub custom_fields_responses: serde_json::Value,
    pub custom_fields_responses_v2: Vec<serde_json::Value>,
    pub discord: DiscordUser,
    pub nft_tokens: Vec<serde_json::Value>,
    pub expires_at: Option<String>,
    pub renewal_period_start: Option<i64>,
    pub renewal_period_end: Option<i64>,
    pub created_at: i64,
    pub manage_url: String,
    pub affiliate_page_url: String,
    pub checkout_session: Option<String>,
    pub access_pass: String,
    pub deliveries: serde_json::Value,
    pub telegram_account_id: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordUser {
    
    pub id: String,
    pub username: String,
    pub image_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserActivity {
    pub Username: String,
    pub Status: String,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub api_key: String,
}

pub fn display_menu() {
    
    let header3 = format!("{:<25}", "Tool");

    

    let column3_option1 = format!("{:<25}", "[1] J1G");
    let column3_option2 = format!("{:<25}", "[2] Generator");
    let column3_option4 = format!("{:<25}", "[4] EXIT");
   

    execute!(
        stdout(),
        SetForegroundColor(Color::DarkRed),
        Print("\n\n"),
        
        Print(&header3),
        ResetColor
    ).unwrap();

    
    println!("\n{}",  column3_option1);
    println!("{}",  column3_option2);
    println!("{}",  column3_option4);
}