// src/table.rs
use colored::Colorize;
use tabled::settings::object::Rows;
use tabled::settings::{Alignment, Disable, Panel, Rotate, Style};
use tabled::Table;

pub struct TransferTransactionData<'a> {
    pub mint: &'a str,
    pub amount: u64,
    pub source: &'a str,
    pub destination: &'a str,
    pub signature: &'a str,
}

pub struct MintTransactionData<'a> {
    pub mint: &'a str,
    pub amount: u64,
    pub destination: &'a str,
    pub signature: &'a str,
}

pub struct BurnTransactionData<'a> {
    pub mint: &'a str,
    pub amount: u64,
    pub source: &'a str,
    pub signature: &'a str,
}

pub struct DeployTransactionData<'a> {
    pub mint: &'a str,
    pub authority: &'a str,
    pub signature: &'a str,
}

pub struct TokenBalanceData<'a> {
    pub mint: &'a str,
    pub balance: u64,
}

pub trait TransactionTableData {
    fn construct_table(&self) -> Table;
}

impl TransactionTableData for TransferTransactionData<'_> {
    fn construct_table(&self) -> Table {
        let mut amount_binding = self.amount.to_string();
        amount_binding.insert_str(0, "-");
        let method = "Transfer".bold().blue().to_string();
        let data: [[&str; 6]; 2] = [
            [
                "Signature",
                "Destination",
                "Source",
                "Mint",
                "Amount",
                "Method",
            ],
            [
                self.signature,
                self.destination,
                self.source,
                self.mint,
                amount_binding.as_str(),
                method.as_str(),
            ],
        ];
        let mut table: Table = Table::new(data);
        style_table(&mut table)
    }
}

impl TransactionTableData for MintTransactionData<'_> {
    fn construct_table(&self) -> Table {
        let mut amount_binding = self.amount.to_string();
        amount_binding.insert_str(0, "+");
        let method = "Mint".bold().green().to_string();
        let data: [[&str; 5]; 2] = [
            ["Signature", "Destination", "Mint", "Amount", "Method"],
            [
                self.signature,
                self.destination,
                self.mint,
                amount_binding.as_str(),
                method.as_str(),
            ],
        ];
        let mut table: Table = Table::new(data);
        style_table(&mut table)
    }
}

impl TransactionTableData for BurnTransactionData<'_> {
    fn construct_table(&self) -> Table {
        let mut amount_binding = self.amount.to_string();
        amount_binding.insert_str(0, "-");
        let method = "Burn".bold().red().to_string();
        let data: [[&str; 5]; 2] = [
            ["Signature", "Source", "Mint", "Amount", "Method"],
            [
                self.signature,
                self.source,
                self.mint,
                amount_binding.as_str(),
                method.as_str(),
            ],
        ];
        let mut table: Table = Table::new(data);
        style_table(&mut table)
    }
}

impl TransactionTableData for DeployTransactionData<'_> {
    fn construct_table(&self) -> Table {
        let method = "Deploy".bold().yellow().to_string();
        let data: [[&str; 4]; 2] = [
            ["Signature", "Authority", "Mint", "Method"],
            [self.signature, self.authority, self.mint, method.as_str()],
        ];
        let mut table: Table = Table::new(data);
        style_table(&mut table)
    }
}

impl TransactionTableData for TokenBalanceData<'_> {
    fn construct_table(&self) -> Table {
        let data: [[&str; 2]; 2] = [
            ["Mint", "Balance"],
            [self.mint, &self.balance.to_string()],
        ];
        let mut table: Table = Table::new(data);
        style_table(&mut table)
    }
}

fn style_table(table: &mut Table) -> Table {
    table
        .with(Style::modern())
        .with(Alignment::left())
        .with(Disable::row(Rows::first()))
        .with(Rotate::Left)
        .with(Panel::horizontal(
            0,
            "TRANSACTION SUMMARY".bold().to_string(),
        ));
    table.clone()
}