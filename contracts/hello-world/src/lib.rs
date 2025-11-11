#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

// Persistent storage keys
#[contracttype]
#[derive(Clone)]
enum DataKey {
    Entrants,
    Winner,
}

#[contract]
pub struct RaffleContract;

#[contractimpl]
impl RaffleContract {
    pub fn enter(env: Env, entrant: Address) {
        entrant.require_auth();

        let mut entrants: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::Entrants)
            .unwrap_or_else(|| Vec::new(&env));

        entrants.push_back(entrant);
        env.storage().persistent().set(&DataKey::Entrants, &entrants);
    }

    pub fn draw_winner(env: Env) {
        let entrants: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::Entrants)
            .expect("No entrants in the raffle");

        if entrants.is_empty() {
            panic!("Cannot draw a winner, no one has entered");
        }

        let num_entrants_u32: u32 = entrants.len();
        let num_entrants_u64: u64 = num_entrants_u32 as u64;

        let winner_index_u64: u64 = env.prng().gen_range(0..num_entrants_u64);
        let winner_index: u32 = winner_index_u64 as u32;

        let winner = entrants.get(winner_index).unwrap();

        env.storage().persistent().set(&DataKey::Winner, &winner);
        env.storage().persistent().remove(&DataKey::Entrants);
    }

    pub fn get_winner(env: Env) -> Address {
        env.storage()
            .persistent()
            .get(&DataKey::Winner)
            .expect("No winner has been drawn yet")
    }
}
