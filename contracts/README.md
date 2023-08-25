# Contracts

This directory contains the smart contracts for the project.

## Pre-requisites

```bash
curl https://sh.rustup.rs -sSf | sh
rustup install nightly-2023-08-03
rustup component add rust-src
cargo install --force --locked cargo-contract --version 4.0.0-alpha
```

## Contracts

### PSP22

The PSP22 contract is a standard ERC20 contract with additional functionality to support the PSP22 standard.

### Test Node

Start a testnode with the following command:

```bash
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.30.0 --force
substrate-contracts-node --base-path chain
```

### Build Contracts

```bash
cd flyconomy_token && cargo contract build
cd flyconomy_bases && cargo contract build
```

### Deploy Contracts

Open the substrate UI at https://contracts-ui.substrate.io/?rpc=ws://127.0.0.1:9944 and deploy the contracts.
