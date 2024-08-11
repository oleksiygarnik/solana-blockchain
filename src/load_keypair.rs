use solana_sdk::signature::{Keypair, Signer};
use sha2::{Sha256, Digest};
use rayon::prelude::*;
use std::sync::Arc;

pub fn load_keypair() {
    let prefix = "a"; // Change to your prefix
    let prefix_bytes = prefix.as_bytes();
    let prefix_str = Arc::new(prefix_bytes.to_vec());

    println!("Starting key generation...");

    let result = (0..u64::MAX).into_par_iter().find_map_first(|_| {
        // Generate a new keypair
        let keypair = Keypair::new();
        let secret_key_bytes = keypair.to_bytes();

        // Hash the secret key
        let mut hasher = Sha256::new();
        hasher.update(&secret_key_bytes);
        let result = hasher.finalize();

        // Check if the hash starts with the prefix
        if result.starts_with(prefix_str.as_ref()) {
            Some((keypair, secret_key_bytes))
        } else {
            None
        }
    });

    match result {
        Some((keypair, secret_key_bytes)) => {
            println!("Keypair found!");
            println!("Public key: {}", keypair.pubkey());
            println!("Secret key: {:?}", secret_key_bytes);
        },
        None => println!("No matching keypair found."),
    }

    println!("✅ Finished!");
}