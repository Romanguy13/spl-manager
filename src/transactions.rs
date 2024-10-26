use solana_sdk::{
    instruction::Instruction,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_token::state::Mint;
use std::str::FromStr;

pub trait TransactionInstructions {
    fn instructions(&self) -> Vec<Instruction>;
    fn signers(&self) -> Vec<&Keypair>;
}

pub struct CreateAssociatedTokenAccountTransaction<'a> {
    pub signer: &'a Keypair,
    pub address: String,
    pub mint: String,
}

impl<'a> TransactionInstructions for CreateAssociatedTokenAccountTransaction<'a> {
    fn instructions(&self) -> Vec<Instruction> {
        let mint_pubkey: Pubkey = Pubkey::from_str(&self.mint).unwrap();
        let address_pubkey: Pubkey = Pubkey::from_str(&self.address).unwrap();
        vec![
            spl_associated_token_account::instruction::create_associated_token_account(
                &self.signer.pubkey(),
                &address_pubkey,
                &mint_pubkey,
                &spl_token::id(),
            ),
        ]
    }

    fn signers(&self) -> Vec<&Keypair> {
        vec![&self.signer]
    }
}

pub struct TransferTransaction<'a> {
    pub signer: &'a Keypair,
    pub mint: String,
    pub destination: String,
    pub amount: u64,
}

impl<'a> TransactionInstructions for TransferTransaction<'a> {
    fn instructions(&self) -> Vec<Instruction> {
        let mint_pubkey: Pubkey = Pubkey::from_str(&self.mint).unwrap();
        let destination_pubkey: Pubkey = Pubkey::from_str(&self.destination).unwrap();

        let associated_token_addresses: Vec<Pubkey> =
            vec![&self.signer.pubkey(), &destination_pubkey]
                .into_iter()
                .map(|pubkey: &Pubkey| {
                    spl_associated_token_account::get_associated_token_address(pubkey, &mint_pubkey)
                })
                .collect();

        vec![spl_token::instruction::transfer_checked(
            &spl_token::id(),
            &associated_token_addresses[0],
            &mint_pubkey,
            &associated_token_addresses[1],
            &self.signer.pubkey(),
            &[],
            self.amount * 10u64.pow(9),
            9,
        )
        .unwrap()]
    }

    fn signers(&self) -> Vec<&Keypair> {
        vec![&self.signer]
    }
}

pub struct MintTransaction<'a> {
    pub signer: &'a Keypair,
    pub mint: String,
    pub amount: u64,
}

impl<'a> TransactionInstructions for MintTransaction<'a> {
    fn instructions(&self) -> Vec<Instruction> {
        let mint_pubkey: Pubkey = Pubkey::from_str(&self.mint).unwrap();
        let associated_token_account: Pubkey =
            spl_associated_token_account::get_associated_token_address(
                &self.signer.pubkey(),
                &mint_pubkey,
            );

        vec![spl_token::instruction::mint_to(
            &spl_token::id(),
            &mint_pubkey,
            &associated_token_account,
            &self.signer.pubkey(),
            &[],
            self.amount * 10u64.pow(9),
        )
        .unwrap()]
    }

    fn signers(&self) -> Vec<&Keypair> {
        vec![&self.signer]
    }
}

pub struct BurnTransaction<'a> {
    pub mint: String,
    pub signer: &'a Keypair,
    pub amount: u64,
}

impl<'a> TransactionInstructions for BurnTransaction<'a> {
    fn instructions(&self) -> Vec<Instruction> {
        let token_pubkey: Pubkey = Pubkey::from_str(&self.mint).unwrap();
        let associated_token_account: Pubkey =
            spl_associated_token_account::get_associated_token_address(
                &self.signer.pubkey(),
                &token_pubkey,
            );
        vec![spl_token::instruction::burn(
            &spl_token::id(),
            &associated_token_account,
            &token_pubkey,
            &self.signer.pubkey(),
            &[],
            self.amount * 10u64.pow(9),
        )
        .unwrap()]
    }

    fn signers(&self) -> Vec<&Keypair> {
        vec![&self.signer]
    }
}

pub struct DeployTransaction<'a> {
    pub signer: &'a Keypair,
    pub mint: &'a Keypair,
    pub rent: u64,
}

impl<'a> TransactionInstructions for DeployTransaction<'a> {
    fn instructions(&self) -> Vec<Instruction> {
        vec![
            system_instruction::create_account(
                &self.signer.pubkey(),
                &self.mint.pubkey(),
                self.rent,
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint2(
                &spl_token::id(),
                &self.mint.pubkey(),
                &self.signer.pubkey(),
                None,
                9,
            )
            .unwrap(),
            spl_associated_token_account::instruction::create_associated_token_account(
                &self.signer.pubkey(),
                &self.signer.pubkey(),
                &self.mint.pubkey(),
                &spl_token::id(),
            ),
        ]
    }

    fn signers(&self) -> Vec<&Keypair> {
        vec![&self.signer, &self.mint]
    }
}

pub fn create_transaction(transaction: &impl TransactionInstructions) -> Transaction {
    let instructions: Vec<Instruction> = transaction.instructions();
    Transaction::new_with_payer(&instructions, Some(&transaction.signers()[0].pubkey()))
}
