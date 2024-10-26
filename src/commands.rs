use clap::{Args, Parser};

#[derive(Parser)]
#[command(name = "Solana SPL Token CLI")]
#[command(version = "1.0")]
#[command(author = "Cody Lambert")]
#[command(about = "Manages Solana SPL Tokens")]
pub enum Cli {
    /// Transfer tokens
    Transfer(TransferArgs),
    /// Mint tokens
    Mint(MintArgs),
    /// Burn tokens
    Burn(BurnArgs),
    /// Deploy SPL token
    Deploy(DeployArgs),
    /// Request token balance
    Balance(TokenBalanceArgs),
}

#[derive(Args)]
pub struct TransferArgs {
    #[arg(help = "Mint address")]
    pub mint: String,
    #[arg(help = "Destination address")]
    pub destination: String,
    #[arg(help = "Amount to transfer")]
    pub amount: u64,
}

#[derive(Args)]
pub struct MintArgs {
    #[arg(help = "Mint address")]
    pub mint: String,
    #[arg(help = "Amount to mint")]
    pub amount: u64,
}

#[derive(Args)]
pub struct BurnArgs {
    #[arg(help = "Mint address")]
    pub mint: String,
    #[arg(help = "Amount to burn")]
    pub amount: u64,
}

#[derive(Args)]
pub struct DeployArgs {}

#[derive(Args)]
pub struct TokenBalanceArgs {
    #[arg(help = "Mint address")]
    pub mint: String,
}