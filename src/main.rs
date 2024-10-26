mod commands;
mod handlers;
mod solana_client;
mod table;
mod transactions;

use clap::Parser;
use commands::Cli;
use dotenv::dotenv;
use handlers::{handle_balance, handle_burn, handle_deploy, handle_mint, handle_transfer};
use solana_client::Client;
use solana_sdk::signer::keypair::Keypair;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Load the keypair from a file or environment variable
    let payer: Keypair = Keypair::from_base58_string(&std::env::var("SECRET").unwrap());
    // If you wish to perform mainnet transactions, change the URL accordingly
    let client: Client = solana_client::Client::new("https://api.devnet.solana.com", &payer);

    // Parse the command line arguments
    let cli: Cli = Cli::parse();

    // Handle the command
    match cli {
        Cli::Transfer(args) => handle_transfer(&client, args).await?,
        Cli::Mint(args) => handle_mint(&client, args).await?,
        Cli::Burn(args) => handle_burn(&client, args).await?,
        Cli::Deploy(_args) => handle_deploy(&client).await?,
        Cli::Balance(args) => handle_balance(&client, args).await?,
    }

    Ok(())
}
