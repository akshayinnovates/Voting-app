# 🗳️ Voting App - Soroban Smart Contract

## 📌 Project Description

This project is a decentralized Voting Application built using Soroban smart contracts on the Stellar network. It allows users to create polls, vote on predefined options, and view voting results transparently on-chain.

The goal of this project is to demonstrate how blockchain can be used for trustless and tamper-proof voting systems.

---

## ⚙️ What It Does

- Enables creation of polls with multiple options
- Allows users to cast votes for any option
- Stores votes securely on the blockchain
- Provides real-time results of voting

All operations are executed on-chain using Soroban smart contracts, ensuring transparency and immutability.

---

## 🚀 Features

### 1. Create Poll
- Initialize a new poll using a unique `poll_id`
- Define multiple voting options

### 2. Vote
- Users can vote for a specific option in a poll
- Vote counts are updated securely on-chain

### 3. Get Results
- Retrieve vote counts for all options
- Results are stored and fetched directly from blockchain storage

### 4. Decentralized & Transparent
- No central authority controls the voting
- Data is immutable and verifiable

---

## 🛠️ Tech Stack

- **Language:** Rust
- **Framework:** Soroban SDK
- **Blockchain:** Stellar (Soroban Smart Contracts)

---

## ⚠️ Limitations (Basic Version)

- No voter identity verification
- No restriction on multiple voting
- No poll expiration mechanism

---

## 🔮 Future Improvements

- Add voter authentication (wallet-based identity)
- Prevent double voting
- Add poll deadlines
- UI frontend (React + Stellar SDK)
- Event logging for better tracking

---

## 📦 How to Deploy

1. Install Soroban CLI
2. Build contract:
   ```bash
   cargo build --target wasm32-unknown-unknown --release# Voting-app
  Contract address -  CCISDKM4PP3E4BPLQSZQDWT5KG7BT57BWCZGSQKNAEBU6KKODWMYJYNY
  
  Wallet address - GCF4X7HPBKYLE6IQ62OTY2XGLYF2RZRLPLCW5IPGFNLUGRXWTWLEOPEM
  
  URL -https://stellar.expert/explorer/testnet/contract/CCISDKM4PP3E4BPLQSZQDWT5KG7BT57BWCZGSQKNAEBU6KKODWMYJYNY

  <img width="1919" height="1079" alt="image" src="https://github.com/user-attachments/assets/1897fc4c-8af8-44d5-add0-59bf7101d408" />
