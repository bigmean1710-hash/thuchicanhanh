# StellarVault — Sổ Thu Chi Cá Nhân On-Chain

## Problem

Quản lý thu chi cá nhân thường nằm rải rác trên app hoặc sổ tay, dễ mất dữ liệu và khó kiểm chứng khi cần đối chiếu lịch sử thu nhập và chi tiêu.

## Solution

Xây dựng sổ thu chi cá nhân trên Soroban smart contract — ghi **thu**, **chi**, ngân sách và báo cáo tổng hợp on-chain, kèm web dashboard kết nối Freighter wallet.

## Why Stellar

Sử dụng **Soroban smart contract** trên Stellar để lưu sổ thu chi bất biến, xác thực giao dịch qua ví Freighter và triển khai chi phí thấp trên **Stellar Testnet**.

## Target User

Cá nhân muốn theo dõi thu nhập, chi tiêu và ngân sách hàng tháng; sinh viên / developer học Soroban và tích hợp blockchain vào ứng dụng tài chính.

## Live Demo

- Network: Stellar Testnet
- **Contract ID**: CC44E3WEJA6XARYY37UV5GPAXZLONW7OG6VNMAMYQ43JKKRAZLV4V7RK
- **Transaction**: [https://stellar.expert/explorer/testnet *(dán link tx sau khi invoke)*](https://stellar.expert/explorer/testnet/contract/CC44E3WEJA6XARYY37UV5GPAXZLONW7OG6VNMAMYQ43JKKRAZLV4V7RK)

## How to Run

1. Clone: `git clone <your-repo-url> && cd stellar-contract`
2. Build: `cd contracts/hello-world && make build`
3. Test: `cargo test --package hello-world`
4. Deploy: `stellar contract deploy --wasm target/wasm32-unknown-unknown/release/hello_world.wasm --source-account <YOUR_ADDRESS> --network testnet`
5. Frontend: mở thư mục `finance/` → chạy Live Server hoặc `npx serve finance` → kết nối Freighter (Testnet) → dán Contract ID

## Tech Stack

- Smart Contract: Rust / Soroban SDK 22.0
- Frontend: HTML / CSS / JavaScript / `@stellar/stellar-sdk` / `@stellar/freighter-api`
- Wallet: Freighter (Stellar Testnet)
- CLI: Stellar CLI (`stellar contract build`, `stellar contract invoke`)

---

## Contract Functions (Reference)

| Function | Description |
|----------|-------------|
| `initialize(user)` | Mở sổ thu chi (1 lần / ví) |
| `thu(user, category, amount)` | Ghi thu nhập |
| `chi(user, category, amount)` | Ghi chi tiêu |
| `chuyen_muc(user, from, to, amount)` | Chuyển giữa danh mục |
| `summary(user)` | Báo cáo tổng thu / tổng chi / còn lại |
| `dat_ngan_sach(user, category, limit)` | Đặt ngân sách tháng |
| `deposit` / `withdraw` | Alias tiếng Anh của `thu` / `chi` |

**Lưu ý:** `category` là Soroban `Symbol` (tối đa 9 ký tự), ví dụ: `salary`, `food`, `save`.
