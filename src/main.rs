mod generate_keypair;
mod load_keypair;
mod check_balance;

use std::io::{self, Write};

fn main() {
    dotenv::dotenv().ok();

    loop {
        println!("Select an option:");
        println!("1. Generate Keypair");
        println!("2. Load Keypair");
        println!("3. Check Balance");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => { generate_keypair::generate_keypair(); },
            "2" => { load_keypair::load_keypair(); },
            "3" => { check_balance::check_balance(); },
            "4" => { 
                println!("Exiting...");
                break; 
            },
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}