#  Payment Tracker – Stellar Soroban

A multi-wallet payment tracking smart contract built on the Stellar Soroban platform using Rust.  
This project demonstrates multi-address payment recording, contract deployment, and on-chain verification.

---

## 📌 Project Overview

The Payment Tracker contract allows multiple wallet addresses to record and store payment transactions securely on-chain.  
Each payment is authenticated and stored in Soroban contract storage.


---

## ✨ Features

- 🔐 Multi-wallet payment support  
- 🧾 On-chain payment recording  
- ⚡ Soroban smart contract in Rust  
- 🛰 Deployed on Stellar Testnet  
- 🔍 Verifiable via Stellar Explorer  
- 📦 WASM build included  



---

## 🛠 Tech Stack

- **Rust**
- **Soroban SDK**
- **Stellar Testnet**
- **Freighter Wallet**
- **Stellar Expert**



---

##  Project Structure
## 📁 Project Structure

```text
payment-tracker/
│
├── src/
│   └── lib.rs                  # Main smart contract
│
├── target/
│   └── wasm32v1-none/
│       └── release/
│           └── payment_tracker.wasm
│
├── screenshots/
│   ├── wallet-options.png
│   └── contract-explorer.png
│
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── LICENSE
└── README.md
```





Build the Contract
cargo build --target wasm32-unknown-unknown --release




⭐Compiled WASM will be generated at:
payment-tracker\target\wasm32v1-none\release\payment_tracker.wasm



⭐steps to deploy
soroban contract deploy \
  --wasm target/wasm32v1-none/release/payment_tracker.wasm \
  --source alice \
  --network testnet




⭐Deployed Contract Address 
Contract Address: CAJ7KQGX6W5YI273J6DXLITFUD23TZXS4WDRARX72AHJELGTHELXKKH2



⭐Wasm hash
50ecbf40e89bf0bce55f78e9e77d50440d45ea4020cb1a86a32b7b7b316c5719



⭐WASM Build

The compiled contract is included:
Users\HP\payment-tracker\target\wasm32v1-none\release\payment_tracker.wasm


⭐screenshots/
![Wallet](C:\Users\HP\payment-tracker\screenshots\contract-explorer.png.png)



# Future Improvements

-  Add payment status updates (pending / completed / failed)
-  Implement real-time event indexing and frontend sync
-  Support batch payments for multiple recipients
-  Add advanced payment filtering and history queries
-  Build full frontend dashboard for payment visualization
-  Add role-based access control for admin operations
-  Deploy to Stellar Mainnet


#Limitations:
Currently deployed only on Stellar Testnet
- Basic storage structure (not optimized for very large datasets)
- No frontend UI included (contract-only implementation)
- Limited error handling and validation
- No pagination for large payment history
- Event emission not fully implemented for all actions