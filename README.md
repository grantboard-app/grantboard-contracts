# grantboard-contracts

Soroban smart contracts for GrantBoard — a decentralized grant management platform built on Stellar.

## Overview

This contract handles the core on-chain logic for GrantBoard:

- Posting grants with milestone-based funding
- Accepting contributor applications
- Selecting a grant recipient
- Releasing funds per milestone approval

## Contract Functions

| Function            | Description                                      |
| ------------------- | ------------------------------------------------ |
| `create_grant`      | Post a grant and lock funds into the contract    |
| `apply`             | Submit an application for a grant                |
| `select_applicant`  | Reviewer selects a contributor from applicants   |
| `approve_milestone` | Reviewer approves a milestone and releases funds |
| `get_grant`         | Read grant data                                  |

## Project Structure

contracts/
└── grantboard/
└── src/
├── lib.rs # Contract logic
└── test.rs # Unit tests

## Getting Started

### Prerequisites

- Rust
- Stellar CLI

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

### Install Stellar CLI

```bash
cargo install --locked stellar-cli
```

### Run Tests

```bash
cargo test
```

### Build Contract

```bash
stellar contract build
```

## Contributing

1. Fork the repo
2. Create a feature branch
3. Make your changes
4. Ensure all tests pass with `cargo test`
5. Open a pull request

## License

MIT
