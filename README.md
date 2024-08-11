# Solana Rust Application

This Rust application provides functionalities for interacting with the Solana blockchain. It allows users to generate keypairs, load existing keypairs, and check the balance of a wallet.

## Features

- **Generate Keypair**: Create a new Solana keypair and display its public and secret keys.
- **Load Keypair**: Load a keypair from a secret key and display its public key.
- **Check Balance**: Retrieve the balance of a Solana wallet using its public key.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70.0 or later)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (optional, for interacting with Solana directly)
- A `.env` file in the project root containing the `PUBLIC_KEY` for checking balance and optionally `SECRET_KEY` for loading a keypair.
