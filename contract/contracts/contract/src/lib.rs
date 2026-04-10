#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Vec, Map};

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // Initialize a new poll with options
    pub fn create_poll(env: Env, poll_id: Symbol, options: Vec<Symbol>) {
        let mut votes: Map<Symbol, u32> = Map::new(&env);

        for option in options.iter() {
            votes.set(option, 0);
        }

        env.storage().instance().set(&poll_id, &votes);
    }

    // Vote for an option
    pub fn vote(env: Env, poll_id: Symbol, option: Symbol) {
        let mut votes: Map<Symbol, u32> = env
            .storage()
            .instance()
            .get(&poll_id)
            .expect("Poll does not exist");

        let count = votes.get(option.clone()).unwrap_or(0);
        votes.set(option, count + 1);

        env.storage().instance().set(&poll_id, &votes);
    }

    // Get results of a poll
    pub fn get_results(env: Env, poll_id: Symbol) -> Map<Symbol, u32> {
        env.storage()
            .instance()
            .get(&poll_id)
            .expect("Poll does not exist")
    }
}