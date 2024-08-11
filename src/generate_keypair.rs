use solana_sdk::signature::{Keypair, Signer};

pub fn generate_keypair() {
    // Generate a new keypair
    let keypair = Keypair::new();

    // Print the public key
    println!("The public key is: {}", keypair.pubkey());

    // Print the secret key (in byte format)
    println!("The secret key is: {:?}", keypair.to_bytes());

    println!("✅ Finished!");
}