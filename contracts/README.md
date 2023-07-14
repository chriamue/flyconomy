# Contracts

This directory contains the smart contracts for the project.

## Contracts

### PSP22

The PSP22 contract is a standard ERC20 contract with additional functionality to support the PSP22 standard.

### Test Node

Start a testnode with the following command:

```bash
substrate-test-node
```

### Build Contracts

```bash
cargo contract build
```

### Deploy Contracts

Open the substrate UI at https://contracts-ui.substrate.io/?rpc=ws://127.0.0.1:9944 and deploy the contracts.

