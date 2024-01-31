mod model;
mod services;
mod jig;
mod gen;
mod menu;
mod firstlunch;

#[macro_use]

extern crate serde_derive;

extern crate reqwest;
extern crate serde_json;
extern crate winapi;

use std::io::{self};
use console::Term;
use crossterm::{style::{Color, Print, ResetColor, SetForegroundColor}, execute};
use std::sync::Arc;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    if firstlunch::is_first_launch() {
        firstlunch::create_files_with_content();
    }
   

    let mut stdout = io::stdout();
  
    let term = Term::stdout();

    let ascii_art = r#"
     ______ _                  _____ ____    _______          _ 
    |  ____| |                |_   _/ __ \  |__   __|        | |
    | |__  | |_   ___  ___   _  | || |  | |    | | ___   ___ | |
    |  __| | | | | \ \/ / | | | | || |  | |    | |/ _ \ / _ \| |
    | |    | | |_| |>  <| |_| |_| || |__| |    | | (_) | (_) | |
    |_|    |_|\__,_/_/\_\\__, |_____\____/     |_|\___/ \___/|_|
                          __/ |                                 
                         |___/                                  
                                                 
                                                 
    "#;

    execute!(stdout, SetForegroundColor(Color::Red), Print(ascii_art), ResetColor)?;

    let api_key = services::load_api_key().await?;

    match services::check_login(&api_key).await {  
        Ok((true, username)) => {

            let username = Arc::new(username);

            term.clear_screen().unwrap();
            menu::menu::display_menu_loop(username).await?;

           
          
        },
        _ => {
            execute!(stdout, SetForegroundColor(Color::Red), Print("Invalid key/Metadata"), ResetColor)?;
            std::process::exit(1);
        }
    }
    Ok(())
}