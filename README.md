# SPL Manager

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)


## Overview

SPL Manager is a Rust-based command-line tool for managing Solana Program Library (SPL) tokens. It provides functionalities for transferring, minting, burning, and deploying on the Solana blockchain.

## Goal

The goal of this project was to help me learn more about the Solana blockchain and the Rust programming language. I took heavy inspiration from the `spl-token-cli` project provided by the Solana team.

***Note***: This project is for development and educational purposes only. It is not recommended for use in production environments.

## Features

- **Transfer Tokens**: Transfer SPL tokens between accounts.
- **Mint Tokens**: Mint new SPL tokens.
- **Burn Tokens**: Burn existing SPL tokens.
- **Deploy Programs**: Deploy new programs to the Solana blockchain.
- **Airdrop Tokens**: Airdrop SPL tokens to multiple accounts.

## Installation

To install SPL Manager, you need to have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).

```sh
# Clone the repository
git clone https://github.com/yourusername/spl_manager.git

# Navigate to the project directory
cd spl_manager

# Build the project
cargo install --path .

# Run the project
spl <commands>
```

# Usage
Before using SPL Manager, ensure you have a valid Solana keypair, with a devnet SOL balance, and the necessary environment variables set up.

## Environment Variables
  
  ```sh
  SECRET=<your_secret_key>
  ```

## Commands

SPL Manager provides several commands for managing SPL tokens. Here are some examples:

```sh
# Transfer Tokens
spl transfer <mint-address> <destination-address> <amount>

# Mint Tokens
spl mint <mint-address> <amount>

# Burn Tokens
spl burn <mint-address> <amount>

# Deploy Programs
spl deploy

# Request Token Balance
spl balance <mint-address>

# Get help
spl -h


# Example Output
$ spl mint Bckf7zDmjesKPWQ6qcmkFFR1k6izG5WY6jnanUKFWyUz 20
✔ Mint finalized!
┌─────────────┬──────────────────────────────────────────────────────────────────────────────────────────┐
│ TRANSACTION SUMMARY                                                                                    │
├─────────────┼──────────────────────────────────────────────────────────────────────────────────────────┤
│ Method      │ Mint                                                                                     │
├─────────────┼──────────────────────────────────────────────────────────────────────────────────────────┤
│ Amount      │ +20                                                                                      │
├─────────────┼──────────────────────────────────────────────────────────────────────────────────────────┤
│ Mint        │ Bckf7zDmjesKPWQ6qcmkFFR1k6izG5WY6jnanUKFWyUz                                             │
├─────────────┼──────────────────────────────────────────────────────────────────────────────────────────┤
│ Destination │ CtLquYWjkGYFN2U4tWdwVJjLMPQhALbjrUw8egbGvBT7                                             │
├─────────────┼──────────────────────────────────────────────────────────────────────────────────────────┤
│ Signature   │ 3usk6GcwQ9qqJqZ1YQG6LfSe5ZTGYLaFnhmcP4uCbENoPhNgY3ds98HqLXHRa1vH7y5PLMRYUXBPLJ9zbUTWd4Ye │
└─────────────┴──────────────────────────────────────────────────────────────────────────────────────────┘
```



## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## Aknowledgements

- [Solana](https://solana.com/)
- [Rust Programming Language](https://www.rust-lang.org/)