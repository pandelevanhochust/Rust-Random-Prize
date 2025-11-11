# 🎯 Soroban Random Prize (Raffle Contract)

This project implements a simple **raffle smart contract** written in **Rust** using the [Soroban SDK](https://soroban.stellar.org/).

It allows users to:
- Enter the raffle with their Stellar address
- Randomly draw a winner using Soroban’s pseudo-random generator (`env.prng()`)
- Retrieve the last drawn winner from contract storage

> ⚠️ Note: This contract uses **insecure randomness** and should only be used for demo/testnet purposes — not for real mainnet prizes!

---

## 🚀 Features
- `enter(env, entrant: Address)` → Add a participant to the raffle
- `draw_winner(env)` → Select a random winner from the entrants
- `get_winner(env)` → Get the most recent winner

---

## 🧱 Project Structure
