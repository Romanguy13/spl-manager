use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use std::str::FromStr;

pub struct Client<'a> {
    pub client: RpcClient,
    pub payer: &'a Keypair,
}

impl<'a> Client<'a> {
    pub fn new(url: &str, payer: &'a Keypair) -> Self {
        let client: RpcClient = RpcClient::new(url.to_string());
        Client { client, payer }
    }

    pub async fn send_transaction(
        &self,
        transaction: Transaction,
        signers: Vec<&'a Keypair>,
    ) -> Result<Signature, Box<dyn std::error::Error>> {
        let recent_blockhash: Hash = self.client.get_latest_blockhash()?;
        let mut transaction: Transaction = transaction;
        transaction.sign(&signers, recent_blockhash);
        let signature: Signature = self
            .client
            .send_and_confirm_transaction_with_spinner(&transaction)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(signature)
    }

    pub async fn has_associated_token_account(
        &self,
        wallet_address: &str,
        mint_address: &str,
    ) -> bool {
        let wallet_pubkey: Pubkey = Pubkey::from_str(wallet_address).unwrap();
        let mint_pubkey: Pubkey = Pubkey::from_str(mint_address).unwrap();
        let associated_token_address: Pubkey =
            spl_associated_token_account::get_associated_token_address_with_program_id(
                &wallet_pubkey,
                &mint_pubkey,
                &spl_token::id(),
            );
        let account = self.client.get_account(&associated_token_address);
        account.is_ok()
    }

    pub async fn get_token_account_balance(
        &self,
        mint_address: &str,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let associated_token_address: Pubkey =
            spl_associated_token_account::get_associated_token_address_with_program_id(
                &self.payer.pubkey(),
                &Pubkey::from_str(mint_address)?,
                &spl_token::id(),
            );

        let account = self.client.get_token_account(&associated_token_address)?;
        match account {
            Some(account) => Ok(account.token_amount.ui_amount.unwrap_or(0.0) as u64),
            None => Ok(0),
        }
    }
}
