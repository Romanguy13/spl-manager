use solana_sdk::{
    program_pack::Pack,
    signature::{self, Signature, Signer},
    transaction::Transaction,
};
use spl_token::state::Mint;
use std::error::Error;
use tabled::Table;

use crate::{
    commands::{BurnArgs, MintArgs, TokenBalanceArgs, TransferArgs},
    solana_client::Client,
    table::{
        BurnTransactionData, DeployTransactionData, MintTransactionData, TokenBalanceData,
        TransactionTableData, TransferTransactionData,
    },
    transactions::{
        create_transaction, BurnTransaction, CreateAssociatedTokenAccountTransaction,
        DeployTransaction, MintTransaction, TransactionInstructions, TransferTransaction,
    },
};

pub async fn handle_transfer<'a>(
    client: &'a Client<'a>,
    args: TransferArgs,
) -> Result<(), Box<dyn Error>> {
    let has_associated_token_account: bool = client
        .has_associated_token_account(&args.destination, &args.mint)
        .await;

    if !has_associated_token_account {
        let create_account_sig =
            create_associated_token_account(client, &args.destination, &args.mint).await?;

        println!(
            "\x1b[32m✔\x1b[0m Associated token account created! Signature: {}",
            create_account_sig
        );
    }

    let transaction: TransferTransaction = TransferTransaction {
        signer: client.payer,
        destination: args.destination.clone(),
        amount: args.amount,
        mint: args.mint.clone(),
    };
    let tx: Transaction = create_transaction(&transaction);

    let signature: Signature = client.send_transaction(tx, transaction.signers()).await?;

    print!("\x1b[32m✔\x1b[0m Transfer finalized!\n");

    let table: Table = TransferTransactionData {
        mint: &args.mint,
        amount: args.amount,
        source: &client.payer.pubkey().to_string(),
        destination: &args.destination,
        signature: &signature.to_string(),
    }
    .construct_table();

    println!("{}", table);

    Ok(())
}

pub async fn handle_mint<'a>(client: &'a Client<'a>, args: MintArgs) -> Result<(), Box<dyn Error>> {
    let transaction: MintTransaction = MintTransaction {
        mint: args.mint.clone(),
        signer: client.payer,
        amount: args.amount,
    };
    let tx: Transaction = create_transaction(&transaction);

    let signature: Signature = client.send_transaction(tx, transaction.signers()).await?;

    print!("\x1b[32m✔\x1b[0m Mint finalized!\n");

    let table: Table = MintTransactionData {
        mint: &args.mint,
        amount: args.amount,
        destination: &client.payer.pubkey().to_string(),
        signature: &signature.to_string(),
    }
    .construct_table();

    println!("{}", table);

    Ok(())
}

pub async fn handle_burn<'a>(client: &'a Client<'a>, args: BurnArgs) -> Result<(), Box<dyn Error>> {
    let transaction: BurnTransaction = BurnTransaction {
        mint: args.mint.clone(),
        signer: client.payer,
        amount: args.amount,
    };
    let tx: Transaction = create_transaction(&transaction);

    let signature: Signature = client.send_transaction(tx, transaction.signers()).await?;

    print!("\x1b[32m✔\x1b[0m Burn finalized!\n");

    let table: Table = BurnTransactionData {
        mint: &args.mint,
        amount: args.amount,
        source: &client.payer.pubkey().to_string(),
        signature: &signature.to_string(),
    }
    .construct_table();

    println!("{}", table);

    Ok(())
}

pub async fn handle_deploy<'a>(client: &'a Client<'a>) -> Result<(), Box<dyn Error>> {
    let mint_kp: signature::Keypair = signature::Keypair::new();
    let rent = client
        .client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let deploy_transaction: DeployTransaction = DeployTransaction {
        signer: client.payer,
        mint: &mint_kp,
        rent,
    };
    let dep: Transaction = create_transaction(&deploy_transaction);

    let dep_signature: Signature = client
        .send_transaction(dep, deploy_transaction.signers())
        .await?;

    print!("\x1b[32m✔\x1b[0m Deployment finalized!\n");

    let table: Table = DeployTransactionData {
        mint: &mint_kp.pubkey().to_string(),
        authority: &client.payer.pubkey().to_string(),
        signature: &dep_signature.to_string(),
    }
    .construct_table();

    println!("{}", table);

    Ok(())
}

pub async fn handle_balance<'a>(
    client: &'a Client<'a>,
    args: TokenBalanceArgs,
) -> Result<(), Box<dyn Error>> {
    let balance = client.get_token_account_balance(&args.mint).await?;

    let table = TokenBalanceData {
        mint: &args.mint,
        balance,
    }
    .construct_table();

    println!("{}", table);

    Ok(())
}

async fn create_associated_token_account<'a>(
    client: &'a Client<'a>,
    address: &str,
    mint: &str,
) -> Result<Signature, Box<dyn Error>> {
    let transaction: CreateAssociatedTokenAccountTransaction =
        CreateAssociatedTokenAccountTransaction {
            signer: client.payer,
            address: address.to_string(),
            mint: mint.to_string(),
        };
    let tx: Transaction = create_transaction(&transaction);

    let signature: Signature = client.send_transaction(tx, transaction.signers()).await?;

    Ok(signature)
}
