# Contracts

This directory contains the smart contracts for the project.

## Pre-requisites

```bash
rustup component add rust-src
cargo install --force --locked cargo-contract --version 3.2.0
```

## Polkadot Contracts

### Test Node

Start a testnode with the following command:

```bash
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.31.0 --force
substrate-contracts-node --base-path chain
```

### Build Contracts

```bash
cd flyconomy_attractions && cargo contract build --release
```

### Test the Contract

```bash
cd flyconomy_attractions && cargo test --release --features e2e-tests
```

### Deploy Contracts

Open the substrate UI at https://contracts-ui.substrate.io/?rpc=ws://127.0.0.1:9944 and deploy the contracts.

Open the Polkadot Apps UI at https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/accounts

## Ethereum Contracts

### Build Contracts

```bash
npm run build
```

### Test the Contract

```bash
npm run test
```

### Deploy Contracts

Deploy local:

```bash
npm run deploy
```

Deploy to Sepolia:
Copy the example.env file to .env and fill in the values.

```bash
INFURA_API_KEY=yourInfuraApiKey SEPOLIA_PRIVATE_KEY=yourSepoliaPrivateKey npx hardhat run scripts/deployFlyconomyAttractions.ts --network sepolia
```

### Mint Tokens

```bash
npm run mint -- 0xYourAddress "Taunus Mountains" "A mountain range in Hesse, Germany located north of Frankfurt" 507500 -82600
```

### show all tokens

```bash
npm run run -- --network sepolia
```

### update token

```bash
npm run update -- 0 "Taunus Mountains" "A mountain range in Hesse, Germany located north of Frankfurt" 502337, 84581
npm run update -- 1 "Biebrich Palace" "A Baroque residence in the borough of Biebrich in the city of Wiesbaden" 500375, 82341
npm run update -- 2 "Saalburg" "A reconstructed Roman fort located near Bad Homburg" 502713, 85667
npm run update -- 3 "Pferdskopf" "A basalt peak with an almost alpine flair in the Rhön Mountains" 504905, 99197
```

## Deployed Contracts

### Sepolia

FlyconomyAttractions deployed to [0x6338b648a9156827e3423A33cb2d32b09076906b](https://sepolia.etherscan.io/address/0x6338b648a9156827e3423A33cb2d32b09076906b#code)
