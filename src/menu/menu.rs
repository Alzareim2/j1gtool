use std::io::{self};
use std::sync::Arc;
use console::Term;
use crossterm::{style::{Color, Print, ResetColor, SetForegroundColor}, execute};
use crate::model;
use crate::jig;

pub async fn display_menu_loop(username: Arc<String>) -> Result<(), Box<dyn std::error::Error>> {
   
    loop {
        
        let mut stdout = io::stdout();
        let stdin = io::stdin();
        let term = Term::stdout();
        term.clear_screen().unwrap();

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

        execute!(stdout, SetForegroundColor(Color::DarkRed), Print(ascii_art), ResetColor)?;

        let username_box: Box<dyn std::fmt::Display> = Box::new(username.clone());


        println!("\nSuccessfully logged in as: {}", &*username_box);
        

        model::display_menu();
        execute!(stdout, SetForegroundColor(Color::Red), Print("\n\nChoose an option / Sélectionner une option :\n"), ResetColor)?;

        let mut choice = String::new();
        stdin.read_line(&mut choice)?;
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                execute!(stdout, SetForegroundColor(Color::Red), Print("\n\nInvalid choice. Please enter a valid number.\n"), ResetColor)?;
                continue;
            }
        };

        match choice {
            
            
            
            1 => {
                term.clear_screen().unwrap();
                execute!(stdout, SetForegroundColor(Color::Yellow), Print("\n\nJIG\n"), ResetColor)?;
                let mut address = String::new();
                let mut mode = String::new();
                let mut filename = String::new();
                let mut num_times = String::new();

                execute!(stdout, SetForegroundColor(Color::White), Print("Enter your address: "), ResetColor)?;
                std::io::stdin().read_line(&mut address).unwrap();

                execute!(stdout, SetForegroundColor(Color::White), Print("Enter jig mode (soft, medium, hard, special): "), ResetColor)?;
                std::io::stdin().read_line(&mut mode).unwrap();

                execute!(stdout, SetForegroundColor(Color::White), Print("Enter filename: "), ResetColor)?;
                std::io::stdin().read_line(&mut filename).unwrap();

                execute!(stdout, SetForegroundColor(Color::White), Print("Enter the number of times to generate the address: "), ResetColor)?;
                std::io::stdin().read_line(&mut num_times).unwrap();
                let num_times: usize = num_times.trim().parse().unwrap();

                match jig::jig(&address.trim(), &mode.trim(), &filename.trim(), num_times) {
                    Ok(_) => execute!(stdout, SetForegroundColor(Color::Green), Print("File successfully created\n"), ResetColor)?,
                    Err(_) => execute!(stdout, SetForegroundColor(Color::Red), Print("Error while creating the file\n"), ResetColor)?,
                }

                execute!(stdout, SetForegroundColor(Color::Green), Print("Press Enter to Continue"), ResetColor)?;
                let _ = stdin.read_line(&mut String::new());
                term.clear_screen().unwrap();
            },
            
            2 => {
                term.clear_screen().unwrap();
                execute!(stdout, SetForegroundColor(Color::Yellow), Print("Select an option..."), ResetColor)?;
            
                println!("\n1 => Number");
                println!("2 => Name");
                println!("3 => Surname");
            
                let mut selection = String::new();
                stdin.read_line(&mut selection)?;
                let selection: i32 = match selection.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        execute!(stdout, SetForegroundColor(Color::Red), Print("\n\nInvalid choice. Please enter a valid number.\n"), ResetColor)?;
                        continue;
                    }
                };
                term.clear_screen().unwrap();
            
                match selection {
                    1 => {
                        
                        execute!(stdout, SetForegroundColor(Color::Yellow), Print("You selected Number\n"), ResetColor)?;
                        let country_code = crate::gen::read_input("Please enter country code:");
                        let num_of_times: u32 = crate::gen::read_input("Please enter the number of phone numbers to generate:").parse().expect("Please enter a number!");
                        let filename = format!("{}.txt", crate::gen::read_input("Please enter filename:").trim_end_matches(".txt"));

                        if let Some(contents) = crate::gen::generate_phone_numbers(&country_code, num_of_times) {
                            crate::gen::write_to_file(filename, contents);
                        }
                    },
                    2 => {
                        execute!(stdout, SetForegroundColor(Color::Yellow), Print("You selected Name\n"), ResetColor)?;
                        let num_of_times: usize = crate::gen::read_input("Please enter the number of Names to generate:")
                                                    .parse()
                                                    .expect("Please enter a number!");
                        let filename = format!("{}.txt", crate::gen::read_input("Please enter filename:")
                                            .trim_end_matches(".txt"));
            
                                            crate::gen::generate_names(num_of_times, filename);
                    },           
                    3 => {
                        execute!(stdout, SetForegroundColor(Color::Yellow), Print("You selected Surames\n"), ResetColor)?;
                        let num_of_times: usize = crate::gen::read_input("Please enter the number of Surnames to generate:")
                                                    .parse()
                                                    .expect("Please enter a number!");
                        let filename = format!("{}.txt", crate::gen::read_input("Please enter filename:")
                                            .trim_end_matches(".txt"));
            
                                            crate::gen::generate_surnames(num_of_times, filename);
                    },
                    _ => {
                        execute!(stdout, SetForegroundColor(Color::Red), Print("\n\nInvalid choice. Please enter a valid number.\n"), ResetColor)?;
                        continue;
                    }
                }
            
                execute!(stdout, SetForegroundColor(Color::Green), Print("Press Enter to Continue"), ResetColor)?;
                let _ = stdin.read_line(&mut String::new());
                term.clear_screen().unwrap();
            },
            
            4 => {
                execute!(stdout, SetForegroundColor(Color::Yellow), Print("Exiting the program..."), ResetColor)?;
                std::process::exit(0);
            },
            _ => {
                execute!(stdout, SetForegroundColor(Color::Red), Print("Invalid choice. Please enter a valid number."), ResetColor)?;
                term.clear_screen().unwrap();
                continue;
            }
        }

        
    }
}

