# Pump.fun Smart Contract

![image](https://github.com/user-attachments/assets/24c9250c-d456-4967-85b5-c0e0cce697a3)

The Pump.fun Smart Contract is the Rust/Anchor smart contract for Pump.fun. The pump.fun smart contract facilitates various decentralized finance (DeFi) functionalities, including adding virtual Liquidity Provider (LP) tokens, removing LP tokens, and creating Raydium Pools.

## Functions

1. Add Virtual LP
   This function allows users to add virtual liquidity to the Pump.fun platform. This increases the liquidity of the pool without requiring actual token deposits. This function is crucial for simulating and testing liquidity scenarios.
2. Remove LP
   This function enables users to remove liquidity from the Pump.fun platform. This reduces the liquidity of the pool and allows users to withdraw their LP tokens.
3. Create Raydium Pool
   The create_raydium_pool function allows users to create a new pool on the Raydium platform. This pool will facilitate decentralized trading and liquidity provision.

## Getting Started

1. git clone https://github.com/lilmoat/solana-pump-fun-smart-contract
2. cd solana-pump-fun-smart-contract
3. cargo build
4. anchor deploy

## Contact
Telegram: [@lilm0at](https://t.me/lilm0at)
