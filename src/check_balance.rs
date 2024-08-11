use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use std::env;
use std::str::FromStr;

pub fn check_balance() {
    // Connection to devnet
    let url = "https://api.devnet.solana.com";
    let client = RpcClient::new(url.to_string());
    println!("⚡️ Connected to devnet");

    // Setup public key
    let public_key_str = env::var("PUBLIC_KEY").expect("PUBLIC_KEY must be set in .env");
    let public_key = Pubkey::from_str(&public_key_str).expect("Invalid public key format");

    // Get the current balance
    let balance_in_lamports = client.get_balance(&public_key).expect("Failed to get balance");
    
    // Convert in SOL
    let lamports_per_sol = 1_000_000_000;
    let balance_in_sol = balance_in_lamports as f64 / lamports_per_sol as f64;

    println!(
        "💰 The balance for the wallet at address {} is: {:.4} SOL",
        public_key, balance_in_sol
    );
}