# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello-world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       ├── Cargo.toml
│       └── Makefile
├── Cargo.toml
├── Makefile
└── README.md
```

- Contract logic is in `contracts/hello-world/src/lib.rs`.
- Unit tests are in `contracts/hello-world/src/test.rs`.
- The workspace root `Cargo.toml` manages dependencies for the contract.

## Contract Functions

| Function | Description |
|----------|-------------|
| `initialize(user)` | Initialize finance ledger for a user |
| `deposit(user, category, amount)` | Record income / deposit |
| `withdraw(user, category, amount)` | Record expense / withdrawal |
| `transfer_category(user, from, to, amount)` | Transfer between categories |
| `balance(user, category)` | Get category balance |
| `total_balance(user)` | Get total balance |
| `summary(user)` | Get income, expense, net balance |
| `hello(to)` | Sample Soroban hello function |
| `set_budget(user, category, limit)` | Set monthly budget limit |
| `budget_remaining(user, category)` | Get remaining budget |

## Build & Test

```bash
# Run all tests
make test

# Build WASM
make build
```

Or from the contract folder:

```bash
cd contracts/hello-world
make test
make build
```

## Deploy (Testnet)

1. Connect Freighter wallet and switch to **Testnet**.
2. Fund the account with testnet XLM (Friendbot).
3. Deploy:

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source-account <YOUR_ADDRESS> \
  --network testnet
```

4. Invoke examples:

```bash
# Initialize
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <YOUR_ADDRESS> \
  --network testnet \
  -- initialize --user <YOUR_ADDRESS>

# Deposit
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <YOUR_ADDRESS> \
  --network testnet \
  -- deposit --user <YOUR_ADDRESS> --category salary --amount 5000000

# Hello
stellar contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- hello --to StellarVault
```

## Notes

- `category` is a Soroban `Symbol` (max 9 characters), e.g. `salary`, `food`, `save`.
- Amounts use `i128` (define your own unit, e.g. VND × 1).
- Run `initialize` once per user before other write functions.
