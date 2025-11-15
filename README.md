# 🎯 Soroban Random Prize (Raffle Contract)

This project implements a simple **raffle smart contract** written in **Rust** using the [Soroban SDK](https://soroban.stellar.org/).

CMD

stellar contract invoke --id <CONTRACT_ID> --network local-testnet --source-account <USER_PUBLIC_ID> -- enter --entrant <USER_PUBLIC_ID>

stellar contract invoke --id <CONTRACT_ID> --network local-testnet --source-account <USER_PUBLIC_ID> -- draw_winner

stellar contract invoke --id <CONTRACT_ID> --network local-testnet --source-account <USER_PUBLIC_ID> -- get_winner

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.

